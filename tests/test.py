import requests
import json
import base64
from io import BytesIO

# API_KEY = "fc5d11c93d23207c35636a3b92c43b555e6a5950e9690e0c3e48e86d6d3c4ffc"
# BASE_URL = "http://localhost:3000"
API_KEY = "f18718512e82150f1c813d750ebd0340a6198635a7b3650ed3077b69fc97725f"
BASE_URL = "https://api.vecstore.app"

# Test data
RED_CAR_IMAGE_URL = "https://robbreport.com/wp-content/uploads/2016/09/lamborghini_huracan_slideshow_lead.jpg?w=772"
BLUE_CAR_IMAGE_URL = "https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/2019_Toyota_Corolla_Icon_Tech_VVT-i_Hybrid_1.8.jpg/960px-2019_Toyota_Corolla_Icon_Tech_VVT-i_Hybrid_1.8.jpg"
NSFW_IMAGE_URL = "https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/2019_Toyota_Corolla_Icon_Tech_VVT-i_Hybrid_1.8.jpg/960px-2019_Toyota_Corolla_Icon_Tech_VVT-i_Hybrid_1.8.jpg"
DATABASE = "vecstore"
TEXT_DATABASE = "vecstore-text"

def get_image_base64(url):
    """Download image and convert to base64"""
    try:
        r = requests.get(url)
        r.raise_for_status()
        return base64.b64encode(r.content).decode('utf-8')
    except Exception as e:
        print(f"Failed to get image from {url}: {e}")
        return None

def get_image_data(url):
    """Download image as bytes (for NSFW endpoint that still uses multipart)"""
    try:
        r = requests.get(url)
        r.raise_for_status()
        return r.content
    except Exception as e:
        print(f"Failed to get image from {url}: {e}")
        return None

def test_insert_images():
    """Insert 2 images: red car and blue car with metadata"""
    print("=== Testing Image Insertion ===")
    
    # Insert red car
    red_car_base64 = get_image_base64(RED_CAR_IMAGE_URL)
    if red_car_base64:
        payload = {
            "image": red_car_base64,
            "database": DATABASE,
            "metadata": {"color": "red", "type": "car"}
        }
        headers = {"Authorization": API_KEY, "Content-Type": "application/json"}
        
        try:
            res = requests.post(f"{BASE_URL}/insert-image", headers=headers, json=payload)
            print(f"Red car insert - Status: {res.status_code}")
            if res.headers.get('content-type', '').startswith('application/json'):
                print(f"Response: {json.dumps(res.json(), indent=2)}")
            else:
                print(f"Response: {res.text}")
        except Exception as e:
            print(f"Red car insert error: {e}")
    
    # Insert blue car
    blue_car_base64 = get_image_base64(BLUE_CAR_IMAGE_URL)
    if blue_car_base64:
        payload = {
            "image": blue_car_base64,
            "database": DATABASE,
            "metadata": {"color": "blue", "type": "car"}
        }
        headers = {"Authorization": API_KEY, "Content-Type": "application/json"}
        
        try:
            res = requests.post(f"{BASE_URL}/insert-image", headers=headers, json=payload)
            print(f"Blue car insert - Status: {res.status_code}")
            if res.headers.get('content-type', '').startswith('application/json'):
                print(f"Response: {json.dumps(res.json(), indent=2)}")
            else:
                print(f"Response: {res.text}")
        except Exception as e:
            print(f"Blue car insert error: {e}")

def test_insert_texts():
    """Insert 2 texts: fast red car and slow blue car"""
    print("\n=== Testing Text Insertion ===")
    
    # Insert fast red car text
    payload = {
        "text": "macbook pro 2019",
        "database": TEXT_DATABASE,
        "metadata": {"brand": "apple"}
    }
    headers = {"Authorization": API_KEY, "Content-Type": "application/json"}
    
    try:
        res = requests.post(f"{BASE_URL}/insert-text", headers=headers, json=payload)
        print(f"Fast red car text insert - Status: {res.status_code}")
        if res.headers.get('content-type', '').startswith('application/json'):
            print(f"Response: {json.dumps(res.json(), indent=2)}")
        else:
            print(f"Response: {res.text}")
    except Exception as e:
        print(f"Fast red car text insert error: {e}")
    
    # Insert slow blue car text
    payload = {
        "text": "windows laptop 2020",
        "database": TEXT_DATABASE,
        "metadata": {"company": "microsoft"}
    }
    
    try:
        res = requests.post(f"{BASE_URL}/insert-text", headers=headers, json=payload)
        print(f"Slow blue car text insert - Status: {res.status_code}")
        if res.headers.get('content-type', '').startswith('application/json'):
            print(f"Response: {json.dumps(res.json(), indent=2)}")
        else:
            print(f"Response: {res.text}")
    except Exception as e:
        print(f"Slow blue car text insert error: {e}")

