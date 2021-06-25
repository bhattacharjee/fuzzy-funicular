fn calculate_length(s: &String) -> usize
{
    return s.len();
}
fn main() {
    let s1 = String::from("Hello");

    /* Calculate the length using a reference pass to a function */
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);

    /* Store the reference in another variable */
    let s2: &String = &s1;
    println!("s2 is {}", s2);
}
