use std::io;
use rand::Rng;
fn main()
{

    loop
    {
        println!("Guess the number!");

        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse()
            {
                Ok(num)     => num,
                Err(_)          => continue,
            };

        println!("You guessed {}", guess);

        let secret_number = rand::thread_rng()
            .gen_range(1..10);

        println!("The secret number is {}", secret_number);

        match guess.cmp(&secret_number)
        {
            std::cmp::Ordering::Equal   =>  {
                                            println!("You win");
                                            break;
                                            }
            std::cmp::Ordering::Less    =>  println!("Too small"),
            std::cmp::Ordering::Greater =>  println!("Too big")
        }
    }
}
