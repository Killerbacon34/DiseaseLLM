import requests
import json
import csv
import time
import os

totalcnt = 0

# Open input and output files
with open('d.csv', 'r') as file, open('o.csv', 'a', newline='') as outputfile:
    csv_reader = csv.reader(file)
    csv_writer = csv.writer(outputfile)
    
    # Create a temporary file to write unprocessed rows
    with open('d_temp.csv', 'w', newline='') as tempfile:
        temp_writer = csv.writer(tempfile)
        
        for row in csv_reader:
            print(row[0])
            x = True
            response = None
            while x:
                print("symptoms for ", row[0])
                response = requests.post(
                    url="https://openrouter.ai/api/v1/chat/completions",
                    headers={
                        "Authorization": "Bearer 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",
                        "Content-Type": "application/json",
                        # "HTTP-Referer": "<YOUR_SITE_URL>", # Optional. Site URL for rankings on openrouter.ai.
                        #"X-Title": "<YOUR_SITE_NAME>", # Optional. Site title for rankings on openrouter.ai.
                    },
                    data=json.dumps({
                        "model": "google/gemini-2.5-pro-exp-03-25:free",
                        "messages": [
                            {
                                "role": "user",
                                "content": "List 2-3 definite symptoms and some situational symptoms for {} , where each listed symptom, no matter the importance, is delineated by a slash. Do not respond with any reasoning, I just want the symptoms. Do not list which symptoms are definite or not, just the symptoms.".format(row[0])
                            }
                        ],
                        'provider': {
                            'order': [
                                "Targon",
                                'OpenAI',
                                'Together'
                            ],
                        }
                    })
                )
                if response.status_code == 200:
                    x = False
                else:
                    time.sleep(4)

            # Print the entire JSON response to inspect its structure
            response_json = response.json()
            #print("Full response JSON:", json.dumps(response_json, indent=2))

            # Extract the content from the JSON response
            try:
                content = response_json['choices'][0]['message']['content']
                if not content:
                    print("Content is empty, retrying...")
                    x = True
                    t = 5  # Initial sleep duration is 5 seconds
                    while x:
                        time.sleep(t)  # Pause execution for t seconds
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
                        if response.status_code == 200:
                            x = False
                        else:
                            time.sleep(4)  # Pause execution for 4 seconds before retrying
                    try:
                        response_json = response.json()
                        content = response_json['choices'][0]['message']['content']
                        if not content:
                            print("Content is still empty, retrying...")
                            t += 1  # Increase sleep duration by 1 second for the next retry
                        else:
                            totalcnt += 1
                            print(totalcnt, row[0], "Extracted content:", content)
                            csv_writer.writerow([row[0], content])  
                            x = False
                    except (KeyError, IndexError) as e:
                        print(f"Error extracting content: {e}")
                        print("Response JSON structure might have changed.")
                        temp_writer.writerow(row)  # Write the unprocessed row to the temporary file
                        x = False
                        continue 
                else:
                    totalcnt += 1
                    print(totalcnt, row[0], "Extracted content:", content)
                    csv_writer.writerow([row[0], content])  
            except (KeyError, IndexError) as e:
                print(f"Error extracting content: {e}")
                print("Response JSON structure might have changed.")
                temp_writer.writerow(row)  # Write the unprocessed row to the temporary file
                continue
            
            # Wait for 1 second before processing the next row
            time.sleep(1)

# Replace the original file with the temporary file
os.replace('d_temp.csv', 'd.csv')
exit(0)
