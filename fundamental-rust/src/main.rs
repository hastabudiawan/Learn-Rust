// Example of a function expression and macro println! in Rust
fn main() {
    println!("Hello, rust!");
    print!("Welcome to the world of Rust programming!")
}

// Example of a statement expression in Rust
#[test]
fn statement_expression() {
    let x = 5;
    let y = tambah_lima(10);

    print!("x = {}, y = {}", x, y);
}

#[cfg(test)]
fn tambah_lima(x: i32) -> i32 {
    x + 5
}

// Example Of a block expression in Rust
#[test]
fn block_expression() {
    let a = {
        let b = 10;
        b + 5
    };
    print!("a = {}", a)
}

// Example Of a Naming Convention in Rust
#[test]
fn naming_convention() {
    let first_name = "John";
    let last_name = "Doe";
    print!("{} {}", first_name, last_name);
}