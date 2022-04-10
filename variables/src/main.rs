fn main() {

    // Variables and Mutability

    let mut x = 6;
    println!("The value of x is: {}", x);

    x = 5;
    println!("The value of x is: {}", x);

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    println!("Three hours in seconds: {} second.", THREE_HOURS_IN_SECONDS);

    // Shadowing
    let y = 2;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);

    // let spacing = "     ";
    // let spacing.len();
    // println!(spacing);
}
