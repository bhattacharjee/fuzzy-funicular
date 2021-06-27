fn first_word(s: &String) -> usize
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return i;
        }
    }

    return s.len();
}

fn main() {
    let s = String::from("Hello world");
    let position = first_word(&s);
    println!("The first word of {} ends at {}", s, position);
}
