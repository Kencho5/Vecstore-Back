import requests
import argparse
import base64
import json

def search_image(api_key, database, image_path=None, image_url=None, limit=5):
    """
    Search for similar images in Vecstore using an image query.
    Either provide image_path (local file) or image_url (download from URL).
    """
    url = "https://api.vecstore.app/search"

    # Get image data
    if image_path:
        with open(image_path, 'rb') as f:
            img_data = f.read()
    elif image_url:
        # Add browser headers to avoid 403 errors
        download_headers = {
            'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
            'Accept': 'image/avif,image/webp,image/apng,image/svg+xml,image/*,*/*;q=0.9',
            'Accept-Language': 'en-US,en;q=0.9',
            'Referer': image_url,
        }
        response = requests.get(image_url, headers=download_headers, timeout=10)
        response.raise_for_status()
        img_data = response.content
    else:
        raise ValueError("Must provide either image_path or image_url")

    # Encode to base64
    base64_image = base64.b64encode(img_data).decode('utf-8')

    # Prepare request
    payload = {
        "image": base64_image,
        "database": database,
        "limit": limit
    }

    headers = {
        "Authorization": api_key,
        "Content-Type": "application/json"
    }

    # Make request
    response = requests.post(url, headers=headers, json=payload)

    if response.status_code != 200:
        print(f"Error: {response.status_code}")
        print(f"Response: {response.text}")
        return

    # Parse response
    data = response.json()
    results = data.get("results", [])
    search_time = data.get("time", "N/A")

    print(f"Search completed in: {search_time}")
    print(f"Found {len(results)} results:\n")

    # Print results
    for i, result in enumerate(results, 1):
        vector_id = result.get("vector_id", "N/A")
        score = result.get("score", "N/A")
        content = result.get("content", "")
        metadata = result.get("metadata", {})

        print(f"{i}. Vector ID: {vector_id}")
        print(f"   Score: {score}")
        if content:
            print(f"   Content: {content[:100]}...")
        print(f"   Metadata: {json.dumps(metadata, indent=3)}")
        print()

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Search for similar images in Vecstore")
    parser.add_argument("--api_key", required=True, help="Your Vecstore API Key")
    parser.add_argument("--database", default="vecstore", help="Database name")
    parser.add_argument("--limit", type=int, default=5, help="Number of results")
    parser.add_argument("--image", help="Path to local image file")
    parser.add_argument("--url", help="URL to download image from")

    args = parser.parse_args()

    if not args.image and not args.url:
        print("Error: Must provide either --image (local file) or --url (download)")
        exit(1)

    try:
        search_image(
            api_key=args.api_key,
            database=args.database,
            image_path=args.image,
            image_url=args.url,
            limit=args.limit
        )
    except Exception as e:
        print(f"Error: {e}")
        import traceback
        traceback.print_exc()
