#[derive(Debug)]
enum MyIpAddrKind
{
    V4,
    V6,
}

#[derive(Debug)]
enum MyIpAddrKindWithStringRepresentation
{
    V4(u8, u8, u8, u8),
    V6(String),
}

impl MyIpAddrKindWithStringRepresentation
{
    fn call(&self)
    {
        println!("called");
    }
}

#[derive(Debug)]
enum Message
{
    Quit,
    Move {x: i32, y: i32},
    Write (String),
    ChangeColor (i32, i32, i32),
}

#[derive(Debug)]
enum UsState
{
    Alabama,
    Alaska,
    // ---
}

#[derive(Debug)]
enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: &Coin) -> u8
{
    return match coin
    {
        Coin::Penny             =>      1,
        Coin::Nickel            =>      5,
        Coin::Dime              =>      10,
        // Example of binding in match
        Coin::Quarter(state)    => 
                                    {
                                        println!("State quarter from {:?}", state);
                                        25
                                    }
    }
}

/* 
 * No need to define this enum as it is already defined
 * by the standard library
 *
enum Option<T>
{
    Some(T),
    None,
}
*/

// Handing options in match
fn plus_one(x: Option<i32>) -> Option<i32>
{
    return match x
    {
        None                =>  None,
        Some(i)             =>  Some(i+1),
    }
}

fn main() {
    let four = MyIpAddrKind::V4;
    let six = MyIpAddrKind::V6;


    let four = MyIpAddrKindWithStringRepresentation::V4(1, 2, 3, 4);
    let six = MyIpAddrKindWithStringRepresentation::V6(String::from("ff:ff:ff:ff:ff:ff"));
    six.call();
    println!("Fouris {:?} and Six is {:?}", four, six);

    let s = Message::Move {x: 15, y: 16};
    println!("The value of s is {:?}", s);

    /* 
     * Rust doesn't have a NULL in the C++ sense of the word
     * But the concept can still be used with options
     */
     let some_number = Some(5);
     let some_string = Some(String::from("Hello"));
     // If we use None, we need to tell the type, because
     // the compiler can't infer the type as in previous cases
     let absent_number: std::option::Option<i32> = None;

     let c = Coin::Quarter(UsState::Alabama);
     println!("value of {:?} is {}", c, value_in_cents(&c));
     let five = Some(5);
     let six = plus_one(five);
     let none = plus_one(None);
     println!("plus_one({:?}) = {:?} . None is {:?}", five, six, none);

     // pattern matches must be exhaustive, but we can use
     // _ to the rescue (acts as the default case)
     let some_u8_value = 0u8;
     match some_u8_value
     {
         0      =>  println!("zero"),
         1      =>  println!("One"),
         _      =>  println!("Something else"),
     }


     // the if-let saves us some typing. If we wanted to handle it
     // using the regular match syntax, we will have to 
     // add a _ case.
     let some_u8_value = Some(3u8);
     if let Some(3) = some_u8_value
     {
         println!("-- The value is 3");
     };
}
