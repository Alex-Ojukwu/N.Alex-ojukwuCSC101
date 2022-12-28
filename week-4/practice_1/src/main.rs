fn main() {
    let name = "aisha lawal";
    let uni:&str = "pan-atlantic university";
    let addr:&str = "km 52 lekki-expressway,igbeju-lekki";
    println!("name: {}",name);
    println! ("university name: {}, address: {}",uni,addr);


    let department:&'static str = "computer science";
    let school:&'static str="computer science";
    println! ("department:{}, \nschool:{}",department,school);
}
