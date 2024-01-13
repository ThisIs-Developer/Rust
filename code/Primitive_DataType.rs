fn main()
{
    // Scalar Types
    let x : i8 = 5; // 8-bit signed integer
    let y : i16 = 10; // 16-bit signed integer
    let z : i32 = 15; // 32-bit signed integer
    let w : i64 = 20; // 64-bit signed integer
    let v : i128 = 25; // 128-bit signed integer
    let u : isize = 30; // signed integer of the size of a pointer

    let a : u8 = 2; // 8-bit unsigned integer
    let b : u16 = 4; // 16-bit unsigned integer
    let c : u32 = 6; // 32-bit unsigned integer
    let d : u64 = 8; // 64-bit unsigned integer
    let e : u128 = 10; // 128-bit unsigned integer
    let f : usize = 12; // unsigned integer of the size of a pointer
    
    println!("Signed Integer: {} {} {} {} {} {}", x, y, z, w, v, u);
    println!("Unsigned Integer: {} {} {} {} {} {}", a, b, c, d, e, f);

    let g : f32 = 3.141592653589793238462643383279502884197169399375105820974944592307816406286; // 32-bit floating point number
    let h : f64 = 3.141592653589793238462643383279502884197169399375105820974944592307816406286; // 64-bit floating point number

    println!("Floating Point f32: {}", g);
    println!("Floating Point f64: {}", h);

    let i : bool = true; // boolean
    let j : char = 'a'; // character

    println!("Boolean: {}", i);
    println!("Character: {}", j);

    // Compound Types
    let m : [i32; 5] = [1, 2, 3, 4, 5]; // array of 5 32-bit signed integers
    let x: (i32, f64, u8, char) = (500, 6.4, 1, 'a'); // tuple

    println!("Array: {:?}", m);
    println!("Tuple: {:?}", x);

}