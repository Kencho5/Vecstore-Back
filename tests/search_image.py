import requests
import json

API_KEY = "62d80151c294f137be9cf22a932dbb9c59e72a651a123641263ef45c5d2eb201"
DATABASE = "vecstore"

def search_with_text_only():
    """Test search with text only (no metadata filter)"""
    print("=== Testing search with text only ===")
    
    # Use files parameter to send multipart data
    files = {
        'text': (None, 'dog'),
        'database': (None, DATABASE)
    }
    headers = {"Authorization": API_KEY}
    
    response = requests.post("http://localhost:3000/search", headers=headers, files=files)
    print(f"Status: {response.status_code}")
    if response.headers.get('content-type', '').startswith('application/json'):
        print(f"Response JSON: {response.json()}")
    else:
        print("Response is not JSON")

def search_with_metadata_filter():
    """Test search with text and metadata filter"""
    print("=== Testing search with metadata filter ===")
    
    # Search for images with category = "landscape" and featured = true
    metadata_filter = {
        "category": "animal",
    }
    
    files = {
        'text': (None, 'landscape nature'),
        'database': (None, DATABASE),
        'metadata': (None, json.dumps(metadata_filter))
    }
    headers = {"Authorization": API_KEY}
    
    response = requests.post("http://localhost:3000/search", headers=headers, files=files)
    print(f"Status: {response.status_code}")
    print(f"Response content: {response.text}")
    if response.headers.get('content-type', '').startswith('application/json'):
        print(f"Response JSON: {response.json()}")
    else:
        print("Response is not JSON")

def search_with_single_metadata_filter():
    """Test search with single metadata filter"""
    print("=== Testing search with single metadata filter ===")
    
    # Search for images with only category = "landscape"
    metadata_filter = {
        "category": "animal"
    }
    
    files = {
        'text': (None, 'dog'),
        'database': (None, DATABASE),
        'metadata': (None, json.dumps(metadata_filter))
    }
    headers = {"Authorization": API_KEY}
    
    response = requests.post("http://localhost:3000/search", headers=headers, files=files)
    print(f"Status: {response.status_code}")
    print(f"Response JSON: {json.dumps(response.json(), indent=2)}")

def search_with_invalid_metadata():
    """Test search with invalid metadata format"""
    print("=== Testing search with invalid metadata ===")
    
    files = {
        'text': (None, 'landscape'),
        'database': (None, DATABASE),
        'metadata': (None, 'invalid json')
    }
    headers = {"Authorization": API_KEY}
    
    response = requests.post("http://localhost:3000/search", headers=headers, files=files)
    print(f"Status: {response.status_code}")
    print(f"Response content: {response.text}")
    if response.headers.get('content-type', '').startswith('application/json'):
        print(f"Response JSON: {response.json()}")
    else:
        print("Response is not JSON")
    print()

def search_with_nonexistent_metadata():
    """Test search with metadata that doesn't exist"""
    print("=== Testing search with non-existent metadata ===")
    
    metadata_filter = {
        "nonexistent": "value"
    }
    
    files = {
        'text': (None, 'landscape'),
        'database': (None, DATABASE),
        'metadata': (None, json.dumps(metadata_filter))
    }
    headers = {"Authorization": API_KEY}
    
    response = requests.post("http://localhost:3000/search", headers=headers, files=files)
    print(f"Status: {response.status_code}")
    print(f"Response content: {response.text}")
    if response.headers.get('content-type', '').startswith('application/json'):
        print(f"Response JSON: {response.json()}")
    else:
        print("Response is not JSON")
    print()

if __name__ == "__main__":
    print("Testing search functionality with metadata filtering\n")
    
    # Run all test cases
    # search_with_text_only()
    # search_with_metadata_filter()
    search_with_single_metadata_filter()
    # search_with_invalid_metadata()
    # search_with_nonexistent_metadata()
    
    print("All tests completed!")
