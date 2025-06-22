import requests
import sys

API_KEY = "f9b5f0b6c1d6efd3c5861e0ea261a5864c6456b9b956130355cc46f543da42b7"

def get_image_data(url):
    """Download image from URL and return the binary data"""
    try:
        response = requests.get(url)
        response.raise_for_status()
        return response.content
    except requests.exceptions.RequestException as e:
        print(f"Error downloading image: {e}")
        return None

def get_filename_from_url(url):
    """Extract filename from URL or generate a default one"""
    try:
        filename = url.split('/')[-1].split('?')[0]  # Remove query params
        if '.' not in filename:
            filename = "image.jpg"  # Default if no extension found
        return filename
    except:
        return "image.jpg"

def insert_image():
    print("=== INSERT IMAGE ===")
    image_url = input("Enter image URL: ").strip()
    if not image_url:
        print("No URL provided!")
        return
    
    img_data = get_image_data(image_url)
    if not img_data:
        return
    
    filename = input(f"Enter filename: ").strip()
    if not filename:
        filename = get_filename_from_url(image_url)
    
    database = input("Enter database name (or press Enter for 'vecstore'): ").strip()
    if not database:
        database = 'vecstore'

    data = {
        'filename': filename,
        'database': database
    }

    files = {
        'image': (filename, img_data, 'image/jpeg')
    }

    headers = {
        "Authorization": API_KEY
    }

    print("Inserting image...")
    res = requests.post("http://localhost:3000/insert-image", headers=headers, data=data, files=files)
    print(f"Insert - Status: {res.status_code}, Response: {res.text}")

def search_by_text():
    print("=== SEARCH BY TEXT ===")
    
    text_query = input("Enter search text: ").strip()
    if not text_query:
        print("No search text provided!")
        return
    
    database = input("Enter database name (or press Enter for 'vecstore'): ").strip()
    if not database:
        database = 'vecstore'

    data = {
        'database': database,
        'text': text_query
    }

    headers = {
        "Authorization": API_KEY
    }

    print("Searching...")
    res = requests.post("http://localhost:3000/search", headers=headers, files=data)
    results = res.json().get("matches", [])
    print(f"\n🔍 Matches ({len(results)}):")
    for i, match in enumerate(results, 1):
        print(f"{i}. 📄 {match['filename']} — Score: {match['score']}")

def search_by_image():
    print("=== SEARCH BY IMAGE ===")
    
    image_url = input("Enter image URL: ").strip()
    if not image_url:
        print("No URL provided!")
        return
    
    img_data = get_image_data(image_url)
    if not img_data:
        return
    
    database = input("Enter database name (or press Enter for 'vecstore'): ").strip()
    if not database:
        database = 'vecstore'

    data = {
        'database': database
    }

    files = {
        'image': (get_filename_from_url(image_url), img_data, 'image/jpeg')
    }

    headers = {
        "Authorization": API_KEY
    }

    print("Searching by image...")
    res = requests.post("http://localhost:3000/search", headers=headers, data=data, files=files)
    results = res.json().get("matches", [])
    print(f"\n🔍 Matches ({len(results)}):")
    for i, match in enumerate(results, 1):
        print(f"{i}. 📄 {match['filename']} — Score: {match['score']}")

def main():
    print("🖼️  Image Vector Database CLI")
    print("=" * 30)
    print("Choose an option:")
    print("1. Insert image")
    print("2. Search by text")
    print("3. Search by image")
    print("4. Exit")
    
    while True:
        choice = input("\nEnter your choice (1-4): ").strip()
        
        if choice == '1':
            insert_image()
        elif choice == '2':
            search_by_text()
        elif choice == '3':
            search_by_image()
        elif choice == '4':
            print("Goodbye!")
            sys.exit(0)
        else:
            print("Invalid choice. Please enter 1, 2, 3, or 4.")
        

if __name__ == "__main__":
    main()
