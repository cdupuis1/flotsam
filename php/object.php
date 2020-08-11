#!/usr/bin/php
<?php
	class tryme {
		function __construct($name) {
			$this->name = $name;
		}

		function print_name() {
			echo $this->name."\n";
		}
	}

	$some_func = new tryme("Phil McCrackin");
	$some_func->print_name();
?>
