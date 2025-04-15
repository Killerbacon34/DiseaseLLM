# DiseaseLLM
This github contains the code used for the Disease Large Language Model capstone project. The goal for this project was to create a website/tool that would take in medical information from a user, and produce a medical diagnosis from multiple large language models.  This medical diagnosis is generated with the help of:
* Deepseek R1
* Llama
* Gemini

The goal for the project was to provide a user-friendly and intuitive interface for the public to obtain accurate medical diagnoses without the need to create an appointment with a doctor.
# Dependencies:
- Latest version of Rust is installed through rustup and in the system path (https://www.rust-lang.org/tools/install). Be sure to do this with Admin privileges.
- Latest version of nodejs and npm is installed (https://docs.npmjs.com/downloading-and-installing-node-js-and-npm).
- Docker Desktop is downloaded and running (https://www.docker.com/get-started/).
- Install Github on machine.
## Run Locally
Listed below are the steps for pulling this project and running it locally on your machine (assuming that your computer is a Windows Computer and you are doing these commands in Powershell):

## Step 1: Clone the Repository
Clone the project
```bash
  git clone https://github.com/Killerbacon34/DiseaseLLM.git
```
Then navigate into the project directory
```bash
  cd DiseaseLLM
```
## Step 2: Initialize Database
Install Docker Desktop using the link below
```
   https://www.docker.com/products/docker-desktop/
```
Pull postgres image
```bash
  docker pull postgres:latest
```
Pull redis image
```bash
  docker pull redis
```
Open the Docker Desktop Application and go to images. Then press the run button on the postgres image
![Screenshot 2025-04-15 144046](https://github.com/user-attachments/assets/3545607a-3dcb-4e74-b794-831ff7419a64)

Enter in the options in the image below and click run
![Screenshot 2025-04-15 144128](https://github.com/user-attachments/assets/700c7d8e-b0d7-4220-9add-7916954e9669)

Go to containers and select the start button on the one called DLLM


Then run the following commands to initialize the project database
```bash
  docker cp backend/sqlinit.sql DLLM:sqlinit.sql 
```
```bash
  docker exec -it DLLM psql -U user -f sqlinit.sql
```
```bash
  docker run --name my-redis -p 6379:6379 -d redis
```
Then, go back to containers and make sure the container for redis is currently running
Make sure to run the DLLM container within Docker Desktop, BOTH containers need to be running on the specfied ports for the backend to start.

## Step 3: Run Backend
In one terminal, navigate to source directory in backend
```bash
  cd backend
```
Start the backend, for first time runs it might take a bit longer as it is installing all the depencies required.
```bash
  cargo run
```
## Step 4: Run Frontend
Navigate to frontend source directory
```bash
  cd frontend/src
```
Install dependencies
```bash
  npm install
```
Start the server
```bash
  npm run dev
```
## Step 5: Visit site!
Open a browser of your choice and enter localhost:3000/ to navigate to the website of the project.

---
## If there are any issues with the code, please teams message Bryant Huang or email Bryant_huang1@baylor.edu or Alex_DeVries1@baylor.edu. We will respond to you as soon as we can.
## If there are ANY issues with the LLM key, please let us know. We are using an Openrouter.ai API KEY and API and it has given issues to us in the past. Bryant usually has to reenable the key, so please reach out.
