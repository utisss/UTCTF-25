#!/bin/bash

while [ true ]; do
	su -l $USER -c "socat -dd TCP4-LISTEN:9000,fork,reuseaddr EXEC:'/shellcode',pty,echo=0,raw,iexten=0"
done;
