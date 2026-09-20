use std::io;
fn main() {
    println!("enter value for a:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let a:f64=input1.trim().parse().expect("not a valid float");

    println!("\nenter value for b:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let b:f64=input2.trim().parse().expect("not a valid float");

       println!("\nenter value for c:");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("not a valid string");
    let c:f64=input3.trim().parse().expect("not a valid float");

    let d:f64 = b *b -4.0* a*c;
    if d > 0.0{
    let root1 =(-b + d.sqrt() ) /(2.0 * a);
    let root2 =(-b - d.sqrt()) / (2.0 * a);
    println!("2 roots {} and {}",root1,root2 );
}
else if d ==  0.0{
    let root3 = -b /(2.0*a);
    println!("exactly one real root: {}", root3);
}
else{

    println!("no real roots");
}



}
