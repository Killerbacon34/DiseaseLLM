-- Create the database
CREATE DATABASE DiseaseLLM;

-- Connect to the database (run this separately if needed)
\c diseasellm;

-- Create the users table
CREATE TABLE users (
    ID SERIAL PRIMARY KEY,
    Username TEXT NOT NULL UNIQUE,
    Password TEXT NOT NULL,
    DevID TEXT[], -- Array to store multiple strings
    Role INT NOT NULL,
    Approval BOOL,
    ApprovalSigned TIMESTAMP,
    TimeCreated TIMESTAMP NOT NULL
);

CREATE TABLE TOKENS (
    ID SERIAL PRIMARY KEY,
    Token TEXT NOT NULL,
    Username INT NOT NULL,
    TimeCreated TIMESTAMP NOT NULL
)
