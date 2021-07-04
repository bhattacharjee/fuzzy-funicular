#[allow(unused_variables)]
use std::fs::File;
use std::io::ErrorKind;


fn error_handling_using_match() {
    println!("--------------------------------------------------------------------------");
    println!("                      ERROR HANDLING USING MATCH");
    println!("--------------------------------------------------------------------------");
    let f = File::open("Hello.txt");
    let f = match f
    {
        Ok(f)       =>      f,  // assign the handle to f
        Err(e)      =>
        {
            println!("Returning from here  because of error {}", e);
            return
        }       
    };

    // Control will not reach here, comment the previous block to reach here
    println!("Continuing with a more complicated arrangement of match");

    // A more complicated example where we do different things ased on
    // what kind of error we encountered
    let f = File::open("Hello.txt");
    let f = match f
    {
    	Ok(f)		=>		f,
    	Err(e)		=>		match e.kind()
    	{
            ErrorKind::NotFound     => match File::create("Hello.txt")
            {
                Ok(fc)      =>      fc,
                Err(e)      =>      panic!("Error creating Hello.txt, err = {}", e)
            }
            other_error             =>
            {
                panic!("Error opening Hello.txt for read, err = {:?}", other_error)
            }
    	}
    };
}

fn main()
{
    // One way to deal with errors is to use match statements
    error_handling_using_match();

    // But match statements are too clumsy and there are a whole lot of them
    // There is a cleaner way to deal with errors
}
