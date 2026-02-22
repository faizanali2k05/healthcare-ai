import requests
import os

HF_API_KEY = os.getenv("HF_API_KEY")
HF_URL = "https://api-inference.huggingface.co/models/google/flan-t5-base"

headers = {
    "Authorization": f"Bearer {HF_API_KEY}"
}

def get_ai_response(message):
    payload = {
        "inputs": f"Provide medical advice: {message}"
    }

    response = requests.post(HF_URL, headers=headers, json=payload)
    result = response.json()

    return result[0]["generated_text"]
