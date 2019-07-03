Rust is a statically typed language, it must know the types of all variables at compile time.

Data type subsets => (Scalar,Compound)

## Scalar Types

Represents a single value. Rust has four primary scalar types:
- integers
- floating-point numbers
- Booleans
- characters

### Integer Types

- No fraction components
signed integer(i) unsigned integer(u)

| Length | Signed | Unsigned |
|--------|--------|----------|
|8-bit   | i8     |u8        |
|16-bit  | i16    |u16       |
|32-bit  | i32    |u32       |
|64-bit  | i64    |u64       |
|128-bit | i128   |u128      |
|arch    | isize  |usize     |

Signed or unsigned => negative number or positive
isize and usize depends on the kind of computer the program is running on

Integer type default is i32

### Floating-Point Types

- f32 (single precision)
- f64 (double precision)(default)

### Boolean

- true
- false

one byte size

### Character Type

- single quotes
- four byte size
- Unicode Scalar Values

## Compound Types

Group multiple values into one type

### Tuple Type

- fixed length => once declared they can't grow or shrink
- coma separated list of values in parantheses

```rust
fn main() {
	let tup : (i32,f64,u8) = (500,6.4,1);
}
```

elements are accesible via <tuple_name>.<index>

### Array Type

Every element in an array must be of the same type
- fixed length
- coma separated values in square brackets

elements accesible <via array_name>[index]
