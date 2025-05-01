#/bin/bash
$PASS="user" #Change here
$USER="cybears' #Change here
docker pull redis:latest
docker run --name redis -d -p 6379:6379 redis:latest
