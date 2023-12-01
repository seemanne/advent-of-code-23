use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    
    let input_file = match File::open("input.txt"){
        Ok(file) => file,
        Err(_) => panic!("Error when opening input.txt")
    };
    let mut num_vec: Vec<u32> = Vec::new();

    let reader = BufReader::new(input_file);
    for line in reader.lines(){
        let read_line = match line{
            Ok(line) => line,
            Err(_) => panic!("Error reading line")
        };
        let mut first_digit = 0;
        let mut latest_digit = 0;
        let mut has_registered_first = false;
        for char in read_line.chars(){
            if char.is_ascii_digit(){

                let num = match char.to_digit(10){
                    Some(num) => num,
                    None => panic!("This shouldn't happen")
                };
                if !has_registered_first{
                    first_digit = num;
                    has_registered_first = true;
                }
                latest_digit = num;
            }
        }

        let insert = first_digit * 10 + latest_digit;

        num_vec.push(insert)
    }

    print!("{}", num_vec.iter().sum::<u32>());
}
