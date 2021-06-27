struct User
{
    username:       String,
    email:          String,
    sign_in_count:  u64,
    active:         bool
}

fn build_user(username: String, email: String) -> User
{
    let user = User {
        username:       username,
        email:          email,
        sign_in_count:  0,
        active:         true
    };

    return user;
}

fn build_user_2(username: String, email: String) -> User
{
    // Can omit field names
    User {
        username,
        email,
        sign_in_count:  0,
        active:         false
    }
}

fn printing_debug_data()
{
    // Inherit from Debug that implements the Debug() function
    #[derive(Debug)]
    struct Rect
    {
        width:          u32,
        height:         u32
    }

    let rect = Rect
    {
        width:          5,
        height:         5
    };

    println!("This print will work because of Debug, rect is {:?}", rect);

}

fn main() {
    println!("Hello, world!");

    let user1 = User {
        email:          String::from("rajbir"),
        username:       String::from("rajbir@example.com"),
        sign_in_count:  0,
        active:         true
    };

    let user2 = build_user(String::from("user2"), String::from("user2@example.com"));
    let mut user2 = build_user_2(String::from("user2"), String::from("user2@example.com"));

    user2.email = String::from("Some noew value");

    let user3 = User
    {
        email:          String::from("another"),
        username:       String::from("user@example.com"),
        ..user2         // Fill the rest from user2
    };

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    /*
     * This will fail because they are of different types
     * origin = black
     */

     printing_debug_data();
}
