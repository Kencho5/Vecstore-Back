import torch
import clip
from PIL import Image
from flask import Flask, request, jsonify
import base64
import io
import requests
import time
import tracemalloc
import psutil
import os

app = Flask(__name__)

# Load CLIP model
device = "cuda" if torch.cuda.is_available() else "cpu"
model, preprocess = clip.load("ViT-B/32", device=device)

def monitor_performance():
    """Context manager to monitor performance metrics"""
    process = psutil.Process(os.getpid())
    tracemalloc.start()
    
    # Get initial values
    start_time = time.perf_counter()
    start_cpu_times = process.cpu_times()
    start_memory = process.memory_info().rss / 1024 / 1024  # MB
    
    def get_stats():
        # Get final values
        end_time = time.perf_counter()
        end_cpu_times = process.cpu_times()
        end_memory = process.memory_info().rss / 1024 / 1024  # MB
        cpu_percent = process.cpu_percent(interval=None)
        
        _, peak = tracemalloc.get_traced_memory()
        tracemalloc.stop()
        
        # Calculate metrics
        execution_time = end_time - start_time
        cpu_time_used = (end_cpu_times.user + end_cpu_times.system) - (start_cpu_times.user + start_cpu_times.system)
        memory_change = end_memory - start_memory
        
        return {
            "execution_time": round(execution_time, 6),
            "cpu_time_used": round(cpu_time_used, 6), 
            "cpu_utilization": round(cpu_percent, 2),
            "memory_change_mb": round(memory_change, 2),
            "peak_memory_mb": round(peak / 1024 / 1024, 2)
        }
    
    return get_stats

@app.route('/health', methods=['GET'])
def health_check():
    return jsonify({"status": "healthy"})

@app.route('/encode', methods=['POST'])
def encode_image():
    try:
        data = request.json
        image = None
        
        # Handle both base64 and URL inputs
        if 'image' in data:
            # Base64 encoded image
            image_data = data['image']
            image_bytes = base64.b64decode(image_data)
            image = Image.open(io.BytesIO(image_bytes))
        elif 'url' in data:
            # Image URL
            url = data['url']
            response = requests.get(url, timeout=10)
            response.raise_for_status()
            image = Image.open(io.BytesIO(response.content))
        else:
            return jsonify({"error": "Either 'image' (base64) or 'url' parameter required"}), 400
        
        # Start performance monitoring for transformation only
        get_stats = monitor_performance()
        
        # Preprocess and encode (this is what we're monitoring)
        image_input = preprocess(image).unsqueeze(0).to(device)
        
        with torch.no_grad():
            image_features = model.encode_image(image_input)
            # Normalize features
            image_features = image_features / image_features.norm(dim=-1, keepdim=True)
        
        # Get performance stats
        performance_stats = get_stats()
        
        # Convert to list for JSON serialization
        vector = image_features.cpu().numpy().flatten().tolist()
        
        return jsonify({
            "dimension": len(vector),
            "performance": performance_stats
        })
    
    except requests.exceptions.RequestException as e:
        return jsonify({"error": f"Failed to fetch image from URL: {str(e)}"}), 400
    except Exception as e:
        return jsonify({"error": str(e)}), 500

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8080)

