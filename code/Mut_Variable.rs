fn main()
{
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6; // error: mismatched types //x is immutable
    // println!("The value of x is: {}", x);

    //lets fix it it by using mut
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
