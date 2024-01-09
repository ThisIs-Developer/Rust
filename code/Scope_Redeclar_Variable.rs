fn main()
{
    let x = 2;
    println!("The value of x is: {}", x);

    //let's try to redeclare x in a new scope
    {
        let x = 1;
        println!("The value of x is: {}", x);
    }

    //let's try to redeclare x in a new scope
    {
        let x = "ThisIs-Developer";
        println!("The value of x is: {}", x);
    }

    let x = x+1;
    println!("The value of x is: {}", x);

    //let's use the x from the exterior scope in the interior scope
    {
        let x = x+1;
        println!("The value of x is: {}", x);
    }

    let x = x;
    println!("The value of x is: {}", x);
    
}
