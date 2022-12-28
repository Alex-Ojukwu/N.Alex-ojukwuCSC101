fn main() {

    let fullname = "chidubem john umeh";
    let departtment = "computer science";
    let uni = "pan atlantic";


    let mut school = "school of science".to_string();
    //push sftring
    school.push_str(" and technology");

    println!("my name is {}",fullname);
    //check length
    println! ("the length of my name is : {} \n i am a student of {} ",fullname.len(),departtment);
    println! ("{}", school);
    println! ("{}", uni);

}
