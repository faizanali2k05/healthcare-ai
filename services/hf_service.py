import requests
import os

HF_API_KEY = os.getenv("HF_API_KEY")
HF_URL = "https://api-inference.huggingface.co/models/google/flan-t5-base"

headers = {
    "Authorization": f"Bearer {HF_API_KEY}"
}

def get_ai_response(message):
    if not HF_API_KEY or "PASTE_YOUR" in HF_API_KEY:
        return "Error: Hugging Face API key is missing or not configured correctly in the backend."

    payload = {
        "inputs": f"Provide medical advice: {message}"
    }

    try:
        response = requests.post(HF_URL, headers=headers, json=payload, timeout=10)
        response.raise_for_status()
        result = response.json()
        
        if isinstance(result, list) and len(result) > 0:
            return result[0].get("generated_text", "No response generated.")
        return str(result)
    except requests.exceptions.HTTPError as e:
        return f"Backend Error: Inference API returned {response.status_code}. Please check your HF API key."
    except Exception as e:
        return f"Backend Error: {str(e)}"
