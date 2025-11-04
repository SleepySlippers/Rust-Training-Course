// This chapter is dedicated to the ownership, borrowing and slices

// OWNERSHIP
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `longest_owned(s1: String, s2: String) -> String` that returns the longer of
// two strings. Check that both original strings are moved into the function, and only the returned
// can still be used.
//
// You can implement the function and use it right inside the `string_ownership` function.
fn longest_owned(s1: String, s2: String) -> String {
    if s1.len() > s2.len() { s1 } else { s2 }
}
#[allow(dead_code)]
pub fn string_ownership() {
    longest_owned(String::from("ABOBA"), String::from("ABOBUS"));
}

// BORROWING
// ================================================================================================

// ----- 2 --------------------------------------
// Write a function `print_length(s: ???)` that takes some string and prints its length without
// taking ownership. First use it with some random (censored) string, and then print this string to
// show that it was not moved and still available.
//
// You can implement the function and use it right inside the `simple_borrowing` function.
fn print_length(s: &str) {
    println!("{}", s.len())
}
#[allow(dead_code)]
pub fn simple_borrowing() {
    let s = String::from("censored");
    print_length(&s);
    println!("{s}");
}

// ----- 3 --------------------------------------
// Implement a function `append_and_return_length(string: ???, suffix: ???) -> usize` that borrows
// some string, appends a suffix to it, and returns the new length. Then call it multiple times
// to check that the string was borrowed, not moved.
//
// You can implement the function and use it right inside the `hard_borrowing` function.
fn append_and_return_length(string: &mut String, suffix: &str) -> usize {
    string.push_str(suffix);
    string.len()
}
#[allow(dead_code)]
pub fn hard_borrowing() {
    let mut s = String::from("ABO");
    let suff = String::from("BA");
    println!("{}", append_and_return_length(&mut s, &suff));
    println!("{}", append_and_return_length(&mut s, &suff));
    println!("{}", append_and_return_length(&mut s, &suff));
    println!("{}", append_and_return_length(&mut s, &suff));
}

// SLICES
// ================================================================================================

// ----- 4 --------------------------------------
// Write a function last_word(s: &str) -> &str that returns the last word from a string slice.
// Assume words are separated by spaces.
pub fn last_word(slice: &str) -> &str {
    slice.split_whitespace().last().unwrap_or(slice)
}

// ----- 5 --------------------------------------
// Write a function longest_word(sentence: &str) -> &str that returns the longest word in a
// sentence (string slice). If several words have the same maximum length, return the last one.
pub fn longest_word(sentence: &str) -> &str {
    sentence
        .split_whitespace()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap_or(sentence)
}
