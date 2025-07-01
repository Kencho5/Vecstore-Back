import requests
import time
from multiprocessing import Process, Value, Lock
import json

API_KEY = "62d80151c294f137be9cf22a932dbb9c59e72a651a123641263ef45c5d2eb201"
BASE_URL = "http://localhost:3000"

# Test data
IMAGE_URL = "https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/2019_Toyota_Corolla_Icon_Tech_VVT-i_Hybrid_1.8.jpg/960px-2019_Toyota_Corolla_Icon_Tech_VVT-i_Hybrid_1.8.jpg"
NSFW_IMAGE_URL = "https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/2019_Toyota_Corolla_Icon_Tech_VVT-i_Hybrid_1.8.jpg/960px-2019_Toyota_Corolla_Icon_Tech_VVT-i_Hybrid_1.8.jpg"
TEXT_CONTENT = "When Tony Stark, an industrialist, is captured, he constructs a high-tech armoured suit to escape."
DATABASE = "vecstore"
TEXT_DATABASE = "vecstore-text"

def get_image_data(url):
    try:
        r = requests.get(url)
        r.raise_for_status()
        return r.content
    except:
        return None

def search_worker(counter, limit, lock, start_time):
    while True:
        with lock:
            if counter.value >= limit:
                return
            counter.value += 1
            current = counter.value

        files = {
            'text': 'girl holding a guitar',
            'database':  DATABASE
        }
        headers = {"Authorization": API_KEY}
        
        try:
            res = requests.post(f"{BASE_URL}/search", headers=headers, files=files)
            if res.headers.get('content-type', '').startswith('application/json'):
                print(f"{current}/{limit} | Status: {res.status_code} | Response: {json.dumps(res.json(), indent=2)}")
            else:
                print(f"{current}/{limit} | Status: {res.status_code} | Response: {res.text}")
        except Exception as e:
            print(f"{current}/{limit} | Error: {e}")

        if current == limit:
            duration = time.time() - start_time.value
            print(f"\nCompleted {limit} search requests in {duration:.2f} seconds")

def insert_image_worker(counter, limit, lock, start_time):
    img_data = get_image_data(IMAGE_URL)
    if not img_data:
        print("Failed to get image data")
        return

    while True:
        with lock:
            if counter.value >= limit:
                return
            counter.value += 1
            current = counter.value

        data = {'filename': 'test.jpg', 'database': DATABASE, 'metadata': '{"category": "cars"}'}
        files = {'image': ('test.jpg', img_data, 'image/jpeg')}
        headers = {"Authorization": API_KEY}
        
        try:
            res = requests.post(f"{BASE_URL}/insert-image", headers=headers, data=data, files=files)
            if res.headers.get('content-type', '').startswith('application/json'):
                print(f"{current}/{limit} | Status: {res.status_code} | Response: {json.dumps(res.json(), indent=2)}")
            else:
                print(f"{current}/{limit} | Status: {res.status_code} | Response: {res.text}")
        except Exception as e:
            print(f"{current}/{limit} | Error: {e}")

        if current == limit:
            duration = time.time() - start_time.value
            print(f"\nCompleted {limit} image inserts in {duration:.2f} seconds")

def insert_text_worker(counter, limit, lock, start_time):
    while True:
        with lock:
            if counter.value >= limit:
                return
            counter.value += 1
            current = counter.value

        payload = {
            "text": TEXT_CONTENT,
            "database": TEXT_DATABASE, 
            'metadata': '{"category": "movies"}'
        }
        headers = {
            "Authorization": API_KEY,
            "Content-Type": "application/json"
        }
        
        try:
            res = requests.post(f"{BASE_URL}/insert-text", headers=headers, json=payload)
            if res.headers.get('content-type', '').startswith('application/json'):
                print(f"{current}/{limit} | Status: {res.status_code} | Response: {json.dumps(res.json(), indent=2)}")
            else:
                print(f"{current}/{limit} | Status: {res.status_code} | Response: {res.text}")
        except Exception as e:
            print(f"{current}/{limit} | Error: {e}")

        if current == limit:
            duration = time.time() - start_time.value
            print(f"\nCompleted {limit} text inserts in {duration:.2f} seconds")

def nsfw_worker(counter, limit, lock, start_time):
    img_data = get_image_data(NSFW_IMAGE_URL)
    if not img_data:
        print("Failed to get image data")
        return

    while True:
        with lock:
            if counter.value >= limit:
                return
            counter.value += 1
            current = counter.value

        files = {'image': (None, img_data, 'image/jpeg')}
        headers = {"Authorization": API_KEY}
        
        try:
            res = requests.post(f"{BASE_URL}/nsfw", headers=headers, files=files)
            if res.headers.get('content-type', '').startswith('application/json'):
                print(f"{current}/{limit} | Status: {res.status_code} | Response: {json.dumps(res.json(), indent=2)}")
            else:
                print(f"{current}/{limit} | Status: {res.status_code} | Response: {res.text}")
        except Exception as e:
            print(f"{current}/{limit} | Error: {e}")

        if current == limit:
            duration = time.time() - start_time.value
            print(f"\nCompleted {limit} NSFW checks in {duration:.2f} seconds")

def run_test(route, num_requests, num_threads):
    print(f"\nStarting {route} test with {num_requests} requests using {num_threads} threads...")
    
    counter = Value('i', 0)
    lock = Lock()
    start_time = Value('d', time.time())

    worker_map = {
        'search': search_worker,
        'insert-image': insert_image_worker,
        'insert-text': insert_text_worker,
        'nsfw': nsfw_worker
    }

    worker_func = worker_map.get(route)
    if not worker_func:
        print(f"Unknown route: {route}")
        return

    processes = [
        Process(target=worker_func, args=(counter, num_requests, lock, start_time))
        for _ in range(num_threads)
    ]
    
    for p in processes:
        p.start()
    for p in processes:
        p.join()

def main():
    print("=== Unified API Load Testing Tool ===")
    print("Available routes:")
    print("1. search")
    print("2. insert-image") 
    print("3. insert-text")
    print("4. nsfw")
    
    try:
        choice = int(input("\nSelect route (1-4): "))
        routes = ['search', 'insert-image', 'insert-text', 'nsfw']
        
        if choice < 1 or choice > 4:
            print("Invalid choice. Please select 1-4")
            return
            
        route = routes[choice - 1]
        
        num_requests = int(input("Enter number of requests: "))
        if num_requests <= 0:
            print("Number of requests must be positive")
            return
            
        num_threads = int(input("Enter number of threads: "))
        if num_threads <= 0:
            print("Number of threads must be positive")
            return
        
        run_test(route, num_requests, num_threads)
        
    except ValueError:
        print("Please enter valid numbers")
    except KeyboardInterrupt:
        print("\nTest interrupted by user")
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
