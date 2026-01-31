import requests
import argparse
import base64
import os

def download_image(url):
    """
    Downloads image data from a URL, pretending to be a browser
    by sending a common User-Agent header.
    """
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
    }
    try:
        print(f"Downloading image from: {url}")
        r = requests.get(url, headers=headers)
        r.raise_for_status()
        print("Download successful.")
        return r.content
    except requests.exceptions.RequestException as e:
        print(f"Failed to download image: {e}")
        return None

def main():
    parser = argparse.ArgumentParser(description="Insert a single image into Vecstore from a URL.")
    parser.add_argument("--api_key", required=True, help="Your Vecstore API Key.")
    parser.add_argument("--database", default="vecstore", help="The database name to insert into.")
    parser.add_argument("--url", required=True, help="The URL of the image to insert.")
    args = parser.parse_args()

    # 1. Download the image
    img_data = download_image(args.url)
    if not img_data:
        return
        
    # 2. Base64-encode the image for the JSON payload
    base64_image = base64.b64encode(img_data).decode('utf-8')
    filename = os.path.basename(args.url)
    
    # 3. Construct the JSON payload
    payload = {
        'image': base64_image,
        'database': args.database,
        'metadata': {
            "source_url": args.url,
            "filename": filename
        }
    }

    headers = {
        "Authorization": args.api_key,
        "Content-Type": "application/json"
    }
    
    # 4. POST the data to the API
    vecstore_url = "https://api.vecstore.app/insert-image"
    print(f"Inserting image into database '{args.database}'...")
    try:
        res = requests.post(vecstore_url, headers=headers, json=payload)
        
        if res.status_code == 200:
            print(f"Success! Response: {res.json()}")
        else:
            print(f"Error: Server returned status {res.status_code}")
            print(f"Response: {res.text}")
            
    except requests.exceptions.RequestException as e:
        print(f"An error occurred during the request to Vecstore API: {e}")

if __name__ == "__main__":
    main()