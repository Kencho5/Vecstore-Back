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

API_KEY = "cf7439a2cff6867a966bec4384d395f09a21b7af64c1e729d24a01ad8e44ea19"
IMAGE_URL = "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTCHo3CkaH0oRY3MvrEN0xgn-x_Lsn3Lm3lVQ&s"
IMG_DATA = get_image_data(IMAGE_URL)

def check_nsfw_loop(counter, limit, lock, start_time):
    while True:
        with lock:
            if counter.value >= limit:
                return
            counter.value += 1
            current = counter.value

        files = {'image': (None, IMG_DATA, 'image/jpeg')}
        headers = {"Authorization": API_KEY}
        res = requests.post("http://localhost:3000/nsfw", headers=headers, files=files)
        print(f"{current}/{limit} | nsfw: {res.text}")

        if current == limit:
            duration = time.time() - start_time.value
            print(f"\nChecked {limit} images in {duration:.2f} seconds")

if __name__ == "__main__":
    num_processes = 1
    total_images = 1

    counter = Value('i', 0)
    lock = Lock()
    start_time = Value('d', time.time())

    processes = [Process(target=check_nsfw_loop, args=(counter, total_images, lock, start_time)) for _ in range(num_processes)]
    for p in processes:
        p.start()
    for p in processes:
        p.join()

