#!/bin/bash

# cannot setfacl in Dockerfile for some reason
setfacl -m u:$SECRET_USER:r /home/$USER1/flag.txt
unset SECRET_USER
unset USER1

exec /usr/sbin/sshd -D