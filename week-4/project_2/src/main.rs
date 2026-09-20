use std::io;
fn main() {
    println!("is the employee experienced?(true/false)");
    let mut expin =String::new();
    io::stdin().read_line(&mut expin).expect("not a valid answer");
    let exp:bool =expin.trim().parse().expect("mot a valid boolean");

    println!("how old is the employee");
    let mut agein =String::new();
    io::stdin().read_line(&mut agein).expect("not a valid number");
    let age:u8=agein.trim().parse().expect("not a valid age");


if exp == true{
    if age  >= 39{
        println!("employee anual incentive is N1,560,000 ");
}
if age >30 && age<39{
println!("employee anual incentive is N1,480,000 ");
}
if age <=29{ //question says 28 but wont make sense
    println!("employee anual incentive is N1,300,000 ");


}

    }
    else{println!("employee anual incentivie is N100,000");}

}
