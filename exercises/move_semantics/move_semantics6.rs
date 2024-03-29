// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.


// To find the answer, you can consult the book section "References and Borrowing":
// https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html
// The first problem is that `get_char` is taking ownership of the string.
// So `data` is moved and can't be used for `string_uppercase`
// `data` is moved to `get_char` first, meaning that `string_uppercase` cannot manipulate the data.
// Once you've fixed that, `string_uppercase`'s function signature will also need to be adjusted.
// Can you figure out how?

fn main() {
    let data = "Rust is great!".to_string();

    println!("{}", get_char(&data));

    string_uppercase(&data);

    println!("{}", data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &String) {
    let new_data = data.to_uppercase();

    println!("{}", new_data);
}