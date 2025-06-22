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

API_KEY = "f9b5f0b6c1d6efd3c5861e0ea261a5864c6456b9b956130355cc46f543da42b7"
IMAGE_URL = "https://c.files.bbci.co.uk/18d0/live/88ff5600-d979-11ef-a5c8-1da73bd59591.jpg"
FILENAME = "image.jpg"
DATABASE = "vecstore"
IMG_DATA = get_image_data(IMAGE_URL)

def insert_image_loop(counter, limit, lock, start_time):
    while True:
        with lock:
            if counter.value >= limit:
                return
            counter.value += 1
            current = counter.value

        data = {'filename': FILENAME, 'database': DATABASE}
        files = {'image': (FILENAME, IMG_DATA, 'image/jpeg')}
        headers = {"Authorization": API_KEY}
        res = requests.post("http://localhost:3000/insert-image", headers=headers, data=data, files=files)
        print(f"{current}/{limit} | time: {res.json()}")

        if current == limit:
            duration = time.time() - start_time.value
            print(f"\nInserted {limit} images in {duration:.2f} seconds")

if __name__ == "__main__":
    num_processes = 1
    total_images = 1

    counter = Value('i', 0)
    lock = Lock()
    start_time = Value('d', time.time())

    processes = [Process(target=insert_image_loop, args=(counter, total_images, lock, start_time)) for _ in range(num_processes)]
    for p in processes:
        p.start()
    for p in processes:
        p.join()
