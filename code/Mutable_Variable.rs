fn main()
{
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6; // error: mismatched types //x is immutable
    // println!("The value of x is: {}", x);

    //lets fix it
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //If we change the type of x although it is mutable
    //let mut x = 5;
    //println!("The value of x is: {}", x);
    //x = "ThisIs-Developer"; // error: mismatched types //x is immutable
    //println!("The value of x is: {}", x);
}
