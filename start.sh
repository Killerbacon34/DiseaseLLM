#/bin/bash
$PASS="user" #Change here
$USER="cybears' #Change here
docker pull postgres:latest
docker run --name DLLMDB -e POSTGRES_PASSWORD=$PASS -e POSTGRES_USER=$USER -p 5432:5432 postgres:latest
docker pull redis:latest
docker run --name redis -d -p 6379:6379 redis:latest
cat .\sqlinit.sql | docker exec -i DLLMDB psql -h localhost -U $user 
