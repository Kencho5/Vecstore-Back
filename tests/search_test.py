import requests
import argparse
import json

def search_image_by_text(api_key, database, text_query, limit):
    """
    Searches for images in Vecstore using a text query.
    """
    url = "https://api.vecstore.app/search"
    
    payload = {
        "text": text_query,
        "database": database,
        "limit": limit,
    }
    
    headers = {
        "Authorization": api_key,
        "Content-Type": "application/json"
    }
    
    print(f"Searching in database '{database}' for: '{text_query}'...")
    
    try:
        res = requests.post(url, headers=headers, json=payload)
        if res.status_code != 200:
            print(f"Error: Server returned status {res.status_code}")
            print(f"Response: {res.text}")
            return

        response_data = res.json()
        results = response_data.get("results", [])
        
        if not results:
            print("No results found.")
            return
            
        search_time = response_data.get('time', 'N/A')
        print(f"\nSearch completed in: {search_time}")
        print(f"Found {len(results)} results:")
        
        for i, result in enumerate(results):
            score = result.get('score', 'N/A')
            metadata = result.get('metadata', {})
            
            image_url = metadata.get('picsum_url')
            
            if not image_url:
                image_url = metadata.get('filename', metadata.get('vector_id', 'No URL or ID found in metadata'))

            print(f"  {i+1}. Score: {score}")
            print(f"     Image URL: {image_url}\n")

    except requests.exceptions.RequestException as e:
        print(f"An error occurred during the request: {e}")
    except json.JSONDecodeError:
        print("Failed to decode the server's JSON response.")
        print(f"Raw response: {res.text}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Search for an image in Vecstore using a text query.")
    parser.add_argument("--api_key", required=True, help="Your Vecstore API Key.")
    parser.add_argument("--database", default="vecstore", help="The database name to search in.")
    parser.add_argument("--query", required=True, help="The text query to search for (e.g., 'a dog on a beach').")
    parser.add_argument("--limit", type=int, default=3, help="Number of results to return.")

    args = parser.parse_args()
    search_image_by_text(args.api_key, args.database, args.query, args.limit)
