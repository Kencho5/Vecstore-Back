import requests
import json
import base64
import time
from io import BytesIO

API_KEY = "fc5d11c93d23207c35636a3b92c43b555e6a5950e9690e0c3e48e86d6d3c4ffc"
BASE_URL = "http://localhost:3000"
# API_KEY = "f18718512e82150f1c813d750ebd0340a6198635a7b3650ed3077b69fc97725f"
# BASE_URL = "https://api.vecstore.app"

RED_CAR_IMAGE_URL = "https://robbreport.com/wp-content/uploads/2016/09/lamborghini_huracan_slideshow_lead.jpg?w=772"
BLUE_CAR_IMAGE_URL = "https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/2019_Toyota_Corolla_Icon_Tech_VVT-i_Hybrid_1.8.jpg/960px-2019_Toyota_Corolla_Icon_Tech_VVT-i_Hybrid_1.8.jpg"
NSFW_IMAGE_URL = BLUE_CAR_IMAGE_URL
DATABASE = "test"
TEXT_DATABASE = "vecstore-text"

def get_image_base64(url):
    try:
        r = requests.get(url)
        r.raise_for_status()
        return base64.b64encode(r.content).decode('utf-8')
    except Exception as e:
        print(f"Failed to get image from {url}: {e}")
        return None

def test_insert_images():
    print("=== Testing Image Insertion ===")

    red_car_base64 = get_image_base64(RED_CAR_IMAGE_URL)
    if red_car_base64:
        payload = {"image": red_car_base64, "database": DATABASE, "metadata": {"color": "red", "type": "car"}}
        headers = {"Authorization": API_KEY, "Content-Type": "application/json"}
        try:
            start = time.time()
            res = requests.post(f"{BASE_URL}/insert-image", headers=headers, json=payload)
            duration = time.time() - start
            print(f"Red car insert - Status: {res.status_code} - Time: {duration:.2f}s")
            print(json.dumps(res.json(), indent=2) if res.headers.get('content-type', '').startswith('application/json') else res.text)
        except Exception as e:
            print(f"Red car insert error: {e}")

    blue_car_base64 = get_image_base64(BLUE_CAR_IMAGE_URL)
    if blue_car_base64:
        payload = {"image": blue_car_base64, "database": DATABASE, "metadata": {"color": "blue", "type": "car"}}
        headers = {"Authorization": API_KEY, "Content-Type": "application/json"}
        try:
            start = time.time()
            res = requests.post(f"{BASE_URL}/insert-image", headers=headers, json=payload)
            duration = time.time() - start
            print(f"Blue car insert - Status: {res.status_code} - Time: {duration:.2f}s")
            print(json.dumps(res.json(), indent=2) if res.headers.get('content-type', '').startswith('application/json') else res.text)
        except Exception as e:
            print(f"Blue car insert error: {e}")

def test_insert_texts():
    print("\n=== Testing Text Insertion ===")
    payload = {"text": "apple pie recipe", "database": TEXT_DATABASE, "metadata": {"type": "recipe", "category": "apple"}}
    headers = {"Authorization": API_KEY, "Content-Type": "application/json"}
    try:
        start = time.time()
        res = requests.post(f"{BASE_URL}/insert-text", headers=headers, json=payload)
        duration = time.time() - start
        print(f"Text insert - Status: {res.status_code} - Time: {duration:.2f}s")
        print(json.dumps(res.json(), indent=2) if res.headers.get('content-type', '').startswith('application/json') else res.text)
    except Exception as e:
        print(f"Text insert error: {e}")

def test_search_image():
    print("\n=== Testing Image Search ===")
    headers = {"Authorization": API_KEY, "Content-Type": "application/json"}
    
    payload = {"text": "mountain", "database": DATABASE }
    try:
        start = time.time()
        res = requests.post(f"{BASE_URL}/search", headers=headers, json=payload)
        duration = time.time() - start
        print(f"Search by text - Status: {res.status_code} - Time: {duration:.2f}s")
        print(json.dumps(res.json(), indent=2) if res.headers.get('content-type', '').startswith('application/json') else res.text)
    except Exception as e:
        print(f"Search by text error: {e}")
    
    red_car_base64 = get_image_base64(RED_CAR_IMAGE_URL)
    if red_car_base64:
        payload = {"image": red_car_base64, "database": DATABASE}
        try:
            start = time.time()
            res = requests.post(f"{BASE_URL}/search", headers=headers, json=payload)
            duration = time.time() - start
            print(f"Search by image - Status: {res.status_code} - Time: {duration:.2f}s")
            print(json.dumps(res.json(), indent=2) if res.headers.get('content-type', '').startswith('application/json') else res.text)
        except Exception as e:
            print(f"Search by image error: {e}")

def test_search_text():
    print("\n=== Testing Text Search ===")
    payload = {"text": "developer called giorgi", "database": TEXT_DATABASE }
    headers = {"Authorization": API_KEY, "Content-Type": "application/json"}
    try:
        start = time.time()
        res = requests.post(f"{BASE_URL}/search", headers=headers, json=payload)
        duration = time.time() - start
        print(f"Search text - Status: {res.status_code} - Time: {duration:.2f}s")
        print(json.dumps(res.json(), indent=2) if res.headers.get('content-type', '').startswith('application/json') else res.text)
    except Exception as e:
        print(f"Search text error: {e}")

def test_nsfw_detection():
    print("\n=== Testing NSFW Detection ===")
    nsfw_image_base64 = get_image_base64(NSFW_IMAGE_URL)
    if not nsfw_image_base64:
        print("Failed to get NSFW test image")
        return
    payload = {"image": nsfw_image_base64}
    headers = {"Authorization": API_KEY, "Content-Type": "application/json"}
    try:
        start = time.time()
        res = requests.post(f"{BASE_URL}/nsfw", headers=headers, json=payload)
        duration = time.time() - start
        print(f"NSFW detection - Status: {res.status_code} - Time: {duration:.2f}s")
        print(json.dumps(res.json(), indent=2) if res.headers.get('content-type', '').startswith('application/json') else res.text)
    except Exception as e:
        print(f"NSFW detection error: {e}")

def main():
    print("=== API Testing Script ===")
    print("1. Insert Images")
    print("2. Insert Texts")
    print("3. Search Images")
    print("4. Search Texts")
    print("5. NSFW Detection")
    print("6. Run All Tests")
    
    try:
        choice = int(input("\nSelect test (1-6): "))
        if choice == 1: test_insert_images()
        elif choice == 2: test_insert_texts()
        elif choice == 3: test_search_image()
        elif choice == 4: test_search_text()
        elif choice == 5: test_nsfw_detection()
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
    except ValueError:
        print("Please enter a valid number")
    except KeyboardInterrupt:
        print("\nTest interrupted by user")
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
