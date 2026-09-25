fn main() {
    let x = 5; // This is immutable by default.
    let mut y = 6; // This is set to mutable.


    println!("The value of x is: {x}"); // x = 5
    println!("The value of y is: {y}"); // y = 6

    // This assignment won't work because x is immutable.
    // x = 6;
    // This assignment works because y is mutable.
    y = 10;

    println!("The value of x is: {x}"); // x = 5
    println!("The value of y is: {y}"); // y = 10

    // This declaration throws "missing type for `const` item"
    // const THREE_HOURS_IN_SECONDS = 60 * 60 * 3;
    // Constants must have type defined at declaration.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours = {THREE_HOURS_IN_SECONDS} seconds");

    // Below example shows how Rust manage variables in different scope.
    let x = x + 1; // This is called shadowing, where a previously declared variable is declared again. New declaration will "shadow" old declaration.
    {
        let x = x * 2;
        println!("The value of inner x is: {x}"); // x = 12
    }
    println!("The value of outer x is: {x}"); // x = 6

    // This is another example of shadowing, where the type of "spaces" variable is overriden.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Length: {spaces}"); // spaces = 3

    // Shadowing creates a new variable then throws the old variable. Not to be confused with mutable types, where type overriding is not allowed.
    let mut spaces = "   "; // Here spaces is shadowed to a mutable type.
    // This assignment won't work because it changes the type of "spaces" variable.
    // spaces = spaces.len();
    let length = spaces.len();
    println!("Length: {length}"); // length = 3
}
