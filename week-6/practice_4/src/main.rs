fn main(){

    

    let fullname = "Tanitoluwa adeyeye";
    let department = "data science";
    let uni = "Pan atlantic niversity";

    let mut school = "school of science".to_string();
    //push string
    school.push_str("and technology");

    println!("my name is: {}", fullname);
    //check length
    println!("the length my fullname is: {}",fullname.len());
    println!("i am a student of {} department", department);
    println!("{}",school);
    println!("{}",uni)


}