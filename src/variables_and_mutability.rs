pub(crate) fn variables_example() {

    // Mutable Variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Immutable Variables
    let y = 100;
    println!("The value of y is: {y}");
    let y = y *100;
    println!("The value of y after shadowing is: {y}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Constant Example {THREE_HOURS_IN_SECONDS}")

}