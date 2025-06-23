import requests
import time
from multiprocessing import Process, Value, Lock

API_KEY = "cf7439a2cff6867a966bec4384d395f09a21b7af64c1e729d24a01ad8e44ea19"
DATABASE = "vecstore-text"
TEXT = "When Tony Stark, an industrialist, is captured, he constructs a high-tech armoured suit to escape. Once he manages to escape, he decides to use his suit to fight against evil forces to save the world."
URL = "http://localhost:3000/insert-text"

def insert_text_loop(counter, limit, lock, start_time):
    while True:
        with lock:
            if counter.value >= limit:
                return
            counter.value += 1
            current = counter.value

        payload = {
            "text": TEXT,
            "database": DATABASE, 
            'metadata': '{"category": "landscape", "featured": true}'
        }
        headers = {
            "Authorization": API_KEY,
            "Content-Type": "application/json"
        }
        res = requests.post(URL, headers=headers, json=payload)
        try:
            print(f"{current}/{limit} | time: {res.json()}")
        except Exception:
            print(f"{current}/{limit} | status: {res.status_code}")

        if current == limit:
            duration = time.time() - start_time.value
            print(f"\nInserted {limit} texts in {duration:.2f} seconds")

if __name__ == "__main__":
    num_processes = 1
    total_texts = 1

    counter = Value('i', 0)
    lock = Lock()
    start_time = Value('d', time.time())

    processes = [
        Process(
            target=insert_text_loop,
            args=(counter, total_texts, lock, start_time)
        )
        for _ in range(num_processes)
    ]
    for p in processes:
        p.start()
    for p in processes:
        p.join()
