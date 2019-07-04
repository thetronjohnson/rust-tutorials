# Structure

eg:
``` rust
	struct User {
		username : String,
		email : String,
		sign_in_count: u64,
		active : bool,
	}
```
```rust
	let user1 = User {
		username : String::from("test"),
		email : String::from("test@example.com"),
		active: true,
		sign_in_count : 1,
	};
```
Elements accesible via dot notation
