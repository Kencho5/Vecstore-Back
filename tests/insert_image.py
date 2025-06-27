import requests
import time
from multiprocessing import Process, Value, Lock

def get_image_data(url):
    try:
        r = requests.get(url)
        r.raise_for_status()
        return r.content
    except:
        return None

API_KEY = "62d80151c294f137be9cf22a932dbb9c59e72a651a123641263ef45c5d2eb201"
IMAGE_URL = "https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/2019_Toyota_Corolla_Icon_Tech_VVT-i_Hybrid_1.8.jpg/960px-2019_Toyota_Corolla_Icon_Tech_VVT-i_Hybrid_1.8.jpg"
FILENAME = "car.jpg"
DATABASE = "vecstore"
IMG_DATA = get_image_data(IMAGE_URL)

def insert_image_loop(counter, limit, lock, start_time):
    while True:
        with lock:
            if counter.value >= limit:
                return
            counter.value += 1
            current = counter.value

        data = {'filename': FILENAME, 'database': DATABASE, 'metadata': '{"category": "cars"}'}
        files = {'image': (FILENAME, IMG_DATA, 'image/jpeg')}
        headers = {"Authorization": API_KEY}
        res = requests.post("http://localhost:3000/insert-image", headers=headers, data=data, files=files)
        print(f"{current}/{limit} | time: {res.json()}")

        if current == limit:
            duration = time.time() - start_time.value
            print(f"\nInserted {limit} images in {duration:.2f} seconds")

if __name__ == "__main__":
    num_processes = 5
    total_images = 100

    counter = Value('i', 0)
    lock = Lock()
    start_time = Value('d', time.time())

    processes = [Process(target=insert_image_loop, args=(counter, total_images, lock, start_time)) for _ in range(num_processes)]
    for p in processes:
        p.start()
    for p in processes:
        p.join()
