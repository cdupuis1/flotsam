#!/usr/bin/php
<?php
	#
	# Sample PHP script to show how to do basic file operations.  In this
	# example we keep outputing the number on the last line plus 1.
	#
	$filename = "temp.txt";
	$handle = fopen($filename, "r");
	$line = "";

	if ($handle == FALSE) {
		# Try opening the file handle to create the file
		$handle = fopen($filename, "w") or die("Cannot create " . $filename);
		fclose($handle);
		$handle = fopen($filename, "r") or die("Cannot open " . $filename);
	}

	# Read line by line to end of the file
	while (!feof($handle)) {
		# We will read the last line and then still go through one more
		# loop iteration so we need to check the output of fgets before
		# we asign the output to $line
		if (($tmp = fgets($handle)) != false) {
			$line = $tmp;
		}
	}
	fclose($handle);

	# Convert the line to a number
	$num = intval($line);
	$num += 1;
	echo "Writing " . strval($num) . " to " . $filename . "\n";

	# Write the new number out to a file
	$handle = fopen($filename, "a") or die("Cannot open " . $filename);
	fwrite($handle, strval($num)."\n");
	fclose($handle);
?>
