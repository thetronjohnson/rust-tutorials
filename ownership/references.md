## References

```rust
fn main() {

	let s = String::from("Hello");

	let len = calculate_length(&s);

	println!("The length of {} is {}",s,len);
	
}

fn calculate_length(s: &String) -> usize {
	s.len()
}
```
The value is s is referenced to the function so it does not go out of scope after being passed

References as function parametes are termed Borrowing

References are immutable by default we cant change it

## Mutable References

```rust
fn main() {
	let mut s = String::from("hello");


	change(&mut s);
}

fn change(str: &mut String){
	str.push(", world!");
}
```
There can only be one mutable reference to a particular piece of data in a particular scope
The benefit of this restriction is that Rust can prevent data races at compile time ie:
	* Two or more pointers access the same data at the same time
	* At least one of the pointers is being used to write to the data
	* There's no mechanism being used to synchronize access to the data

