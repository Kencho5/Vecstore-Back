import asyncio
import aiohttp
import argparse
import base64
import random
import json

async def get_image_data(session, url):
    """Asynchronously downloads image data from a URL."""
    try:
        async with session.get(url) as response:
            response.raise_for_status()
            return await response.read()
    except aiohttp.ClientError as e:
        print(f"Error downloading image from {url}: {e}")
        return None

def get_random_image_url():
    """Generates a random URL from picsum.photos."""
    width = random.randint(400, 800)
    height = random.randint(400, 800)
    seed = random.randint(1, 1000000)
    url = f"https://picsum.photos/seed/{seed}/{width}/{height}"
    return url

async def main():
    parser = argparse.ArgumentParser(description="Search for an image in Vecstore using a random image query.")
    parser.add_argument("--api_key", required=True, help="Your Vecstore API Key.")
    parser.add_argument("--database", default="vecstore", help="The database name to search in.")
    parser.add_argument("--limit", type=int, default=3, help="Number of results to return.")
    parser.add_argument("--url", type=str, help="Optional: URL of the image to use for the search query. If not provided, a random image from Picsum will be used.")
    args = parser.parse_args()

    # Disable SSL verification - adjust if your environment has a proper certificate store
    connector = aiohttp.TCPConnector(ssl=False)
    async with aiohttp.ClientSession(connector=connector) as session:
        # 1. Get an image to use as the query
        if args.url:
            input_image_url = args.url
            print(f"Using provided image as input: {input_image_url}\n")
        else:
            input_image_url = get_random_image_url()
            print(f"Using random image as input: {input_image_url}\n")
        
        img_data = await get_image_data(session, input_image_url)
        if not img_data:
            print("Could not download image to use for the search query. Exiting.")
            return
            
        # 2. Prepare the payload for the search API
        base64_image = base64.b64encode(img_data).decode('utf-8')
        
        payload = {
            "image": base64_image,
            "database": args.database,
            "limit": args.limit
        }
        
        headers = {
            "Authorization": args.api_key,
            "Content-Type": "application/json"
        }
        
        # 3. Call the search API
        search_url = "https://api.vecstore.app/search"
        try:
            async with session.post(search_url, headers=headers, json=payload) as res:
                if res.status != 200:
                    print(f"Error: Server returned status {res.status}")
                    print(f"Response: {await res.text()}")
                    return

                response_data = await res.json()
                results = response_data.get("results", [])
                
                search_time = response_data.get('time', 'N/A')
                print(f"Search completed in: {search_time}")
                print(f"Found {len(results)} results:")
                
                if not results:
                    return

                # 4. Print results
                for i, result in enumerate(results):
                    score = result.get('score', 'N/A')
                    metadata = result.get('metadata', {})
                    result_image_url = metadata.get('picsum_url', 'No URL found in metadata')

                    print(f"  {i+1}. Score: {score}")
                    print(f"     Result URL: {result_image_url}\n")

        except aiohttp.ClientError as e:
            print(f"An error occurred during the request: {e}")
        except json.JSONDecodeError:
            print("Failed to decode the server's JSON response.")
            print(f"Raw response: {await res.text()}")

if __name__ == "__main__":
    asyncio.run(main())
