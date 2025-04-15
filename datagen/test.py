import requests
import json

response = requests.post(
  url="https://openrouter.ai/api/v1/chat/completions",
  headers={
    "Authorization": "Bearer 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",
    "Content-Type": "application/json",
  },
  data=json.dumps({
    "model": "google/gemini-2.5-pro-exp-03-25:free",
    "messages": [
      {
        "role": "user",
        "content": "What is the meaning of life?"
      }
    ],
    
  })
)

print(response.status_code)
output = response.json()
print(output)