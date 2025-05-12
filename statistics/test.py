#!/usr/bin/env python3
import requests
import json
import csv
import time
import os
import sys

def main():
    print("Input file name:")
    input_file = input().strip()
    if not input_file:
        print("No input file provided. Exiting...")
        sys.exit(1)
    if not os.path.exists(input_file):
        print("Input file does not exist. Exiting...")
        sys.exit(1)

    results = []
    totalcnt = 0

    with open(input_file, 'r', newline='', encoding='utf-8') as infile:
        with open('d.csv', 'r', newline='', encoding='utf-8') as dfile:
            csv_reader = csv.reader(infile)
            # Read the first line of d.csv to get the header
            d_reader = csv.reader(dfile)
            for j, low in enumerate(d_reader):
                if j  < 20 :
                    print(f"Skipping header row: {low}")
                    continue
                if len(low) < 2:
                    print(f"Invalid row format on line {j + 1}. Skipping...")
                    continue
                #get the attributes from the line of d.csv
                # get each individual column of the row
                for i, row in enumerate(csv_reader):
                    if i == 0:
                        print(f"Skipping header row: {row}")
                        continue
                    if len(row) < 2:
                        print(f"Invalid row format on line {i + 1}. Skipping...")
                        continue
                    bloodPressure = 110
                    # Build your JSON payload
                    if low[7] == "low":
                        bloodPressure = 80
                    elif low[7] == "high":
                        bloodPressure = 130
                    elif low[7] == "normal":
                        bloodPressure = 110
                    symptoms_list = [s.strip() for s in row[1].split(",")]
                    payload = {
                        "height": 0,
                        "weight": 0,
                        "age": int(low[5]),
                        "gender": low[6],
                        "race": "unknown",
                        "symptoms": symptoms_list,
                        "bloodpressure": bloodPressure,
                        "heartrate": 0,
                        "temperature": 0,
                        "medications": [],
                        "allergies": [],
                        "alcohol": "",
                        "smoking": "",
                        "druguse": ""
                    }
                    print(f"Row {i + 1} payload: {payload}")

                    try:
                        response = requests.post(
                            "http://localhost:4545/insecure/results",
                            headers={'Content-Type': 'application/json'},
                            json=payload
                        )
                        if response.status_code != 200:
                            print(f"Error: {response.status_code} for row {i + 1}")
                            continue

                        response_data = response.json()
                        totalcnt += 1

                        # Extract and clean each field
                        consensus = response_data.get('Consensus', '').strip()
                        deepseek  = response_data.get('Deepseek', '').replace('\n', ' ').strip()
                        gemini    = response_data.get('Gemini',   '').replace('\n', ' ').strip()
                        llama     = response_data.get('Llama',    '').replace('\n', ' ').strip()

                        results.append({
                            'Row': i + 1,
                            'Consensus': consensus,
                            'Deepseek': deepseek,
                            'Gemini': gemini,
                            'Llama': llama
                        })

                        print(f"Row {i + 1} parsed. Consensus: {consensus}, Deepseek: {deepseek}, Gemini: {gemini}, Llama: {llama}")

                    except requests.exceptions.RequestException as e:
                        print(f"Request failed on row {i + 1}: {e}")
                    except json.JSONDecodeError as e:
                        print(f"JSON decode error on row {i + 1}: {e}")
                    except Exception as e:
                        print(f"Unexpected error on row {i + 1}: {e}")

    # Write everything to parsed_results.csv
    output_file = 'parsed_results.csv'
    with open(output_file, 'w', newline='', encoding='utf-8') as outcsv:
        fieldnames = ['Row', 'Consensus', 'Deepseek', 'Gemini', 'Llama']
        writer = csv.DictWriter(outcsv, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(results)

    print(f"Total successful rows: {totalcnt}")
    print(f"Wrote parsed output to {output_file}")

if __name__ == "__main__":
    main()
