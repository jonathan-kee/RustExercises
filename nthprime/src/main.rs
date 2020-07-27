use std::io;

fn main() {
    loop {
        println!("Enter a number!");
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        println!("Your number: {}", number);

        let number: u32 = match number.trim().parse() {
            //page 28, Handling Invalid Input
            Ok(num) => num, //If parse is success, return Ok value with resulting number
            Err(_) => continue,
        };

        println!("Your number: {}", nth(number) );
        
    
    }
}

pub fn nth(n: u32) -> Option<u32>{
    if n <= 0{
        return None;
    }

    let mut primes: Vec<u32> = vec![];
    let index = n as usize;
    let mut i = 2;

    loop{
        if is_prime(i){
            primes.push(i);
        }

        if primes.len() >= index {
            break;
        }

        i = i + 1;
    }
    
    let maybe_val = primes.get(index - 1);

    match maybe_val {
        Some(val) => return Some(*val),
        _ => return None,
    }
}

fn is_prime(n: u32) -> bool{
    for i in 2..n{
        if n % i == 0 {
            return false;
        }
    }

    return n != 1;
}

