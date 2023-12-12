use std::io;

fn main() {
loop{
    println!{"Please input a temperature in Fahrenheit"};
         let mut fahrenheit = String::new();
         io::stdin().read_line(& mut fahrenheit).expect("Failed to read line");
         let fahrenheit: i32 = fahrenheit.trim().parse().expect("Please type a number");

    let celsius = ({fahrenheit}-32)*5/9;
    
    println!("{}°F is {}°C", fahrenheit, celsius);
     }
} 
