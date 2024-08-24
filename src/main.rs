fn main() {
    println!("Hello World!");
    immutable_variables();
    mutable_variables();
}

fn immutable_variables() {
    println!("[Immutable value]");
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6;
    println!("The value of x is: {x}");
}

fn mutable_variables() {
    println!("[Mutable value]");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}