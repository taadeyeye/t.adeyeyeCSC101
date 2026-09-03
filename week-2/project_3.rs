fn  main() {
   let p:f64 =210_000.0;
    let n:f64 =3.00;
     let r:f64 =5.00;
     

     //deprecation
     let a= p*(1.0 -(r/100.0)).powf(n);
     println!("amount is {} naira", a);
     let de = a - p;
     println!("deprecation is {} naira", de );
}