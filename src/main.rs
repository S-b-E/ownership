


fn main() {
    let mut s = "string";
    println!("S is: {}", s);
    s = " modification";
    println!("S is: {}", s);

    let string_one = String::from("String One");
    let string_two = string_one;

    // println!("The value of string_one is: {}", string_one); // This is an error! It wont even compile in Rust because the value has been moved to string_two.
    println!("The value of string_two is {}", string_two); // Works because string_two takes over ownership of the value

    let string_one = String::from("Strang");
    let string_two = string_one.clone();

    println!("The value of string_one is: {}", string_one); //Completely fine
    println!("The value of string_two is: {}", string_two);

    let x = 5;
    let y = x; // This is a copy. We get a new variable with the value of 5 copied into it.
    println!("x = {}, y = {}", x, y); // prints x = 5, y = 5

    let words = String::from("First Second Third");
    let first = first_word(&words);
    println!("This should say 'First': {}", first);
}



// Returns a slice of the given slice which contains the first word.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    // Enumerate returns each iterated segment of the variable as a tuple, the first value which
    // returns the index and the second value which contains the value of the enumerated piece.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // Comparing the item (a byte) to the byte for empty space
            return &s[0..i]; // return a string slice of start to the index of the byte for space
        }
    }

    &s[..] // If there is no space, return the whole string; Notice the implicit return without a semicolon.
}
