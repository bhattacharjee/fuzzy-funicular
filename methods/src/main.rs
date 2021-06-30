#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32
}

impl Rectangle
{
    fn area(&self) -> u32
    {
        return self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool
    {
        return self.width > other.width && self.height > other.height
    }

    fn get_square(width: u32) -> Rectangle
    {
        let r = Rectangle
        {
            width: width,
            height: width
        };

        return r;
    }
}

fn main() {
    let rect1 = Rectangle
    {
        width: 30,
        height: 40
    };

    let rect2 = Rectangle
    {
        width: 20,
        height: 20
    };
    println!("The area of rectangle is {}", rect1.area());
    println!("can be fully contained? {}", rect1.can_hold(&rect2));

    let sq = Rectangle::get_square(5);
    println!("The area of a square of size 5 is {}", sq.area());
}
