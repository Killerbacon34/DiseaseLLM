import requests
import json
import csv
import time

# Open input and output files
with open('d.csv', 'r') as file, open('o.csv', 'w', newline='') as outputfile:
    if file is None:
        print("File not found")
        exit(1)
    
    csv_reader = csv.reader(file)
    csv_writer = csv.writer(outputfile)
    
    for row in csv_reader:
        response = requests.post(
            url="https://openrouter.ai/api/v1/chat/completions",
            headers={
                "Authorization": "Bearer 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",
                "Content-Type": "application/json",
                # "HTTP-Referer": "<YOUR_SITE_URL>", # Optional. Site URL for rankings on openrouter.ai.
                #"X-Title": "<YOUR_SITE_NAME>", # Optional. Site title for rankings on openrouter.ai.
            },
            data=json.dumps({
                "model": "deepseek/deepseek-r1:free",
                "messages": [
                    {
                        "role": "user",
                        "content": "List 2-3 definite symptoms and some situational symptoms for {} , where each listed symptom, no matter the importance, is delineated by a slash. Do not respond with any reasoning, I just want the symptoms. Do not list which symptoms are definite or not, just the symptoms.".format(row[0])
                    }
                ],
            })
        )

        # Check if the request was successful
        if response.status_code != 200:
            print(f"Request failed with status code {response.status_code}")
            print(response.text)
            continue

        # Print the entire JSON response to inspect its structure
        response_json = response.json()
        print("Full response JSON:", json.dumps(response_json, indent=2))

        # Extract the content from the JSON response
        try:
            content = response_json['choices'][0]['message']['content']
            if not content:
                print("Content is empty")
            else:
                print("Extracted content:", content)
                csv_writer.writerow([row[0], content])  
        except (KeyError, IndexError) as e:
            print(f"Error extracting content: {e}")
            print("Response JSON structure might have changed.")
            continue
        time.sleep(3) 
        # Exit after the first iteration for debugging purposes
exit(0)
