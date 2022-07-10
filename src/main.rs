// Steffano Martinez | Chapter 3 | Rev1.0 // 
use std::io;

fn main() {
    println!("Hello, world!, what sequence of fibo you want?; ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to get an input");
    let choice: u32 = choice.trim().parse().expect("Failed to get a number");
    fibotime(choice);
}

fn fibotime(x: u32) {
    //Im passing the n in the parameter of the function
    let mut a = 0; // X(n-1)
    let mut b = 1; // X(n-2)
    let mut c = 2; // This will be the control variable (set at 2 since this is the start)
    let mut sum = 0; // this would be the updated Xn
    while c <= x
    {
        sum = a + b;
        a = b;
        b = sum;
        c = c + 1;
    }
    println!("The sum is {sum} for the {x}th term");

}