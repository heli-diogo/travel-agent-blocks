fn main() {
  let a: &i32;
  {
    // b lifetime is not same as a
    let b = 3; 
    a = &b;
    println!("{}",b);
  }  
// dscghdftw_sgh_dfhdf
  println!("{}",a);
}
