fn main() {
    let name = "tanitoluwa adeyeye";
    let uni:&str = "Pan atlantic university";
    let addr:&str ="Km 52,Lekki-Epe Expressway, Ibeju-Lekki, Lagos, Nigeria";
    println!("Name:{}",name );
    println!("university:{}, \nadresss:  {}" ,uni,addr);


    let department:&'static str ="computer science";
    let school:&'static str = "school of science and technology";
    println!("department:{},\nschool:{}",department,school);




}
