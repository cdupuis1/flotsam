#!/usr/bin/php
<?php
	# Sample PHP program to perform mysql database queries using PHP
	# database objects.  Note that these are somewhat insecure. 
	$servername = "localhost";
	$username = "chad";
	$password = "password";

	try {
		// Assumes user and password were already created
		$conn = new PDO("mysql:host=$servername;dbname=test", $username, $password);
		echo "Connection succeeded\n";

		// Create table
		$sql = "create table numbers(id INT, num INT)";
		$conn->exec($sql);
		echo "Create table succeeded\n";

		// Insert a number
		$sql = "insert into numbers (id, num) values (1, 1)";
		$conn->exec($sql);
		echo "Insert succeeded\n";

		// Get the number at id=1
		$stmt = $conn->prepare("select num from numbers where id=1");
		$stmt->execute();
		$result = $stmt->fetch();
		$the_num = intval($result["num"]);
		echo("the_num=" . strval($the_num) . "\n");

		// Add 1 to the number and update the database
		$the_num++;
		$sql = "update numbers set num=" . strval($the_num) . " where id=1";
		$conn->exec($sql);
		echo "Update succeeded\n";

		// Get the number again to confirm it was changed
		$the_num = 0; 
		$stmt = $conn->prepare("select num from numbers where id=1");
		$stmt->execute();
		$result = $stmt->fetch();
		$the_num = intval($result["num"]);
		echo("the_num=" . strval($the_num) . "\n");

		// Drop tabl
		$sql = "drop table numbers";
		$conn->exec($sql);
		echo "Drop table succeeded\n";
	} catch (PDOException $e) {
		echo "Connection failed: " . $e->getMessage();
	}

	// Close the connection
	$conn = null;
	echo "Disconnected\n";
?>
