use std::io;

fn main() {

    let mut input2 = String::new();
    println!(" enter base number:");
    io::stdin().read_line(&mut input2).expect("failed to read input ");
    let a:f64 =input2 .trim().parse().expect("failed to read");

    let mut input3 =String::new();
    println!("enter height number:");
    io::stdin().read_line(&mut input3).expect("failed to read input");
    let b:f64 =input3.trim().parse().expect("failed to read");

    let area:f64= (a*b)/2.0 ;
    println!("the area of the whatever shape is: {}",area);
}
