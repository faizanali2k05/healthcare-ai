from supabase import create_client
import os

url = os.getenv("SUPABASE_URL")
key = os.getenv("SUPABASE_KEY")

supabase = create_client(url, key)

def save_chat(user_id, user_msg, bot_reply):
    supabase.table("chat_history").insert({
        "user_id": user_id,
        "user_message": user_msg,
        "bot_reply": bot_reply
    }).execute()
