#!/bin/bash

sudo systemctl start redis-server
source flask_sse_env/bin/activate
gunicorn sse_generator:app --worker-class gevent --bind 127.0.0.1:5000
