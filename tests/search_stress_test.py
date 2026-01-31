import asyncio
import aiohttp
import time
import random
import argparse
import base64
import statistics

async def get_image_data(session, url):
    """Download image from URL"""
    try:
        async with session.get(url) as response:
            response.raise_for_status()
            return await response.read()
    except aiohttp.ClientError as e:
        print(f"Failed to download image from {url}: {e}")
        return None

def get_random_image_url():
    """Generate random Picsum image URL using seed (no 404s)"""
    width = random.randint(400, 800)
    height = random.randint(400, 800)
    seed = random.randint(1, 1000000)
    url = f"https://picsum.photos/seed/{seed}/{width}/{height}"
    return url

async def search_image(session, sem, api_key, database, search_id, total_searches, search_times, use_text=False):
    """Perform a single search request"""
    async with sem:
        start_time = time.time()

        # Get random image for search
        image_url = get_random_image_url()
        img_data = await get_image_data(session, image_url)

        if img_data is None:
            print(f"[{search_id}/{total_searches}] Skipping failed image download")
            return

        base64_image = base64.b64encode(img_data).decode('utf-8')

        # Prepare payload
        payload = {
            'database': database,
            'limit': 10
        }

        if use_text:
            # Hybrid search (image + text)
            payload['image'] = base64_image
            payload['text'] = random.choice([
                'photo', 'landscape', 'person', 'city', 'nature',
                'building', 'animal', 'food', 'abstract', 'pattern'
            ])
        else:
            # Image-only search
            payload['image'] = base64_image

        headers = {
            "Authorization": api_key,
            "Content-Type": "application/json"
        }

        vecstore_url = "https://api.vecstore.app/search"
        try:
            async with session.post(vecstore_url, headers=headers, json=payload) as res:
                request_time = time.time() - start_time
                search_times.append(request_time * 1000)  # Convert to ms

                if res.status == 200:
                    response_json = await res.json()
                    results_count = len(response_json.get('results', []))
                    server_time = response_json.get('time', 'N/A')

                    if search_id % 50 == 0 or search_id == total_searches:
                        print(f"[{search_id}/{total_searches}] Status: {res.status} | "
                              f"Results: {results_count} | Server: {server_time} | "
                              f"Total: {request_time*1000:.0f}ms")
                else:
                    response_text = await res.text()
                    print(f"[{search_id}/{total_searches}] ERROR: Status {res.status} | Response: {response_text[:100]}")
        except aiohttp.ClientError as e:
            print(f"[{search_id}/{total_searches}] REQUEST FAILED: {e}")
        except Exception as e:
            print(f"[{search_id}/{total_searches}] UNEXPECTED ERROR: {e}")

async def main():
    parser = argparse.ArgumentParser(description="Search stress test for Vecstore API")
    parser.add_argument("--api_key", required=True, help="Your Vecstore API Key")
    parser.add_argument("--database", default="vecstore", help="The database name to search")
    parser.add_argument("--concurrency", type=int, default=10, help="Number of concurrent requests")
    parser.add_argument("--total_searches", type=int, default=1000, help="Total number of searches to perform")
    parser.add_argument("--hybrid", action="store_true", help="Use hybrid search (image + text)")
    args = parser.parse_args()

    search_type = "hybrid (image + text)" if args.hybrid else "image-only"
    print(f"Starting search stress test with concurrency {args.concurrency}")
    print(f"Performing {args.total_searches} {search_type} searches in database '{args.database}'...\n")

    start_time = time.time()
    sem = asyncio.Semaphore(args.concurrency)
    search_times = []

    connector = aiohttp.TCPConnector(ssl=False)
    async with aiohttp.ClientSession(connector=connector) as session:
        tasks = [
            search_image(session, sem, args.api_key, args.database, i, args.total_searches, search_times, args.hybrid)
            for i in range(1, args.total_searches + 1)
        ]

        try:
            await asyncio.gather(*tasks)
        except KeyboardInterrupt:
            print("\n\nCtrl+C detected! Cancelling all pending tasks...")
            for task in tasks:
                task.cancel()
            await asyncio.gather(*tasks, return_exceptions=True)
            final_duration = time.time() - start_time
            print(f"Interrupted after {final_duration:.2f} seconds")
            return

    final_duration = time.time() - start_time

    # Calculate statistics
    successful_searches = len(search_times)
    if successful_searches > 0:
        avg_time = statistics.mean(search_times)
        median_time = statistics.median(search_times)
        p95_time = statistics.quantiles(search_times, n=20)[18] if len(search_times) > 20 else max(search_times)
        p99_time = statistics.quantiles(search_times, n=100)[98] if len(search_times) > 100 else max(search_times)
        min_time = min(search_times)
        max_time = max(search_times)
        qps = successful_searches / final_duration

        print(f"\n{'='*60}")
        print(f"SEARCH STRESS TEST RESULTS")
        print(f"{'='*60}")
        print(f"Total searches:        {args.total_searches}")
        print(f"Successful searches:   {successful_searches}")
        print(f"Failed searches:       {args.total_searches - successful_searches}")
        print(f"Total duration:        {final_duration:.2f} seconds")
        print(f"Queries per second:    {qps:.2f} QPS")
        print(f"\nLatency Statistics (ms):")
        print(f"  Average (mean):      {avg_time:.0f}ms")
        print(f"  Median (p50):        {median_time:.0f}ms")
        print(f"  95th percentile:     {p95_time:.0f}ms")
        print(f"  99th percentile:     {p99_time:.0f}ms")
        print(f"  Min:                 {min_time:.0f}ms")
        print(f"  Max:                 {max_time:.0f}ms")
        print(f"{'='*60}")
    else:
        print(f"\nNo successful searches completed in {final_duration:.2f} seconds")

if __name__ == "__main__":
    asyncio.run(main())
