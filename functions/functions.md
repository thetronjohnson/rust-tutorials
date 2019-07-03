# Functions

main function -> Entry point of the program
keyword - fn

Rust uses snake case as a convention => All letters are lower case and underscores separate words

Rust does not care where you define your functions it can be before or after the main function

## Function Parameters

```rust
fn main() {
	print_num(5);
}

fn print_num(x:i32){
	println!("The value of x is: {}",x);
}
```

In function signatures the type of each parameters must be declared

## Functions with return values

- we donot name return values but we do declare their type after an arrow(->)

```rust
fn five()-> i32{
	5 // returns 5 
}

fn main(){
	let x = five()
	println!("The value of x is: {}",x);
}
```

