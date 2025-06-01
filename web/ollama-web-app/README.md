# Simple Ollama Webpage

The purpose of this particular application is to have a simple webpage that sends a POST request to an application which then relays the request to anollama server, gets the result and then returns the response back to the
webpage to display.

This ends up involving three different entities:

ollama server <-> rust rocket application <-> html page

The rust application and the ollama server communicate using a REST api call to http://server:11434/api/generate and then gets a JSON response from which we extract the response text.

To communicate between the rust application and the webpage we send a generic POST request to the rust application and use server-sent events (SSE) to push the response asynchronously back to the web page.

The rust application use the rocket framework to expose the api endpoints to send the POST request to and register an SSE connection wnith.








