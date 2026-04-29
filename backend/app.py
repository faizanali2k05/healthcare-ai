import os

from dotenv import load_dotenv
from flask import Flask, jsonify, request
from flask_cors import CORS

load_dotenv()

app = Flask(__name__)
CORS(app)

API_KEY = os.getenv("GEMINI_API_KEY", "")
MODEL_NAME = os.getenv("GEMINI_MODEL", "gemini-2.5-flash-lite")

# Edit these instructions any time you want to change how the assistant behaves.
SYSTEM_INSTRUCTIONS = os.getenv(
    "SYSTEM_INSTRUCTIONS",
    "You are a helpful healthcare assistant. Keep responses concise, "
    "friendly, and limited to 4 to 6 lines. Do not diagnose. Encourage users "
    "to contact a qualified medical professional for urgent or serious symptoms.",
)


def _model():
    if not API_KEY:
        raise RuntimeError("GEMINI_API_KEY is missing.")

    import google.generativeai as genai

    genai.configure(api_key=API_KEY)
    return genai.GenerativeModel(
        model_name=MODEL_NAME,
        system_instruction=SYSTEM_INSTRUCTIONS,
    )


@app.get("/")
def index():
    return jsonify(
        {
            "status": "ok",
            "service": "AI Healthcare Backend",
            "model": MODEL_NAME,
        }
    )


@app.get("/health")
def health():
    return jsonify({"status": "ok"})


@app.get("/routes")
def routes():
    return jsonify(
        {
            "routes": [
                "GET /",
                "GET /health",
                "GET /routes",
                "POST /chat",
            ]
        }
    )


@app.post("/chat")
def chat():
    data = request.get_json(silent=True) or {}
    message = str(data.get("message", "")).strip()

    if not message:
        return jsonify({"error": "Message is required."}), 400

    try:
        response = _model().generate_content(message)
        reply = (response.text or "").strip()
        if not reply:
            reply = "I could not generate a response. Please try again."
        return jsonify({"reply": reply})
    except Exception as exc:
        app.logger.exception("Chat generation failed")
        return jsonify({"error": str(exc)}), 500


@app.errorhandler(404)
def not_found(_error):
    return (
        jsonify(
            {
                "error": "Route not found.",
                "available_routes": ["/", "/health", "/routes", "/chat"],
            }
        ),
        404,
    )


if __name__ == "__main__":
    port = int(os.getenv("PORT", "8000"))
    app.run(host="0.0.0.0", port=port)
