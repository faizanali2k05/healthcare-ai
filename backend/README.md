# AI Healthcare Backend

## Local Run

```bash
pip install -r requirements.txt
python app.py
```

The local server runs on `http://localhost:8000`.

## Render Setup

Create a new Render Web Service and use:

- Root Directory: `backend`
- Build Command: `pip install -r requirements.txt`
- Start Command: `gunicorn app:app`

Add these environment variables in Render:

- `GEMINI_API_KEY`
- `GEMINI_MODEL=gemini-2.5-flash`
- `SYSTEM_INSTRUCTIONS`

After Render gives you a URL, put it in:

```dart
// lib/constants.dart
static const String backendUrl = 'https://your-service-name.onrender.com';
```
