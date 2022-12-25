fn main() {
	let p:f64 = 520000000.0;
	let r:f64 = 10.0;
	let t:f64 = 5.0;

	//compound interest
	let  a = p*( 1.0+(r/100.0)) * t;
	let ci :f64 = a-p;
	println!("compound interest is {}", ci);
}