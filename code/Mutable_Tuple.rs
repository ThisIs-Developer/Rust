fn main()
{
    let mut tup: (i32, f64, u8, char) = (500, 6.4, 1, 'a'); // tuple
    tup.0 = 501;
    println!("Tuple: {:?}", tup);
}