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

API_KEY = "58315b813632e7469c99b0809393e9ec8bfeb2fa7a8f71d534a0b56f656bbb2e"
IMAGE_URL = "https://cdn.outsideonline.com/wp-content/uploads/2023/03/Funny_Dog_H.jpg?crop=25:14&width=500&enable=upscale"
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
        res = requests.post("http://localhost:3000/insert", headers=headers, data=data, files=files)
        print(f"{current}/{limit} | Status: {res.status_code}")

        if current == limit:
            duration = time.time() - start_time.value
            print(f"\nInserted {limit} images in {duration:.2f} seconds")

if __name__ == "__main__":
    num_processes = 5
    total_images = 20

    counter = Value('i', 0)
    lock = Lock()
    start_time = Value('d', time.time())

    processes = [Process(target=insert_image_loop, args=(counter, total_images, lock, start_time)) for _ in range(num_processes)]
    for p in processes:
        p.start()
    for p in processes:
        p.join()
