// Collections: vectors, strings, hashmaps and sets

fn vectors() {
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

fn main()
{
    vectors();
}