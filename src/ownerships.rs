pub(crate) fn ownerships_example() {
    let s = "hello";
    {
        let s = "world";
        println!("Value of s inside : {}",s);
    }
    println!("Value of s outside : {}",s);

    // Append String
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    // Clone the value from a variable
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Providing ownership to the function
    let s3 = String::from("World");
    getting_ownership(s3);
    // S3 is no logger available (went out of scope)


    let s4 = String::from("Hi");
    let s5 = getting_and_providing_ownership(s4);
    println!("Value of s5: {}",s5);
    // Here s4 is no longer available
    // s5 got the scope from function



    // Instead of setting out of scope
    let s6 = String::from("Hi");
    getting_reference(&s6);
    // Here s6 is still have the scope Because we sent only the reference not the scope
    println!("S6 value after calling the function {}",s6);

    let mut s7 = String::from("Hello");
    println!("Before sending s7: {}",s7);
    getting_reference_and_changing_value(&mut s7);
    println!("After sending s7: {}",s7);

}

fn getting_ownership(s: String) {
    println!("Getting ownership of s {}",s);
}

fn getting_and_providing_ownership(s: String) -> String {
    println!("Getting and providing ownership");
    s
}

fn getting_reference(s: &String) {
    println!("Getting reference of s {}",s);
}

fn getting_reference_and_changing_value(s: &mut String) {
    s.push_str(" World")
}