# Ownership

Enables Rust to make memory safety guarantees without the need of a garbage collector

Programs have to manage the way to use the computers memory while running
Some languages have a garbage collector which looks for no longer used memory as the program runs
In other languages the programer explicitly does this

Rust uses an approach which is managed through a system of ownership with a set of rules that the compiler checks at runtime

### Ownership Rules

* Each value in rust has a variable that's called its owner
* There can only be one owner at a time
* When the owner goes out of scope the value will be dropped

```rust 
{					 // s is not valid here it's not yet declared
	let s ="hello"; // s is valid from this point
}				   // scope of s is over, s is no longer valid
```

## String Type

String is stored on a heap as such it is able to store an amount of text that is unknown at compile time

```rust

let s = String::from("hello");
// this kind of string can be mutated
s.push_str(", world!");
```

String can be mutated but literals cannot

### Memory Allocation

String literal => we know the contents at compile time so the text is hardcoded directly into the final executable
so string literals are fast and efficient

With th String type in order to support a mutable growable text it is allocated the memory in a heap
==> Memory requested from OS at runtime
=>> We need a way to return the memory to the OS after we are done with our string

Memory is requested when we call String::from

But the second part =>
	In languages with a garbage collector the GC keeps track and cleans up the memory not in use
	In Rust the memory is automatically returned once the variable that owns it goes out of scope
	When the variable goes out of scope Rust calls a special function "drop" and the memory is returned

When we assign a value stored in a stack to a variable or pass it to a function the value gets copied if the data value is stored in a heap the value gets moved and goes out of scope if it is moved to another function

