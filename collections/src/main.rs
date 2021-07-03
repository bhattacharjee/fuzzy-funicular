// Collections: vectors, strings, hashmaps and sets

fn vectors() {
    println!("-------------------------------------------------------------------------");
    println!("                               VECTORS                                   ");
    println!("-------------------------------------------------------------------------");

    // Create a vector
    let v: Vec<i32> = Vec::new();

    // Another way to create a vector
    let v = vec![1, 2, 3, 4, 5];

    // Update a vector
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    // Dropping a vector
    {
        let v1 = vec![1,2, 3, 4, 5];
        // do something with v1
    } // v1 is automatically dropped here

    //////////////////////////////////////////////////////////////////////////////////
    // Reading a vector
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The value of third element is {}", third);

    match v.get(2)
    {
        Some(third) => println!("!!The tird element is {}", third),
        None        => println!("There isn't a third element"),
    }

    match v.get(100)
    {
        Some(third) => println!("!!The hundredth element is {}", third),
        None        => println!("There isn't a 100th element"),
    }
    //////////////////////////////////////////////////////////////////////////////////
    
    //////////////////////////////////////////////////////////////////////////////////
    // Iterating over a vector
    for i in &v
    {
        println!("element inside vector: {}", i);
    }
    for (i, &e) in v.iter().enumerate()
    {
        println!("{}th element is {}", i, e);
    }
    println!("");

    //////////////////////////////////////////////////////////////////////////////////
    // Use enum to store multiple types
    enum SpreadsheetCell
    {
        Int(i32),
        Float(f64),
        Text(String)
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(1.42),
        SpreadsheetCell::Text(String::from("blue"))
    ];
    for (i, &e) in v.iter().enumerate()
    {
        println!("{}th element is {}", i, e);
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////
// String functions
fn string_handling()
{
    println!("-------------------------------------------------------------------------");
    println!("                             STRINGS                                     ");
    println!("-------------------------------------------------------------------------");

    // create a string
    let mut s = String::from("Hello ");
    // modify it
    s.push_str(" world");
    println!("The pushed string is {}", s);

    let mut s = "Hello".to_string();
    let s2 = "bar";
    s.push_str(s2);
    println!("{}", s);
    println!("{}", s2); // This works because it is a slice and is passed as reference

    // Push a single character with push
    s.push('l');
    println!("after pushing a single character: {}", s);

    ////////////////////////////////////////////////////////////////////////////////////
    // String concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // Note that the second term must be a slice
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // The following line no longer works because s1 is no longer valid as it was moved
    // println!("{} = {} + {}", s1, s2, s3);

    // Concatenate multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    // s1 is no longer valid, but s2 and s3 are
    println!("After concatenating three strings, {} = - + {} + {}", s, s2, s3);
}

fn main()
{
    vectors();
    string_handling();
}