fn main() {

    let b:(i32,bool,f64) = (30,true,4.9);
    println!(b);
}
fn print (x:(i32,bool,f64)) {
    println!("inside print method");
    //assigns a tuple to distinct variables
    let (age,is_male,cgpa) = x;
    println!("age is  {} , ismale? {}, cgpa is {}",age,is_male,cgpa);
    
}
