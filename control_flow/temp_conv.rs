use std::io;

fn f_to_c(mut x: f64){

  x = (x-32.00)*(0.55555);
  println!("Temperature in Celsius is {}",x);
}
fn c_to_f(mut x: f64){

	x = (x*1.8)+32.00;
	println!("Temperature in Fahrenheit is {}",x);
}

fn main(){

	let mut choice = String::new();
	println!("Enter your choice");
	println!("1.Fahrenheit to Celsius");
	println!("2.Celsius to Fahrenheit");

	io::stdin().read_line(&mut choice).expect("Failed to read choice");

	let choice = match choice.trim().parse(){
		Ok(num) => num,
		Err(_) => -1
	};

	let mut temp = String::new();
	println!("Enter the Temperature");
	io::stdin().read_line(&mut temp).expect("Failed to read temp");

	let temp: f64 = match temp.trim().parse(){
		Ok(num) => num,
		Err(_) => -1.00
	};

	if choice == 1{

		f_to_c(temp);

	}else {
		c_to_f(temp);
	}

}