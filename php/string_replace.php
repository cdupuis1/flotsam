#!/usr/bin/php
<?php
	# String replacement example
	$the_string = "This is now {bold}bold{/bold}\n";
	$the_string = str_replace("{bold}", "<b>", $the_string);
	$the_string = str_replace("{/bold}", "</b>", $the_string);
	echo $the_string;
?>
