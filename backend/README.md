# AI Healthcare Backend

## Local Run

```bash
pip install -r requirements.txt
python app.py
```

The local server runs on `http://localhost:8000`.

## Render Setup

Create a new Render Web Service. Use one of these setups.

### Recommended

Set the Render **Root Directory** to `backend`, then use:

- Root Directory: `backend`
- Build Command: `pip install -r requirements.txt`
- Start Command: `gunicorn app:app`

### If Root Directory Is Empty

If you leave Render **Root Directory** blank, use these commands instead:

- Build Command: `pip install -r backend/requirements.txt`
- Start Command: `gunicorn --chdir backend app:app`

The error `Could not open requirements file: requirements.txt` means Render
is using the second setup but still has the first build command.

Add these environment variables in Render:

- `GEMINI_API_KEY`
- `GEMINI_MODEL=gemini-2.5-flash`
- `SYSTEM_INSTRUCTIONS`

After Render gives you a URL, put it in:

```dart
// lib/constants.dart
static const String backendUrl = 'https://healthcare-ai-xt67.onrender.com';
```

For this project, the current Render URL is:

```text
https://healthcare-ai-xt67.onrender.com
```
