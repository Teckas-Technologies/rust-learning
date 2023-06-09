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
}