fn takes_ownership(s: String)
{
    println!("the parameter is {}", s);
}

fn takes_and_returns_ownership(s: String) -> String
{
    println!("The parameter is {}", s);
    return s;
}

fn take_and_return_multiple(s1: String, s2: String) -> (String, String)
{
    return (s1, s2);
}

fn main() {
    let x = 5;
    let mut y = x;

    println!("The value of x is {} and y is {}", x, y);
    y = y + 1;
    println!("The value of x is {} and y is {}", x, y);

    println!("");

    let s1 = String::from("Hello");
    let mut s2 = s1;
    // The following will error out since s1 is no longer valid
    //println!("The value of s1 is {} and s2 is {}", s1, s2);
    s2.push_str(", world!");
    //println!("The value of s1 is {} and s2 is {}", s1, s2);

    takes_ownership(s2);
    // println!("Can I access s2 after it was passed as a paramter? {}", s2);
    // s2 is no longer valid as it was passed to take_ownership

    let mut s2 = String::from("Hello");
    // The proper way to do it is to return ownership
    s2 = takes_and_returns_ownership(s2);
    let s2 = takes_and_returns_ownership(s2);
    println!("s2 is still valid? {}", s2);

    //check if copy is implemented for string literals
    let s1 = "Hello";
    let s2 = s1;
    println!("The value of s1 si {} and s2 is {}", s1, s2);


    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let (s1, s2) = take_and_return_multiple(s1, s2);
}
