 fn main() {
   let p:f64 = 210000.0;
   let r:f64 = 5.0;
   let t:f64 = 3.0;

    //depreciation in compound interest
   let a:f64 = p*(1.0-(r/100.0))* t ;
   println! ("The value of depreciation is {}",a);

 }