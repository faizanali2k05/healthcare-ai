from flask import Flask, request, jsonify
from supabase import create_client
import google.generativeai as genai
import os
from dotenv import load_dotenv

load_dotenv()

app = Flask(__name__)

# --- CONFIGURATION ---
SUPABASE_URL = os.getenv("SUPABASE_URL")
SUPABASE_KEY = os.getenv("SUPABASE_KEY")
GEMINI_API_KEY = os.getenv("GEMINI_API_KEY")

# Initialize Supabase
supabase = None
if SUPABASE_URL and SUPABASE_KEY:
    try:
        supabase = create_client(SUPABASE_URL, SUPABASE_KEY)
    except Exception as e:
        print(f"Supabase Client Init Failed: {e}")

# Initialize Gemini
if GEMINI_API_KEY:
    genai.configure(api_key=GEMINI_API_KEY)

@app.route("/")
def home():
    return {"status": "Healthcare AI Backend is Live!"}

@app.route("/chat", methods=["POST"])
def chat():
    data = request.json
    user_id = data.get("user_id")
    message = data.get("message")

    if not message:
        return jsonify({"reply": "Error: Send a proper message."}), 400

    try:
        # 1. Try Gemini
        try:
            model = genai.GenerativeModel('gemini-1.5-flash')
            response = model.generate_content(message)
            if response.candidates:
                reply = response.text
            else:
                reply = "AI: I cannot answer this due to safety filter."
        except Exception as ge:
            return jsonify({"reply": f"Gemini API Error: {str(ge)}"}), 200

        # 2. Try Supabase
        if supabase and user_id:
            try:
                supabase.table("chat_history").insert({
                    "user_id": user_id,
                    "user_message": message,
                    "bot_reply": reply
                }).execute()
            except Exception as se:
                print(f"Database Save Failed: {se}")

        return jsonify({"reply": reply})

    except Exception as e:
        return jsonify({"reply": f"Internal Server Error: {str(e)}"}), 200

if __name__ == "__main__":
    port = int(os.environ.get("PORT", 5000))
    app.run(host="0.0.0.0", port=port)
