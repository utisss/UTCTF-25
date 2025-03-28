#!/bin/bash

#Copy binary from temporary container
docker-compose up --no-start --build
docker cp $(docker ps -alq):/shellcode .
docker cp $(docker ps -alq):/lib/x86_64-linux-gnu/libc-2.23.so .
