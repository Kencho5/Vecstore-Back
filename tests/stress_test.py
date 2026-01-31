import asyncio
import aiohttp
import time
import random
import argparse
import base64
import uuid
import re

# Categories for product variety
PRODUCT_KEYWORDS = [
    "smartwatch", "laptop", "smartphone", "headphones", "camera", 
    "drone", "sneakers", "backpack", "coffee maker", "blender",
    "vacuum cleaner", "monitor", "keyboard", "mouse", "speaker",
    "tablet", "gaming console", "watch", "sunglasses", "jacket"
]

async def get_image_data(session, url):
    try:
        headers = {"User-Agent": "Mozilla/5.0"}
        async with session.get(url, headers=headers, timeout=10) as response:
            if response.status != 200:
                return None
            return await response.read()
    except Exception:
        return None

async def fetch_ebay_pool(session):
    """Scrapes a few pages of eBay to get real product images and titles."""
    pool = []
    print("Seeding product pool from eBay...")
    test_keywords = random.sample(PRODUCT_KEYWORDS, 8) + ["smartwash"]
    
    headers = {
        "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
        "Accept-Language": "en-US,en;q=0.9",
        "Referer": "https://www.google.com/"
    }

    for kw in test_keywords:
        url = f"https://www.ebay.com/sch/i.html?_nkw={kw.replace(' ', '+')}"
        try:
            # Small delay to be less aggressive
            await asyncio.sleep(random.uniform(0.5, 1.5))
            async with session.get(url, headers=headers, timeout=15) as resp:
                if resp.status != 200:
                    print(f"Error scraping eBay for {kw}: Status {resp.status}")
                    continue
                
                html = await resp.text()
                # Find eBay image URLs and Alt text (titles)
                # eBay sometimes uses data-src for lazy loading
                matches = re.findall(r'<img[^>]+(?:src|data-src)="(https://i\.ebayimg\.com/images/g/[^"]+)"[^>]*alt="([^"]+)"', html)
                
                count = 0
                for img_url, title in matches:
                    if "s-l" in img_url and len(title) > 10:
                        pool.append({"url": img_url, "title": title, "category": kw})
                        count += 1
                
                if count == 0:
                    # Fallback regex for different eBay layouts
                    matches_alt = re.findall(r'{"image":"(https://i\.ebayimg\.com/images/g/[^"]+)","title":"([^"]+)"}', html)
                    for img_url, title in matches_alt:
                        pool.append({"url": img_url, "title": title, "category": kw})
                        count += 1

        except Exception as e:
            print(f"Exception scraping eBay for {kw}: {type(e).__name__} - {e}")
    
    print(f"Collected {len(pool)} real products for the stress test.")
    return pool

def get_fallback_url(category):
    """Fallback to LoremFlickr product images if pool is empty or for extra variety."""
    seed = random.randint(1, 1000000)
    return f"https://loremflickr.com/800/800/{category}?lock={seed}"

async def insert_image(session, sem, api_key, database, image_id, total_images, product_pool):
    async with sem:
        # 70% chance to use a real product from the pool, 30% random fallback
        if product_pool and random.random() < 0.7:
            product = random.choice(product_pool)
            image_url = product["url"]
            title = product["title"]
            category = product["category"]
        else:
            category = random.choice(PRODUCT_KEYWORDS)
            image_url = get_fallback_url(category)
            title = f"Product {category} - {uuid.uuid4().hex[:6]}"

        img_data = await get_image_data(session, image_url)
        if img_data is None:
            # Final fallback to Picsum if all else fails
            image_url = f"https://picsum.photos/seed/{random.randint(1,1000)}/800/800"
            img_data = await get_image_data(session, image_url)
            if img_data is None:
                return

        base64_image = base64.b64encode(img_data).decode('utf-8')
        
        payload = {
            'image': base64_image,
            'database': database,
            'metadata': {
                "name": title,
                "category": category,
                "product_id": f"PROD-{uuid.uuid4().hex[:8].upper()}",
                "source": "ebay_stress_test" if "ebayimg" in image_url else "lorem_flickr",
                "original_url": image_url
            }
        }

        headers = {
            "Authorization": api_key,
            "Content-Type": "application/json"
        }
        
        vecstore_url = "https://api.vecstore.app/insert-image"
        
        try:
            async with session.post(vecstore_url, headers=headers, json=payload) as res:
                if res.status == 200:
                    print(f"[{image_id}/{total_images}] Inserted: {title[:50]}...")
                else:
                    response_text = await res.text()
                    print(f"[{image_id}/{total_images}] ERROR: {res.status} | {response_text}")
        except Exception as e:
            print(f"[{image_id}/{total_images}] REQUEST FAILED: {e}")

async def main():
    parser = argparse.ArgumentParser(description="Async product stress test for Vecstore API.")
    parser.add_argument("--api_key", required=True, help="Your Vecstore API Key.")
    parser.add_argument("--database", default="vecstore", help="The database name to insert into.")
    parser.add_argument("--concurrency", type=int, default=30, help="Number of concurrent requests.")
    parser.add_argument("--total_images", type=int, default=1000, help="Total number of images to insert.")
    args = parser.parse_args()

    start_time = time.time()
    sem = asyncio.Semaphore(args.concurrency)

    connector = aiohttp.TCPConnector(ssl=False)
    async with aiohttp.ClientSession(connector=connector) as session:
        # Pre-fetch a pool of real products
        product_pool = await fetch_ebay_pool(session)
        
        print(f"Starting stress test: inserting {args.total_images} products...")
        tasks = [insert_image(session, sem, args.api_key, args.database, i, args.total_images, product_pool) 
                 for i in range(1, args.total_images + 1)]

        try:
            await asyncio.gather(*tasks)
        except KeyboardInterrupt:
            print("\nInterrupted!")

    final_duration = time.time() - start_time
    print(f"\nDone! Inserted {args.total_images} products in {final_duration:.2f} seconds")

if __name__ == "__main__":
    asyncio.run(main())