import requests
import json
import csv
import time
import os
import sys

totalcnt = 0

print("Input file name:")
input_file = input()
if not input_file:
    print("No input file provided. Exiting...")
    sys.exit(1)
if not os.path.exists(input_file):
    print("Input file does not exist. Exiting...")
    sys.exit(1)

# Open input and output files
with open(input_file, 'r') as file:
    csv_reader = csv.reader(file)
    Time = time.strftime("%Y-%m-%d %H:%M:%S", time.localtime())
    json_data = {}

    # Read first row of CSV file, skip first line
    for i, row in enumerate(csv_reader):
        if i == 0:
            print(f"Skipping header row: {row}")
            continue

        if len(row) < 2:
            print(f"Invalid row format on line {i + 1}. Skipping...")
            continue

        try:
            # Split the symptoms string into a list
            symptoms_list = row[1].split(",") if len(row) > 1 else []

            # Trim whitespace from each symptom
            symptoms_list = [symptom.strip() for symptom in symptoms_list]

            data = {
                "height": 0,
                "weight": 0,
                "age": 0,
                "gender": "",
                "race": "",
                "symptoms": symptoms_list,  # Use the list for symptoms
                "bloodpressure": 0,
                "heartrate": 0,
                "temperature": 0.0,
                "medications": [],
                "allergies": [],
                "alcohol": "",
                "smoking": "",
                "druguse": ""
            }
            json_data = data
            print(f"Row {i + 1} data: {json_data}")
        except Exception as e:
            print(f"Error processing row {i + 1}: {e}")
            sys.exit(1)

        try:
            # Upload
            headers = {
                'Content-Type': 'application/json',
            }
            response = requests.post("http://localhost:4545/insecure/results", headers=headers, json=json_data)
            if response.status_code != 200:
                print(f"Error: {response.status_code} for /api/uploadForm")
                continue
            else:
                print(f"Success: {response.status_code} for /api/uploadForm")
                totalcnt += 1
                response_data = response.json()
                print(f"Response: {response_data}")

        except requests.exceptions.RequestException as e:
            print(f"Request failed: {e}")
        except json.JSONDecodeError as e:
            print(f"JSON decode error: {e}")
        except Exception as e:
            print(f"An unexpected error occurred: {e}")

print(f"Total count: {totalcnt}")