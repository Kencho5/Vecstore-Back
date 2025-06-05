import requests
import base64
import json

url = "https://i.natgeofe.com/n/548467d8-c5f1-4551-9f58-6817a8d2c45e/NationalGeographic_2572187_16x9.jpg?w=1200"
img_data = requests.get(url).content
b64_image = base64.b64encode(img_data).decode()

payload = {
    "filename": "cat.jpg",
    "database": "vecstore",
    "image": f"data:image/jpeg;base64,{b64_image}"
}

headers = {
    "Content-Type": "application/json",
    "Authorization": "2cddb9fcc785ebac5b02be0c7a7056a63c5bde8b0db448198d626562e3643cea"
}

res = requests.post("http://localhost:3000/insert", headers=headers, data=json.dumps(payload))
print(res.status_code, res.text)

