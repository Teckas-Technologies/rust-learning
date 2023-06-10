pub(crate) fn function_examples() {
    function_statement();
    println!("Printing function return value {}",function_with_return_value());
    println!("Printing value passed {}",function_with_parameters(5))
}

fn function_statement() {
    println!("Printing Something")
}

fn function_with_return_value() -> i32 {
    388
}

fn function_with_parameters(x : i32) -> i32 {
    x+1
}