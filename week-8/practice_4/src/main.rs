fn main() {
    //name vector
    let name = vec!["mary","sam","sally","greg","ade","mark","june","ife"];

    // age vector 
    let age = vec![16,17,19,22,20,21,18,23];

    println!("\n age allocation:\n");

    //loop to iterate elements in vector
    for i in 0..age.len()
    {
        // iterating through i on the vector
        println!("{} is {} years old\n",name[i],age[i]);
    }
}
