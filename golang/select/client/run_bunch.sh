#!/bin/sh
#
# Run a bunch of clients in the background to saturate the server
#

go build client.go

i=0

while [ $i -lt 20 ]
do
	./client &
	i=$((i + 1))
done

rm -f client
