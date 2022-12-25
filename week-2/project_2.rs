fn main() {
	let t:f64 = 450000.0;
	let m:f64 = 1500000.0;
	let h:f64 = 750000.0;
	let d:f64 = 2850000.0;
	let a:f64 = 250000.0;

	//sum and average of sales
	let sum = t+m+h+d+a;
	let avg:f64 = (t+m+h+d+a)/5.0;
	println! ("The sum of sales is {} and the average is {}",sum,avg);

}