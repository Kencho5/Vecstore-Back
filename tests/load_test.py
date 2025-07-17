#!/usr/bin/env python3
import requests
import json
import base64
import time
import threading
from concurrent.futures import ThreadPoolExecutor, as_completed
from io import BytesIO
import argparse
import statistics

# Configuration
API_KEY = "f18718512e82150f1c813d750ebd0340a6198635a7b3650ed3077b69fc97725f"
BASE_URL = "https://api.vecstore.app"

RED_CAR_IMAGE_URL = "https://robbreport.com/wp-content/uploads/2016/09/lamborghini_huracan_slideshow_lead.jpg?w=772"
BLUE_CAR_IMAGE_URL = "https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/2019_Toyota_Corolla_Icon_Tech_VVT-i_Hybrid_1.8.jpg/960px-2019_Toyota_Corolla_Icon_Tech_VVT-i_Hybrid_1.8.jpg"
DATABASE = "vecstore"
TEXT_DATABASE = "test-db"

# Global cache for base64 images
image_cache = {}
stats_lock = threading.Lock()

def get_image_base64(url):
    if url in image_cache:
        return image_cache[url]
    
    try:
        r = requests.get(url)
        r.raise_for_status()
        base64_data = base64.b64encode(r.content).decode('utf-8')
        image_cache[url] = base64_data
        return base64_data
    except Exception as e:
        print(f"Failed to get image from {url}: {e}")
        return None

class TestStats:
    def __init__(self):
        self.response_times = []
        self.status_codes = []
        self.errors = []
        self.start_time = None
        self.end_time = None

    def add_result(self, response_time, status_code, error=None):
        with stats_lock:
            self.response_times.append(response_time)
            self.status_codes.append(status_code)
            if error:
                self.errors.append(error)

    def print_summary(self, route_name, num_requests, num_threads):
        with stats_lock:
            total_time = self.end_time - self.start_time if self.start_time and self.end_time else 0
            
            print(f"\n=== {route_name} Load Test Summary ===")
            print(f"Requests: {num_requests}, Threads: {num_threads}")
            print(f"Total Time: {total_time:.2f}s")
            print(f"Requests/sec: {num_requests/total_time:.2f}" if total_time > 0 else "Requests/sec: N/A")
            
            if self.response_times:
                print(f"Response Times:")
                print(f"  Min: {min(self.response_times):.3f}s")
                print(f"  Max: {max(self.response_times):.3f}s")
                print(f"  Avg: {statistics.mean(self.response_times):.3f}s")
                print(f"  Median: {statistics.median(self.response_times):.3f}s")
            
            # Status code summary
            status_summary = {}
            for code in self.status_codes:
                status_summary[code] = status_summary.get(code, 0) + 1
            
            print(f"Status Codes: {status_summary}")
            
            if self.errors:
                print(f"Errors: {len(self.errors)}")
                for error in self.errors[:5]:  # Show first 5 errors
                    print(f"  - {error}")

def make_request(route, payload, headers, stats):
    try:
        start = time.time()
        response = requests.post(f"{BASE_URL}{route}", headers=headers, json=payload, timeout=30)
        duration = time.time() - start
        
        stats.add_result(duration, response.status_code)
        return response.status_code, duration
        
    except Exception as e:
        duration = time.time() - start if 'start' in locals() else 0
        stats.add_result(duration, 0, str(e))
        return 0, duration

