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

fn first_word_slice(s: &String) -> &str
{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate()
    {
        if b' ' == item
        {
            return &s[0..i];
        }
    }
    return &s[..];
}

fn first_word_slice_v2(s: &str) -> &str
{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate()
    {
        if b' ' == item
        {
            return &s[0..i];
        }
    }
    return &s[..];
}

fn main() {
    let s = String::from("Hello world");
    let position = first_word(&s);
    println!("The first word of {} ends at {}", s, position);

    println!("The first word of {} is {}", s, first_word_slice(&s));

    println!("This also works {}", first_word_slice_v2(&s[..]));

    let arr = [1, 2, 3, 4, 5];
    let arr_slice = &arr[1..3]; // Type of this is &[i32]

    for (i, &item) in arr_slice.iter().enumerate()
    {
        println!("{} - {}", i, item);
    }

    assert_eq!(arr_slice, &[2, 3]);
}
