fn main() {
    println!("Hello, world!");

    // infinite loop
    let mut x :usize = 10;
    let result = loop
    {
        println!("The value of x is {}", x);
        x = x - 1;

        if 0 == x
        {
            break x + 50;   // will work without this semicolon as well
        }
    };
    println!("The value of result is {}", result);

    // Conditional loops with while
    x = 10;
    while x > 0
    {
        println!("The value of x is {}", x);
        x = x - 1;
        if x == 5
        {
            // break x * 10; // This syntax is invalid in while loop
            break;
        }
    }
    println!("The value of result is {}", x);


    // Use for loops for collections
    let a = [1, 2, 3, 4, 5];
    for element in a.iter()
    {
        println!("The value of element is {}", element);
    }

    for element in (0..5).rev()
    {
        println!("The value of element is {}", element);
    }
}
