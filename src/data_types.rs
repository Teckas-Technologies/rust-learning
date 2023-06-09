pub(crate) fn data_type_examples() {

    // Integer
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Integer Value: {guess}");

    // Tuple
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    println!("Printing 500 from tuple {five_hundred}");

    // Arrays
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("First {first}, Second {second}");
}