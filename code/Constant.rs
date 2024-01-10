fn main()
{
    // we cant use mut with const
    // **we cant use small letters for const

    const SECONDS_PER_MINUTE: u32 = 60; // we have to provide a type for the const
    println!("{} seconds in a minute", SECONDS_PER_MINUTE);

    const MINUTES_PER_HOUR: i32 = 60;
    println!("{} minutes in an hour", MINUTES_PER_HOUR);

    const HOURS_PER_DAY: f32 = 24.0;
    println!("{} hours in a day", HOURS_PER_DAY);

    const DAYS_PER_WEEK: f64 = 7.0;
    println!("{} days in a week", DAYS_PER_WEEK);

    const WEEKS_PER_YEAR: u64 = 52;
    println!("{} weeks in a year", WEEKS_PER_YEAR);

    const USER_LIMIT:i32 = 100;
    const PI:f32 = 3.14;
     
    println!("user limit is {}",USER_LIMIT);
    println!("pi value is {}",PI);  
}
