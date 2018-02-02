extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    loop{
        println!("Number of die: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input dice");
        let values: Vec<i32>;
        if input.contains("d") {
            values = input.trim().split("d").map({|x| x.parse().unwrap()}).collect();
        } else {
            values = [input.trim().parse().expect("Not a number"),6].to_vec()
        }
        let mut dice:Vec<i32> = Vec::new();

        for _ in 0..values[0] {
            dice.push(rand::thread_rng().gen_range(1,values[1]+1))
        }
        let mut over5 = 0; 
        let mut sum = 0;
        
        for i in &dice {
            if *i > 4 {
                over5 += 1
            }
            sum += i;
        }
        println!("\n{:?}", dice);
        println!("Sum: {}", sum);
        println!("5 or greater: {}\n", over5);
    }

    


}
