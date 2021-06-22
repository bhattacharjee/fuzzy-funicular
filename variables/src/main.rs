use std::io;

fn main()
{
    let mut x = 5;
    println!("Value of x is {}", x);

    x = 7;
    println!("Value of x is {}", x);

    let x:u32 = match "42".parse()
                {
                    Ok(num)     =>      num,
                    Err(_)          =>      u32::MAX
                };

    println!("value of x is {}", x);

    // Tuples
    let xx = (1, 2, 3);
    let (x1, x2, x3) = xx;
    println!("The values are {} {} {}", x1, x2, x3);
   
    let x4: (i32, i64, f64) = (1, 4, 3.);
    let (x1, x2, x3) = x4;
    println!("The values are {} {} {}", x1, x2, x3);

    // Arrays
    let arr = ["January", "February", "March", "April",
                        "May", "June", "July", "August", "September",
                        "October", "November", "December"];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    let first = arr[0];
    println!("The first element is {}", first);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter the index");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Could not read input");
    let index:usize = index
        .trim()
        .parse()
        .expect("Could not parse");

    println!("The value at {} is {}", index, a[index]);

}
