use std::io;

fn main() {
     let sequence=[1,1,2,3,5,8,13,21,34,55,89,144,233,377];
     let mut index= String::new();
     println!("Please enter an n value between 0 and 13 to print the nth Fibonacci number");
     io::stdin().read_line(&mut index).expect("failed to read line");
     let index: usize =index.trim().parse().expect("Index entered was not a number");
     let element=sequence[index];
     println!("The value of the {index}th Fibonacci number is {element}");

}
