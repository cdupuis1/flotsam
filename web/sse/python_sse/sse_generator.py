#!/usr/bin/python3
#
# Sample flask app that let's clients subscribe and push SSE events
#
from flask import Flask
from flask_sse import sse
from flask_cors import CORS
import json

# Declare flask app
app = Flask(__name__)

# Setup the app for Cross Origin Resource Sharing so the calling webpage can
# Access us if the page was opened from the filesystem
CORS(app)

# Configure a redis server so a client can subscript to our pushes
app.config["REDIS_URL"] = "redis://"

# Set up the flask app so clients can use the /stream API node to subscribe to
# events
app.register_blueprint(sse, url_prefix='/stream')

# Global for keeping track of the current id
serial_num = 1

# Define the /events API node.  If someone calls use publish a string as SSE data
# to subscribers
@app.route("/events")
def events():
    global serial_num
    message = "The id is {} <br>".format(serial_num)
    data = {'id': serial_num, 'message': message}

    # Push the data to clients
    sse.publish(data)

    # Increment the serial_num for next time
    serial_num = serial_num + 1
    return "Update published!"

if __name__ == '__main__':
    app.run(debug=True)
