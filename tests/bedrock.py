import boto3
import json
import base64
from PIL import Image
import io
import requests
import time
import torch
import clip
import numpy as np

# Initialize Bedrock client
session = boto3.Session(profile_name='kencho')
bedrock = session.client(
    service_name='bedrock-runtime',
    region_name='eu-central-1'
)

# Initialize CLIP model
device = "cuda" if torch.cuda.is_available() else "cpu"
print(f"Using device: {device}")
clip_model, preprocess = clip.load("ViT-B/32", device=device)

def cosine_similarity(vec1, vec2):
    """Calculate cosine similarity between two vectors"""
    vec1 = np.array(vec1)
    vec2 = np.array(vec2)
    vec1_norm = vec1 / np.linalg.norm(vec1)
    vec2_norm = vec2 / np.linalg.norm(vec2)
    return np.dot(vec1_norm, vec2_norm)

def encode_image_to_base64(image_source):
    """Convert image to base64 string from URL or local path"""
    try:
        if image_source.startswith(('http://', 'https://')):
            headers = {
                'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
            }
            response = requests.get(image_source, headers=headers, stream=True, timeout=30)
            response.raise_for_status()
            image = Image.open(io.BytesIO(response.content))
        else:
            with open(image_source, "rb") as image_file:
                image = Image.open(image_file)
        
        if image.mode != 'RGB':
            image = image.convert('RGB')
            
        with io.BytesIO() as buffer:
            image.save(buffer, format="JPEG")
            image_data = buffer.getvalue()
            
        return base64.b64encode(image_data).decode('utf-8'), image
        
    except Exception as e:
        raise Exception(f"Failed to process image: {e}")

def get_titan_image_embedding(image_source, embedding_dimensions=384):
    """Get Titan embeddings for an image"""
    base64_image, pil_image = encode_image_to_base64(image_source)
    
    start_time = time.time()
    
    body = json.dumps({
        "inputImage": base64_image,
        "embeddingConfig": {
            "outputEmbeddingLength": embedding_dimensions
        }
    })
    
    response = bedrock.invoke_model(
        body=body,
        modelId="amazon.titan-embed-image-v1",
        accept="application/json",
        contentType="application/json"
    )
    
    titan_time = time.time() - start_time
    response_body = json.loads(response.get('body').read())
    
    return response_body['embedding'], pil_image, titan_time

def get_titan_text_embedding(text, embedding_dimensions=384):
    """Get Titan embeddings for text"""
    start_time = time.time()
    
    body = json.dumps({
        "inputText": text,
        "embeddingConfig": {
            "outputEmbeddingLength": embedding_dimensions
        }
    })
    
    response = bedrock.invoke_model(
        body=body,
        modelId="amazon.titan-embed-image-v1",
        accept="application/json",
        contentType="application/json"
    )
    
    titan_time = time.time() - start_time
    response_body = json.loads(response.get('body').read())
    
    return response_body['embedding'], titan_time

def get_clip_embeddings(pil_image, text_descriptions):
    """Get CLIP embeddings for image and text"""
    start_time = time.time()
    
    image_input = preprocess(pil_image).unsqueeze(0).to(device)
    text_inputs = clip.tokenize(text_descriptions).to(device)
    
    with torch.no_grad():
        image_features = clip_model.encode_image(image_input)
        image_features = image_features / image_features.norm(dim=-1, keepdim=True)
        
        text_features = clip_model.encode_text(text_inputs)
        text_features = text_features / text_features.norm(dim=-1, keepdim=True)
        
        similarities = (image_features @ text_features.T).softmax(dim=-1)
    
    clip_time = time.time() - start_time
    
    return {
        'image_embedding': image_features.cpu().numpy().flatten(),
        'text_embeddings': text_features.cpu().numpy(),
        'similarities': similarities.cpu().numpy().flatten(),
        'time': clip_time
    }

