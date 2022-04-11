## Scalar Types
A _scalar_ type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. You may recognize these from other programming languages. Let’s jump into how they work in Rust.

## Integer Types
An integer is a number without a fractional component. We used one integer type in Chapter 2, the u32 type. This type declaration indicates that the value it’s associated with should be an unsigned integer (signed integer types start with i, instead of u) that takes up 32 bits of space. Table 3-1 shows the built-in integer types in Rust. We can use any of these variants to declare the type of an integer value.

| Length | Signed |Unsigned |
|--------|--------|---------|
| 8-bit  | i8     | u8      |
| 16-bit | i16    | u16     |
| 32-bit | i32    | u32     |
| 64-bit | i64    | u64     |
| 128-bit| i128   | u128    |
| arch   | isize  | usize   |

> `i` belgisi minus & pilusli raqamlarni ishlatish uchun, `u` esa faqat musbat raqamlarni qo'llash uchun ishlatiladi.

Each variant can be either signed or unsigned and has an explicit size. Signed and unsigned refer to whether it’s possible for the number to be negative—in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned). It’s like writing numbers on paper: when the sign matters, a number is shown with a plus sign or a minus sign; however, when it’s safe to assume the number is positive, it’s shown with no sign. Signed numbers are stored using two’s complement representation.

Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses. So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.

Additionally, the isize and usize types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

You can write integer literals in any of the forms shown in Table 3-2. Note that number literals that can be multiple numeric types allow a type suffix, such as 57u8, to designate the type. Number literals can also use _ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.

| Number literals | Example     |
| --------------- | ------------|
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |

## Floating-Point Types
Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.

```rust
fn main(){
  let x = 2.0 // f64
  let y: f32 = 3.0 // f32
}
```
Floating-point numbers are represented according to the IEEE-754 standard. The f32 type is a single-precision float, and f64 has double precision.

## Numeric Operations
Rust supports the basic mathematical operations you’d expect for all of the number types: addition, subtraction, multiplication, division, and remainder. Integer division rounds down to the nearest integer. The following code shows how you’d use each numeric operation in a let statement:

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}
```

## The Boolean Type
As in most other programming languages, a Boolean type in Rust has two possible values: true and false. Booleans are one byte in size. The Boolean type in Rust is specified using bool. For example:

```rust
fn main() {
  let t = true;

  let f: bool = false; // with explicit type annotation
}
```

## The Character Type
Rust’s char type is the language’s most primitive alphabetic type. Here’s some examples of declaring char values:

```rust
fn main() {
  let c = 'z';
  let z = 'ℤ';
  let heart_eyed_cat = '😻';
}
```
Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes. Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust. Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust. We’ll discuss this topic in detail in “Storing UTF-8 Encoded Text with Strings” in Chapter 8.

## Compound Types
Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

## The Tuple Type
A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same. We’ve added optional type annotations in this example:

```rust
fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

The variable tup binds to the entire tuple, because a tuple is considered a single compound element. To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```
This program first creates a tuple and binds it to the variable tup. It then uses a pattern with let to take tup and turn it into three separate variables, x, y, and z. This is called destructuring, because it breaks the single tuple into three parts. Finally, the program prints the value of y, which is 6.4.

We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access. For example:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```
This program creates the tuple x and then makes new variables for each element by using their respective indices. As with most programming languages, the first index in a tuple is 0.

## The Array Type
Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.

We write the values in an array as a comma-separated list inside square brackets:

```rust
fn main() {
  let a = [1, 2, 3, 4, 5];
}
```
Arrays are useful when you want your data allocated on the stack rather than the heap (we will discuss the stack and the heap more in Chapter 4) or when you want to ensure you always have a fixed number of elements. An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size. If you’re unsure whether to use an array or a vector, chances are you should use a vector. Chapter 8 discusses vectors in more detail.

However, arrays are more useful when you know the number of elements will not need to change. For example, if you were using the names of the month in a program, you would probably use an array rather than a vector because you know it will always contain 12 elements:

```rust

#![allow(unused)]
fn main() {
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
}
```
You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:

```rust
#![allow(unused)]
fn main() {
let a: [i32; 5] = [1, 2, 3, 4, 5];
}
```
Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.

You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:

```rust

#![allow(unused)]
fn main() {
let a = [3; 5];
}
```
The array named a will contain 5 elements that will all be set to the value 3 initially. This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.

An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. You can access elements of an array using indexing, like this:
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```