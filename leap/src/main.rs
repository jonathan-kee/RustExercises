use std::io;

fn main() {
    loop {
        println!("Enter a year!");
        let mut year = String::new();
        io::stdin()
            .read_line(&mut year)
            .expect("Failed to read line");
        println!("You year: {}", year);

        let year: u32 = match year.trim().parse() {
            //page 28, Handling Invalid Input
            Ok(num) => num, //If parse is success, return Ok value with resulting number
            Err(_) => continue,
        };

        if year%4 == 0 {
            if year%100 != 0 {
                leapyear(year);
            } else if year%400 == 0 {
                leapyear(year);
            } else {
                notleapyear(year);
            }
        } else {
            notleapyear(year);
        }
    
    }
}

pub fn leapyear(year: u32){
    println!("{} is a leap year!", year);
}

pub fn notleapyear(year: u32){
    println!("{} is a not leap year!", year);
}