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

fn main() {
    let four = MyIpAddrKind::V4;
    let six = MyIpAddrKind::V6;


    let four = MyIpAddrKindWithStringRepresentation::V4(1, 2, 3, 4);
    let six = MyIpAddrKindWithStringRepresentation::V6(String::from("ff:ff:ff:ff:ff:ff"));
    six.call();
    println!("Fouris {:?} and Six is {:?}", four, six);
}
