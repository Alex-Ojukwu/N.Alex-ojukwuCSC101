use std::io;

fn checker() {
    let mut input = String::new();
    println!("enter a character:");
    io::stdin().read_line(&mut input).expect("failed to read input");
    let ch:char = input.trim().parse().expect("invalid input");
}

    if ch >= '0' && ch <= '9'
    {
        println!("character '{}' is a digit",ch);
    }
    else 
        {
            println!("Character '{}' is not a digit",ch);
        }
    
    fn main(){
        //calling function
        println!("welcome! this program checks whether a character variable
            contains a digit or not");
        checker()
    
}
