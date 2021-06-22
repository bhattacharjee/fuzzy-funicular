fn main() {

    let number = 3;

    // if-else are expressions so can return a value 
    // from them
    let y = if number > 5
    {
        println!("Condition was true");
        number + 3
    }
    else
    {
        println!("Condition was true");
        number + 4
    }; // Notice the semicolon here
    println!("y is {}", y);


    // Another way is to convert them to expressions
    if number > 5
    {
        println!("Condition was true");
    }
    else
    {
        println!("Condition was false");
    } // No semicolon required here


    // Since it is an expression, we can use an if in a let
    let z = if true {5} else {6};
    println!("Value of z is {}", z);
}
