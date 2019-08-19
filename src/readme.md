Overview
======
These are my notes on Rust's ownership concept. I also have included little illuminating code snippets in `src/main.rs`. These notes are mostly here to help me learn since I learn well by injesting information and then having to recall and explain it. These notes are also here to prove that I spent my training time actually doing training rather than sleeping all day or whatever.

Ownership
=========
Rust was created to be a fast, compiled memory-safe language that helps solve
the common problem of lower-level languages that do not use Garbage collection
and rely on manual allocation and deallocation. The way that rust accomplishes this memory-safety is through the concept of *ownership*: In Rust there are several rules for how data is referenced and those control how exposed any allocation of memory can be. The three rules that govern ownership of an piece of allocated memory in rust are:

* Each value in Rust has a variable called its owner.
* There can only ever be one owner at a time.
* When the owner goes out of scope, the value will be dropped (Deallocated)

These rules seem really simple, but they interact in a very interesting way with common programming concepts, such as aliasing, that might not seem readily apparent.

Reminder of Heap vs Stack
=========
I occasionally need a reminder of which is which and how they work, so here is a quick note that I can refer back to: The Stack is used for fixed-sized allocations that will not change. Typically 'primitives' are thrown here because they are of a known size. The Heap is where values with unknown sizes at compile-time must be stored. The heap is slower to access and allocate to because of this.

An example of the difference in Rust is how Strings are handled. If you declare a string literal (i.e. `let s = "a literal";`) it is stored on the stack because it is of fixed size and known at compile time. Other types of strings, such as accepting input from a stream, must be stored in allocated memory from the Heap because we don't know what will show up at compile-time. In fact, a string literal in rust is actually of the type 'str' or 'string slice' rather than *String*, which is the allocated type.

Aliasing
=========

A common concept in programming is aliasing, or assigning another variable to the value of another variable. This can be used to help organize code or to support certain patterns. In Rust, this concept is disallowed by assigning the value of one variable to the next because of rule #2 of Ownership: 'There can only ever be one owner at a time for a given value' (You can get a reference the value of another variable, however). Instead of creating an alias, this code performs a *move*:

```Rust
let string_one = String::from("String");
let string_two = string_one;

println!("The value of string_one is: {}", string_one); // This is an error!
```
This moves the ownership of the value 'String' from string_one to string_two. The attempt to print out the value of the original variable at the end of the code example would result in a compile-time error because the compiler tracks the ownership of each value and it can see that you moved the ownership to string_two.

Cloning and Copying
=========
While directly aliasing is disallowed and is replaced with moving ownership, cloning (for Heap types) and copying (For primitives/stack types)are allowed because this creates a separate variable with the same value:

```Rust
let string_one = String::from("String");
let string_two = string_one.clone();

println!("The value of string_one is: {}", string_one); //Completely
fine
println!("The value of string_two is: {}", string_two);

let x = 5;
let y = x; // This is a copy. We get a new variable with the value of 5 copied into it.
```

Functions and Ownership
=========
Another interesting ownership interaction is with Functions. The book gives multiple examples of this interaction, but the basic idea is that Functions take ownership just like variables. If you pass a full variable (Not a reference, but directly passing the variable into the function), the function assumes ownership of the value. If the value isn't passed back in the return of the function, it will be dropped when the function finishes. You cant pass a variable into a function directly and use the value after the function exits unless you pass back the value.

Referencing
=========
Reference are pointers to the value stored in another value. The ownership of the value is not transfered when you get a reference to it. The syntax to create a reference is an ampersand prepended to a variable:
```
var reference = &another_variable;
```
Since a reference does not gain ownership of the value, we can pass the reference freely to a function without losing the value due to the owner going out of scope.

The same rules of mutability also apply to references. Passing a simple reference, `&some_variable` creates an immutable reference, which will not allow the original value to be changed. To get a mutable reference, we need to use `&mut some_variable` instead.
You can have as many immutable reference to a value as you want, but rust imposes a limit to mutable references: You may only ever have a single mutable reference to a value, which is compiler-time checked to prevent data-race conditions, and while you have a mutable reference you may not create or use any immutable reference because immutable reference would potentially change behavior if the value changed underneath them.

You may create a mutable reference after creating an immutable one as long as your code never makes use of the immutable references after the mutable reference is created.

References are also compiler-checked to ensure that they do not reference a value that has gone out of scope. If you attempt to get a reference from a borrowed value, the compiler will throw an error because it would have created a dangling reference.

The example from the book is helpful in understanding this:
```
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```
Rust eliminates many common, hard-to-debug errors at compile time with these rules. Here is an example compiler error from the book that shows this:
```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 |
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 |
20 |     println!("the first word is: {}", word);
   |                                       ---- immutable borrow later used here
```
The clear function uses a mutable reference, so if the compiler didn't check this, we would still have the reference 'word' after the value had been blow away, leading us to have an error when trying to print the value of word.

Slices
=========
References in Rust can be obtained not only for entire values, but for subsections of those values. These subsections are called Slices.

String slices are a very common type of slice and can be declared like this: `&variable_name[<start index>..<end index>]`. You can omit the first index if it is 0 and you can omit both if you want a slice of the entire string. Strings can be passed as slices of themselves. The type in rust for a slice is `&str`.
