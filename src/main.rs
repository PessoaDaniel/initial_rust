/**
 * Rust conventions:
 * function names uses snake_case
 * Constants names uses CAPITALIZED_SNAKE_CASE
 *
 */
/** Global scope constant */
const MINIMUN_USER_AGE_ALLOWED:i32 = 18;
fn main() {
    println!("Hello World!");
    immutable_variables();
    mutable_variables();
    constant_value();
    shadowing_values();
}

fn immutable_variables() {
    /**
     * By default Rust  variables are immutable, its values doesn't changes
     * along code execution
     */
    println!("[Immutable value]");
    let x = 5;
    let x = 2;
    println!("The value of x is: {x}");
    // x = 6;
    println!("The value of x is: {x}");
}

fn mutable_variables() {
    println!("[Mutable value]");
    /** To allow value changes use the mut keyword after let declaration */
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn constant_value() {
    /** Local constant scope*/
    const USER_AGE:i32 = 15;
    println!("The user age is {USER_AGE}, the minimum age to use this system is: {MINIMUN_USER_AGE_ALLOWED}");
}
fn shadowing_values() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x out of the scope s is: {x}");
}