import ollama

MODEL = "deepseek-r1:7b"
PROMPT = "List 2-3 definite symptoms and situational symptoms for prostate cancer, where each listed symptom, no matter the importance, is delineated by a slash. Do not respond with any reasoning, I just want the symptoms. Do not list which symptoms in any other way besides symptom/symptom/symptom" 

result = ollama.generate(MODEL, PROMPT)
response = result["response"]
print("Response:", response)