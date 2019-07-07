#
# test.sh
#
# Need to start server to test it
#!/bin/sh
./speed_server &
go test
kill %1