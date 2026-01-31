import requests
import argparse
import base64
import json
from PIL import Image
import io

def compress_image(img_data, quality=0.9, max_width=2000, max_height=2000):
    """
    Compress image to match frontend compression logic.
    - Resizes if larger than max dimensions (maintaining aspect ratio)
    - Converts to JPEG with specified quality
    - Uses high quality resampling (LANCZOS)
    """
    # Open image
    img = Image.open(io.BytesIO(img_data))

    # Convert to RGB if needed (for JPEG compatibility)
    if img.mode in ('RGBA', 'LA', 'P'):
        background = Image.new('RGB', img.size, (255, 255, 255))
        if img.mode == 'P':
            img = img.convert('RGBA')
        background.paste(img, mask=img.split()[-1] if img.mode in ('RGBA', 'LA') else None)
        img = background
    elif img.mode != 'RGB':
        img = img.convert('RGB')

    # Get original dimensions
    width, height = img.size

    # Resize if needed (maintaining aspect ratio)
    if width > max_width or height > max_height:
        scale_ratio = min(max_width / width, max_height / height)
        new_width = round(width * scale_ratio)
        new_height = round(height * scale_ratio)
        # LANCZOS is high quality resampling (equivalent to canvas imageSmoothingQuality='high')
        img = img.resize((new_width, new_height), Image.Resampling.LANCZOS)
        print(f"Resized from {width}x{height} to {new_width}x{new_height}")

    # Convert to JPEG bytes with specified quality
    output = io.BytesIO()
    jpeg_quality = int(quality * 100)  # Convert 0.9 to 80
    img.save(output, format='JPEG', quality=jpeg_quality, optimize=True)

    return output.getvalue()

def search_image(api_key, database, image_path=None, image_url=None, limit=5, quality=0.9):
    """
    Search for similar images in Vecstore using an image query.
    Either provide image_path (local file) or image_url (download from URL).
    """
    url = "https://api.vecstore.app/search"

    # Get image data
    if image_path:
        print(f"Reading local image: {image_path}")
        with open(image_path, 'rb') as f:
            img_data = f.read()
    elif image_url:
        print(f"Downloading image from: {image_url}")
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

    print(f"Original image size: {len(img_data)} bytes")

    # Compress image (matching frontend logic)
    compressed_data = compress_image(img_data, quality=quality)
    print(f"Compressed image size: {len(compressed_data)} bytes")

    # Encode to base64
    base64_image = base64.b64encode(compressed_data).decode('utf-8')

    print(f"Base64 length: {len(base64_image)} characters")
    print(f"Base64 prefix: {base64_image[:50]}...\n")

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
    print(f"Searching in database: {database}\n")
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
    parser.add_argument("--quality", type=float, default=1, help="JPEG compression quality (0.0-1.0, default 0.9)")
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
            limit=args.limit,
            quality=args.quality
        )
    except Exception as e:
        print(f"Error: {e}")
        import traceback
        traceback.print_exc()
