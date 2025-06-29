import requests
import time
from multiprocessing import Process, Value, Lock
import random

def get_image_data(url):
    try:
        r = requests.get(url)
        r.raise_for_status()
        return r.content
    except:
        return None

API_KEY = "62d80151c294f137be9cf22a932dbb9c59e72a651a123641263ef45c5d2eb201"
DATABASE = "vecstore"

def get_random_image():
    """Get a random image from Lorem Picsum"""
    # Random dimensions between 400-800 pixels
    width = random.randint(400, 800)
    height = random.randint(400, 800)
    
    # Random seed for different images
    seed = random.randint(1, 1000)
    
    url = f"https://picsum.photos/seed/{seed}/{width}/{height}"
    filename = url
    
    return url, filename

def insert_image_loop(counter, limit, lock, start_time):
    while True:
        with lock:
            if counter.value >= limit:
                return
            counter.value += 1
            current = counter.value
        
        # Get random image URL and filename
        image_url, filename = get_random_image()
        
        # Download the random image
        img_data = get_image_data(image_url)
        if img_data is None:
            print(f"Failed to download random image {current}: {filename}")
            continue
        
        data = {
            'filename': filename, 
            'database': DATABASE, 
            'metadata': f'{{"category": "random", "image_id": {current}, "source": "picsum"}}'
        }
        files = {'image': (filename, img_data, 'image/jpeg')}
        headers = {"Authorization": API_KEY}
        
        try:
            res = requests.post("http://localhost:3000/insert-image", headers=headers, data=data, files=files)
            print(f"{current}/{limit} | {filename} | Response: {res.json()}")
        except Exception as e:
            print(f"{current}/{limit} | {filename} | ERROR: {e}")
        
        if current == limit:
            duration = time.time() - start_time.value
            print(f"\nInserted {limit} random images in {duration:.2f} seconds")

if __name__ == "__main__":
    num_processes = 4  # Adjust based on your needs
    total_images = 100  # Change this to however many random images you want
    
    counter = Value('i', 0)
    lock = Lock()
    start_time = Value('d', time.time())
    
    processes = [Process(target=insert_image_loop, args=(counter, total_images, lock, start_time)) for _ in range(num_processes)]
    
    for p in processes:
        p.start()
        
    for p in processes:
        p.join()
