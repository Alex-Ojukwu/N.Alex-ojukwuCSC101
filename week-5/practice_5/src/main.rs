//rust program to read the height of a program 
//and then print if person is tall, dwraf,
// or average height person

use std::io;




fn main() {
    let mut input = String::new();

    println!(" enter your height ( in centimeters)");
    io::stdin().read_line(&mut input).expect("not a valid string");
    let height:f32 = input.trim().parse().expect("not a valid number");

    if height >= 150.0 && height <= 170.0{println!("you are average height")}
    else if height > 170.0  &&  height <= 195.0
    {println!("you're a tall ass nigga")}
    else if height < 150.0 &&  height >100.0{println!("you're short asf nigga")}
    else {println!("abnormal height");}

}
