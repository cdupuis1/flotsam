//
// Shows some sample code for borrowed pointers
//

fn print_num(num_to_print: i32) {
	println!("print_num(): num_to_print is {}", num_to_print);
}

// Take a Box reference allocated off the stack and change it
fn change_num(num_to_change: &mut Box<i32>) {
	println!("change_num(): Value of num_to_change is {} and address of num_to_change is {:p} before Box::new", num_to_change, num_to_change);
	*num_to_change = Box::new(53);
	println!("change_num(): Value of num_to_change is {} and address of num_to_change is {:p} after Box::new", num_to_change, num_to_change);
}

fn main() {
	let mut x = Box::new(75);
	println!("Value of x is {} at address {:p}", x, x);

	// Change value of Box value and print it
	*x = 30;
	print_num(*x);

	// Take a mutable reference
	let ptr_x = &mut x;
	println!("Address of ptr_x now is {:p} and a value of {}", ptr_x, *ptr_x);

	// Pass the mutable reference where it will be changed
	change_num(ptr_x);
	println!("ptr_x after change_num() is {} at address {:p}", ptr_x, ptr_x);
}