def compare_accuracy(image_url, text_descriptions, correct_index):
    """Compare accuracy between Titan and CLIP for image-text matching"""
    
    print(f"\nTesting: {text_descriptions}")
    print(f"Correct: '{text_descriptions[correct_index]}'")
    
    try:
        # Get Titan embeddings
        titan_img_emb, pil_image, titan_img_time = get_titan_image_embedding(image_url)
        
        titan_similarities = []
        titan_text_times = []
        
        for text in text_descriptions:
            titan_text_emb, titan_text_time = get_titan_text_embedding(text)
            titan_text_times.append(titan_text_time)
            similarity = cosine_similarity(titan_img_emb, titan_text_emb)
            titan_similarities.append(similarity)
        
        titan_total_time = titan_img_time + sum(titan_text_times)
        titan_predicted_index = np.argmax(titan_similarities)
        titan_correct = titan_predicted_index == correct_index
        titan_max_score = max(titan_similarities) * 100  # Convert to percentage
        
        # Get CLIP embeddings
        clip_results = get_clip_embeddings(pil_image, text_descriptions)
        clip_predicted_index = np.argmax(clip_results['similarities'])
        clip_correct = clip_predicted_index == correct_index
        clip_max_score = max(clip_results['similarities']) * 100  # Convert to percentage
        
        # Output with percentages
        print(f"Titan: {'✓' if titan_correct else '✗'} | {titan_total_time:.2f}s | {titan_max_score:.1f}% | '{text_descriptions[titan_predicted_index]}'")
        print(f"CLIP:  {'✓' if clip_correct else '✗'} | {clip_results['time']:.2f}s | {clip_max_score:.1f}% | '{text_descriptions[clip_predicted_index]}'")
        
        return {
            'titan_correct': titan_correct,
            'clip_correct': clip_correct,
            'titan_time': titan_total_time,
            'clip_time': clip_results['time'],
            'titan_score': titan_max_score,
            'clip_score': clip_max_score
        }
        
    except Exception as e:
        print(f"Error: {e}")
        return None

def run_accuracy_tests():
    """Run accuracy tests with simplified output"""
    
    test_cases = [
        {
            'image': 'https://images.saatchiart.com/saatchi/1061025/art/5361099/4430913-HSC00001-7.jpg',
            'descriptions': [
                'a pornographic scene showing nudity or sexual acts',
                'an image safe for work with no nudity or sexual content'
            ],
            'correct': 0,
            'name': 'Mountain Sunset Scene'
        },
    ]
    
    print("=" * 80)
    print("TITAN vs CLIP COMPARISON")
    print("=" * 80)
    
    results = []
    
    for test_case in test_cases:
        print(f"\n🧪 {test_case['name']}")
        result = compare_accuracy(
            test_case['image'], 
            test_case['descriptions'], 
            test_case['correct']
        )
        
        if result:
            results.append(result)
        
        time.sleep(1)
    
    # Summary
    if results:
        titan_wins = sum(r['titan_correct'] for r in results)
        clip_wins = sum(r['clip_correct'] for r in results)
        avg_titan_time = sum(r['titan_time'] for r in results) / len(results)
        avg_clip_time = sum(r['clip_time'] for r in results) / len(results)
        avg_titan_score = sum(r['titan_score'] for r in results) / len(results)
        avg_clip_score = sum(r['clip_score'] for r in results) / len(results)
        
        print(f"\n" + "=" * 80)
        print("SUMMARY")
        print("=" * 80)
        print(f"Accuracy: Titan {titan_wins}/{len(results)} | CLIP {clip_wins}/{len(results)}")
        print(f"Speed: Titan {avg_titan_time:.2f}s | CLIP {avg_clip_time:.2f}s")
        print(f"Avg Match %: Titan {avg_titan_score:.1f}% | CLIP {avg_clip_score:.1f}%")
        print(f"Winner: {'CLIP' if clip_wins > titan_wins else 'Titan' if titan_wins > clip_wins else 'Tie'}")

if __name__ == "__main__":
    run_accuracy_tests()
