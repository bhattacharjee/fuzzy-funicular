

fn another_function(x:i32, y:i32)
{
    println!("Another function!");
    println!("The value of x is {} and y is {}", x, y);
}

fn function_with_retval(x:i32) -> i32
{
    return x + 1
}

fn main() {
    println!("Hello, world!");
    another_function(5, 6);

    let y = 
        {
            let x = 3;
            x + 1
        };
    
    println!("The value of y is {}", y);

    println!("function_with_retval(5) returned {}", function_with_retval(5));
}
