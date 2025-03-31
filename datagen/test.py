import requests
import json

response = requests.post(
  url="https://openrouter.ai/api/v1/chat/completions",
  headers={
    "Authorization": "Bearer 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 ",
    "Content-Type": "application/json",
    "HTTP-Referer": "<YOUR_SITE_URL>", # Optional. Site URL for rankings on openrouter.ai.
    "X-Title": "<YOUR_SITE_NAME>", # Optional. Site title for rankings on openrouter.ai.
  },
  data=json.dumps({
    "model": "deepseek/deepseek-r1-zero:free",
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