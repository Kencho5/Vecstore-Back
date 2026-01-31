import asyncio
import aiohttp
import time
import random
import argparse
import base64
import uuid

async def get_image_data(session, url):
    try:
        async with session.get(url) as response:
            response.raise_for_status()
            return await response.read()
    except aiohttp.ClientError as e:
        print(f"Failed to download image from {url}: {e}")
        return None

def get_random_image_url():
    # Use Picsum's random endpoint with seed for variety (no 404s)
    width = random.randint(400, 800)
    height = random.randint(400, 800)
    seed = random.randint(1, 1000000)
    url = f"https://picsum.photos/seed/{seed}/{width}/{height}"
    filename = f"picsum_seed_{seed}.jpeg"
    return url, filename

async def insert_image(session, sem, api_key, database, image_id, total_images):
    async with sem:
        image_url, filename = get_random_image_url()
        
        img_data = await get_image_data(session, image_url)
        if img_data is None:
            print(f"[{image_id}/{total_images}] Skipping failed download: {filename}")
            return

        base64_image = base64.b64encode(img_data).decode('utf-8')
        
        payload = {
            'image': base64_image,
            'database': database,
            'metadata': {
                "category": "random_async",
                "image_id": image_id,
                "source": "picsum",
                "filename": filename,
                "picsum_url": image_url
            }
        }

        headers = {
            "Authorization": api_key,
            "Content-Type": "application/json"
        }
        
        BATCH_REPORT_SIZE = 500 # Constant for reporting frequency
        
        vecstore_url = "https://api.vecstore.app/insert-image"
        try:
            async with session.post(vecstore_url, headers=headers, json=payload) as res:
                if res.status == 200:
                    if image_id % BATCH_REPORT_SIZE == 0 or image_id == total_images:
                        response_json = await res.json()
                        print(f"[{image_id}/{total_images}] | {filename} | Response: {res.status} {response_json}")
                else:
                    response_text = await res.text()
                    print(f"[{image_id}/{total_images}] | {filename} | ERROR: Status {res.status} | Response: {response_text}")
        except aiohttp.ClientError as e:
            print(f"[{image_id}/{total_images}] | {filename} | REQUEST FAILED: {e}")

async def main():
    parser = argparse.ArgumentParser(description="Async stress test for Vecstore API.")
    parser.add_argument("--api_key", required=True, help="Your Vecstore API Key.")
    parser.add_argument("--database", default="vecstore", help="The database name to insert into.")
    parser.add_argument("--concurrency", type=int, default=30, help="Number of concurrent requests.")
    parser.add_argument("--total_images", type=int, default=50000, help="Total number of images to insert.")
    args = parser.parse_args()

    print(f"Starting async stress test with concurrency {args.concurrency}, inserting {args.total_images} images into database '{args.database}'...")

    start_time = time.time()
    sem = asyncio.Semaphore(args.concurrency)

    connector = aiohttp.TCPConnector(ssl=False)
    async with aiohttp.ClientSession(connector=connector) as session:
        tasks = [insert_image(session, sem, args.api_key, args.database, i, args.total_images) for i in range(1, args.total_images + 1)]

        try:
            await asyncio.gather(*tasks)
        except KeyboardInterrupt:
            print("\n\nCtrl+C detected! Cancelling all pending tasks...")
            for task in tasks:
                task.cancel()
            # Wait briefly for cancellations to complete
            await asyncio.gather(*tasks, return_exceptions=True)
            final_duration = time.time() - start_time
            print(f"Interrupted after {final_duration:.2f} seconds")
            return

    final_duration = time.time() - start_time
    print(f"\ninserted {args.total_images} images in {final_duration:.2f} seconds")

if __name__ == "__main__":
    asyncio.run(main())
