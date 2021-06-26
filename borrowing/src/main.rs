fn calculate_length(s: &String) -> usize
{
    return s.len();
}

fn change_string(s: &mut String)
{
    s.push_str(", changed");
}
fn main() {
    let mut s1 = String::from("Hello");

    /* Calculate the length using a reference pass to a function */
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);

    /* Store the reference in another variable */
    let s2: &String = &s1;
    println!("s2 is {}", s2);


    /* To be able to change the function we need to pass a mutable reference */
    change_string(&mut s1);
    println!("The value of s1 after changing is {}", s1);


    let s3 = String::from("Hello");
    /* There can only be one mutable reference */
    /*
     * let r1 = &mut s3;
     *
     * This does not work because we cannot borrow a non-mutable
     * variable as a mutable reference
     */

    let mut s4 = String::from("Hello");
    let r4 = &mut s4;
    /*
     * Creating another mutable reference will cause a compilation error
     * let r2 = &mut s3;
     * This is restrictive, but it helps prevent data races
     */
    //let r5 = &s4;
    

}
