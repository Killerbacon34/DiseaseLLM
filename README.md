# DiseaseLLM
This github contains the code used for the Disease Large Language Model capstone project. The goal for this project was to create a website/tool that would take in medical information from a user, and produce a medical diagnosis from multiple large language models.  This medical diagnosis is generated with the help of:
* Deepseek R1
* Llama
* Gemini
The goal for the project was to provide a user-friendly and intuitive interface for the public to obtain accurate medical diagnoses without the need to create an appointment with a doctor.


## Run Locally
Listed below are the steps for pulling this project and running it locally on your machine:
## Step 1: Clone the Repository
Clone the project

```bash
  git clone https://github.com/Killerbacon34/DiseaseLLM.git
```
## Step 2: Initialize Database
Install Docker Desktop using the link below
```
https://www.docker.com/products/docker-desktop/
```
Pull postgres image
```bash
  docker pull postgres
```
Pull redis image
```bash
  docker pull redis
```
Then run the following commands to create Docker containers for each
```bash
  docker cp sqlinit.sql DLLM:sqlinit.sql 
```
```bash
  docker exec -it DLLM psql -U user -f sqlinit.sql
```
```bash
  docker run --name redis-c -d redis -p 6379
```
Make sure to run the DLLM container within Docker Desktop
## Step 3: Run Backend
In one terminal, navigate to source directory in backend
```bash
  cd backend/src
```
Start the backend
```bash
  cargo run
```
## Step 4: Run Frontend
Navigate to frontend directory
```bash
  cd frontend
```
Install dependencies
```bash
  npm install
```
Navigate to source directory in the frontend
```bash
  cd src
```
Start the server
```bash
  npm run dev
```
## Step 5: Visit site!
Open a browser of your choice and enter localhost:3000/ to navigate to the website of the project