def run_load_test(route, payload_func, num_requests, num_threads):
    stats = TestStats()
    headers = {"Authorization": API_KEY, "Content-Type": "application/json"}
    
    print(f"\nStarting load test for {route}")
    print(f"Requests: {num_requests}, Threads: {num_threads}")
    
    stats.start_time = time.time()
    
    with ThreadPoolExecutor(max_workers=num_threads) as executor:
        futures = []
        
        for i in range(num_requests):
            payload = payload_func(i)
            if payload is None:
                print(f"Failed to generate payload for request {i}")
                continue
                
            future = executor.submit(make_request, route, payload, headers, stats)
            futures.append(future)
        
        # Wait for all requests to complete
        completed = 0
        for future in as_completed(futures):
            completed += 1
            if completed % max(1, num_requests // 10) == 0:
                print(f"Completed {completed}/{num_requests} requests")
    
    stats.end_time = time.time()
    stats.print_summary(route, num_requests, num_threads)

# Payload generators for each route
def insert_image_payload(request_num):
    image_url = RED_CAR_IMAGE_URL if request_num % 2 == 0 else BLUE_CAR_IMAGE_URL
    color = "red" if request_num % 2 == 0 else "blue"
    
    image_base64 = get_image_base64(image_url)
    if not image_base64:
        return None
    
    return {
        "image": image_base64,
        "database": DATABASE,
        "metadata": {"color": color, "type": "car", "request_num": request_num}
    }

def insert_text_payload(request_num):
    texts = [
        "apple pie recipe",
        "chocolate cake ingredients",
        "pasta cooking instructions",
        "grilled chicken marinade",
        "vegetable soup recipe"
    ]
    
    return {
        "text": f"{texts[request_num % len(texts)]} - request {request_num}",
        "database": TEXT_DATABASE,
        "metadata": {"type": "recipe", "request_num": request_num}
    }

def search_text_payload(request_num):
    queries = ["mountain", "car", "recipe", "apple", "blue", "red vehicle", "cooking", "nature"]
    return {
        "text": queries[request_num % len(queries)],
        "database": DATABASE,
        "limit": 5
    }

def search_image_payload(request_num):
    image_url = RED_CAR_IMAGE_URL if request_num % 2 == 0 else BLUE_CAR_IMAGE_URL
    image_base64 = get_image_base64(image_url)
    if not image_base64:
        return None
    
    return {
        "image": image_base64,
        "database": DATABASE,
        "limit": 5
    }

def nsfw_payload(request_num):
    image_base64 = get_image_base64(BLUE_CAR_IMAGE_URL)
    if not image_base64:
        return None
    
    return {"image": image_base64}

# Route configurations
ROUTES = {
    "1": {
        "name": "Insert Image",
        "route": "/insert-image",
        "payload_func": insert_image_payload
    },
    "2": {
        "name": "Insert Text", 
        "route": "/insert-text",
        "payload_func": insert_text_payload
    },
    "3": {
        "name": "Search Text",
        "route": "/search", 
        "payload_func": search_text_payload
    },
    "4": {
        "name": "Search Image",
        "route": "/search",
        "payload_func": search_image_payload
    },
    "5": {
        "name": "NSFW Detection",
        "route": "/nsfw",
        "payload_func": nsfw_payload
    }
}

def main():
    parser = argparse.ArgumentParser(description="API Load Testing Tool")
    parser.add_argument("--route", type=str, help="Route to test (1-5)")
    parser.add_argument("--requests", type=int, default=10, help="Number of requests")
    parser.add_argument("--threads", type=int, default=5, help="Number of threads")
    
    args = parser.parse_args()
    
    if args.route and args.route in ROUTES:
        route_config = ROUTES[args.route]
        run_load_test(
            route_config["route"],
            route_config["payload_func"],
            args.requests,
            args.threads
        )
        return
    
    print("=== API Load Testing Tool ===")
    print("Available routes:")
    for key, config in ROUTES.items():
        print(f"{key}. {config['name']} ({config['route']})")
    print("6. Run all routes")
    
    try:
        choice = input("\nSelect route to test (1-6): ").strip()
        
        if choice == "6":
            # Run all routes
            num_requests = int(input("Number of requests per route (default 10): ") or "10")
            num_threads = int(input("Number of threads (default 5): ") or "5")
            
            for key, config in ROUTES.items():
                print(f"\n{'='*50}")
                run_load_test(
                    config["route"],
                    config["payload_func"], 
                    num_requests,
                    num_threads
                )
                time.sleep(2)  # Brief pause between tests
            
        elif choice in ROUTES:
            num_requests = int(input("Number of requests (default 10): ") or "10")
            num_threads = int(input("Number of threads (default 5): ") or "5")
            
            route_config = ROUTES[choice]
            run_load_test(
                route_config["route"],
                route_config["payload_func"],
                num_requests, 
                num_threads
            )
        else:
            print("Invalid choice. Please select 1-6")
            
    except ValueError:
        print("Please enter valid numbers")
    except KeyboardInterrupt:
        print("\nTest interrupted by user")
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()