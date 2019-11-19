use std::io;
fn main() {
 let mut Num1 = String::new();
 io::stdin().read_line(&mut Num1).expect("");
 let Num1:i32 = Num1.trim().parse().unwrap();

 let mut Num2 = String::new();
 io::stdin().read_line(&mut Num2).expect("");
let Num2:i32 = Num2.trim().parse().unwrap();
      
println!("The number of sum is :{}",addition_of_number(Num1,Num2));
println!("The number of subtraction is :{}",subtraction_of_number(Num1,Num2));
println!("The number of multiple is :{}",multiplication_of_number(Num1,Num2));
println!("The number of division is :{}",division_of_number(Num1,Num2));



}

            //Q1//

fn addition_of_number(Num1:i32,Num2:i32)->i32{
    let mut sum:i32;
    sum = Num1+Num2;
    sum
}
                //for subraction//

fn subtraction_of_number(Num1:i32,Num2:i32)->i32{
    let mut sum:i32;
    sum = Num1-Num2;
    sum
}
                //for multiplication
fn multiplication_of_number(Num1:i32,Num2:i32)->i32{
    let mut sum:i32;
    sum = Num1*Num2;
    sum
}

fn division_of_number(Num1:i32,Num2:i32)->i32{
    let mut sum:i32;
    sum = Num1-Num2;
    sum
}