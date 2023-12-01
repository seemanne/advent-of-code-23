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
        let mut index_of_first_digit = -1;
        let mut latest_digit = 0;
        let mut index_of_last_digit = -1;

        let mut counter = 0;
        let mut has_registered_first = false;
        for char in read_line.chars(){
            if char.is_ascii_digit(){

                let num = match char.to_digit(10){
                    Some(num) => num,
                    None => panic!("This shouldn't happen")
                };
                if !has_registered_first{
                    index_of_first_digit = counter;
                    first_digit = num;
                    has_registered_first = true;
                }
                latest_digit = num;
                index_of_last_digit = counter;
            }
            counter += 1;
        }

        let words_result = find_outermost_words(read_line);

        if words_result.index_of_first_digit < index_of_first_digit || index_of_first_digit == -1{
            first_digit = words_result.first_digit;
        }
        if words_result.index_of_last_digit > index_of_last_digit{
            latest_digit = words_result.last_digit;
        }


        let insert = first_digit * 10 + latest_digit;
        
//        println!("{}", insert);
        num_vec.push(insert)
    }

    println!("{}", num_vec.iter().sum::<u32>());
}

fn recursive_find_all_appearances(string: &str, pattern: &str, len: usize, prev_pos: usize) -> Vec<i32> {
    let mut res = Vec::new();

    if let Some(position) = string.find(pattern){

        res.push((position + prev_pos) as i32);
        let mut future = recursive_find_all_appearances(&string[position + len..], pattern, len, prev_pos + position + len);
        res.append(&mut future);
        res
    } else {
        res
    }
}

fn find_outermost_words(string: String) -> ResultFromWords {

    let mut all_vecs = Vec::new();
    let patterns = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for pattern in patterns {
        all_vecs.push(recursive_find_all_appearances(&string, &pattern, pattern.len(), 0))
    }

    let mut first_digit = 0;
    let mut index_of_first_digit: i32 = 9999999;
    let mut last_digit = 0;
    let mut index_of_last_digit: i32 = -1; 

    let mut counter = 1;
    for vec in all_vecs{
        if vec.len() == 0{
            counter += 1;
            continue;
        }

        for item in vec{
            if item < index_of_first_digit{
                index_of_first_digit = item;
                first_digit = counter;
            }
            if item > index_of_last_digit{
                index_of_last_digit = item;
                last_digit = counter;
            }
        }
        counter += 1;
    }

    ResultFromWords { first_digit: first_digit, index_of_first_digit: index_of_first_digit, last_digit: last_digit, index_of_last_digit: index_of_last_digit }
}

struct ResultFromWords {
    first_digit : u32,
    index_of_first_digit : i32,
    last_digit : u32,
    index_of_last_digit : i32,
}