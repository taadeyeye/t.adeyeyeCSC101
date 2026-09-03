fn  main() {
	let t :f64= 450_000.00;
	let m :f64= 1_500_000.00;
	let h :f64= 750_000.00;
	let d :f64= 2_850_000.0;
	let a :f64= 250_000.00;
	//calcuating sum
	let to = (t*2.0)+m+(h*3.0)+(d*3.0)+a;
	println!("sum is {}", to);
	//calculating average
    let n = 2.0+1.0+3.0+3.0+1.0;
	let av = to/n;
	println!("average is {}",av );
}