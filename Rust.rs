fn main(){
   let mut line1 = String::new();
   std::io::stdin().read_line(&mut line1).unwrap();
   let num1: i32 = line1.trim().parse().ok().expect("Try");
   let mut line2 = String::new();
   std::io::stdin().read_line(&mut line2).unwrap();
   let num2: i32 = line2.trim().parse().ok().expect("Try");

   println!("PROD = {}", num1 * num2)
}
