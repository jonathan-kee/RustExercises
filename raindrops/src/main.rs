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

        println!("Your number: {}", raindrops(number) );
        
    
    }
}

pub fn raindrops(number: u32) -> String{
    let mut str_vec = vec![];

    if number % 3 == 0 {
        str_vec.push("Pling");
    }

    if number % 5 == 0 {
        str_vec.push("Plang")
    }

    if number % 7 == 0 {
        str_vec.push("Plong")
    }

    if str_vec.len() <= 0{
        return number.to_string();
    } else {
        return str_vec.join("");
    }
    
}

