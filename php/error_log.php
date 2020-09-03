<?php
	// Simple example of how to log an error from PHP
	//
	// This example should be run from a web server

	// This will go to the client
	echo("Hello from PHP!\n");

	// This will go to the web server error log
	error_log("Hello from PHP!\n");
?>
