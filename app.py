from flask import Flask, request, jsonify
from dotenv import load_dotenv
import os

from services.hf_service import get_ai_response
from services.supabase_service import save_chat

load_dotenv()

app = Flask(__name__)

@app.route("/")
def home():
    return {"status": "AI Healthcare Backend Running"}

@app.route("/chat", methods=["POST"])
def chat():
    data = request.json
    user_id = data.get("user_id")
    message = data.get("message")

    if not message:
        return jsonify({"error": "Message required"}), 400

    reply = get_ai_response(message)

    if user_id:
        save_chat(user_id, message, reply)

    return jsonify({"reply": reply})

if __name__ == "__main__":
    app.run()
