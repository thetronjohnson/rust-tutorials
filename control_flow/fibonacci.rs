use std::io;

fn fib(n: u32) -> u32{

	if n==1||n==0{
		1
	}else {
		fib(n-1) + fib(n-2)
	}
}

fn main() {
	println!("Enter a number: ");
	let mut num = String::new();
	io::stdin().read_line(&mut num).expect("Failed to read");

	let num = match num.trim().parse(){
		Ok(num) => num,
		Err(_) => 0,
	};

	let x = {
		fib(num-1)
	};
	println!("The nth fibonacci number is {}",x);
}