def test_search_image():
    """Search images by text and by image"""
    print("\n=== Testing Image Search ===")
    
    # Search by text
    payload = {
        "text": "red car",
        "database": DATABASE,
        "metadata": {"color": "red"}
    }
    headers = {"Authorization": API_KEY, "Content-Type": "application/json"}
    
    try:
        res = requests.post(f"{BASE_URL}/search", headers=headers, json=payload)
        print(f"Search by text 'red car' - Status: {res.status_code}")
        if res.headers.get('content-type', '').startswith('application/json'):
            print(f"Response: {json.dumps(res.json(), indent=2)}")
        else:
            print(f"Response: {res.text}")
    except Exception as e:
        print(f"Search by text error: {e}")
    
    # Search by image
    red_car_base64 = get_image_base64(RED_CAR_IMAGE_URL)
    if red_car_base64:
        payload = {
            "image": red_car_base64,
            "database": DATABASE
        }
        
        try:
            res = requests.post(f"{BASE_URL}/search", headers=headers, json=payload)
            print(f"Search by image - Status: {res.status_code}")
            if res.headers.get('content-type', '').startswith('application/json'):
                print(f"Response: {json.dumps(res.json(), indent=2)}")
            else:
                print(f"Response: {res.text}")
        except Exception as e:
            print(f"Search by image error: {e}")

def test_search_text():
    """Search text by text query"""
    print("\n=== Testing Text Search ===")
    
    payload = {
        "text": "laptop made by apple",
        "database": TEXT_DATABASE,
        # "metadata": {"speed": "fast"}
    }
    headers = {"Authorization": API_KEY, "Content-Type": "application/json"}
    
    try:
        res = requests.post(f"{BASE_URL}/search", headers=headers, json=payload)
        print(f"Search text for 'fast car' - Status: {res.status_code}")
        if res.headers.get('content-type', '').startswith('application/json'):
            print(f"Response: {json.dumps(res.json(), indent=2)}")
        else:
            print(f"Response: {res.text}")
    except Exception as e:
        print(f"Search text error: {e}")

def test_nsfw_detection():
    """Test NSFW detection"""
    print("\n=== Testing NSFW Detection ===")
    
    nsfw_image_base64 = get_image_base64(NSFW_IMAGE_URL)
    if not nsfw_image_base64:
        print("Failed to get NSFW test image")
        return
    
    payload = {
        "image": nsfw_image_base64
    }
    headers = {"Authorization": API_KEY, "Content-Type": "application/json"}
    
    try:
        res = requests.post(f"{BASE_URL}/nsfw", headers=headers, json=payload)
        print(f"NSFW detection - Status: {res.status_code}")
        if res.headers.get('content-type', '').startswith('application/json'):
            print(f"Response: {json.dumps(res.json(), indent=2)}")
        else:
            print(f"Response: {res.text}")
    except Exception as e:
        print(f"NSFW detection error: {e}")

def main():
    print("=== API Testing Script ===")
    print("Available tests:")
    print("1. Insert Images (red car + blue car)")
    print("2. Insert Texts (fast red car + slow blue car)")
    print("3. Search Images (by text and image)")
    print("4. Search Texts (by text)")
    print("5. NSFW Detection")
    print("6. Run All Tests")
    
    try:
        choice = int(input("\nSelect test (1-6): "))
        
        if choice == 1:
            test_insert_images()
        elif choice == 2:
            test_insert_texts()
        elif choice == 3:
            test_search_image()
        elif choice == 4:
            test_search_text()
        elif choice == 5:
            test_nsfw_detection()
        elif choice == 6:
            print("Running all tests...")
            test_insert_images()
            test_insert_texts()
            test_search_image()
            test_search_text()
            test_nsfw_detection()
            print("\n=== All tests completed! ===")
        else:
            print("Invalid choice. Please select 1-6")
            return
        
    except ValueError:
        print("Please enter a valid number")
    except KeyboardInterrupt:
        print("\nTest interrupted by user")
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
