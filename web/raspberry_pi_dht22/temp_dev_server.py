#!/usr/bin/python
#
# DHT22 temperature sensor application using built in flash development server
#
from flask import Flask, jsonify, request
from http import HTTPStatus
import adafruit_dht
import board
import json

# Initialize the DHT sensor
dhtSensor = adafruit_dht.DHT22(board.D4)

# Create new flask app
app = Flask(__name__)

#Define a path to respond to and the HTTP method(s) we support 
@app.route("/temp", methods=['GET'])

# The naming convention here matters.  It should be <path>_<method>
def temp_get():
    # Get the temperature from the sensor
    try:
        humidity = dhtSensor.humidity
        temp_c = dhtSensor.temperature
    except RuntimeError:
        print("RuntimeError!")
        sys.exit()

    # Roundings and such.  This could be done on the client side as well
    # but I figure it makes the JSON easier if we put integers vs floats
    humidity = round(humidity)
    temp_c = round(temp_c)
    temp_f = round(temp_c * 9 / 5 + 32)

    # Create empty dict 
    temp = {}
    temp['temp'] = temp_f 
    temp['humidity'] = humidity

    # Convert dict to JSON response object
    response = jsonify(temp)

    # Need to add this response header so browsers will allow ajax requests to our server
    response.headers.add('Access-Control-Allow-Origin', '*')
    return response

# Start theh app
if __name__ == "__main__":
    # 0.0.0.0 allows other hosts to see flask server
    app.run(host="0.0.0.0")
