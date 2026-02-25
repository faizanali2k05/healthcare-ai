from supabase import create_client
import os

url = os.getenv("SUPABASE_URL")
key = os.getenv("SUPABASE_KEY")

supabase = None
if url and key and "SUPABASE_URL" not in url:
    try:
        supabase = create_client(url, key)
    except Exception as e:
        print(f"Supabase Client Error: {e}")

def save_chat(user_id, user_msg, bot_reply):
    if not supabase:
        print("Skipping save_chat: Supabase not configured.")
        return
        
    try:
        supabase.table("chat_history").insert({
            "user_id": user_id,
            "user_message": user_msg,
            "bot_reply": bot_reply
        }).execute()
    except Exception as e:
        print(f"Supabase Insert Error: {e}")
