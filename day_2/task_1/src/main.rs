use std::{fs::File, io::BufReader, io::BufRead};

use regex::{Regex, Captures};
fn main() {
    
    let loaded_file=File::open("input.txt").unwrap();
    let reader = BufReader::new(loaded_file);

    //This looks shitty but avoids recompiling the regex for every iteration loop
    let GAME_RE = Regex::new(r"Game ([\d]*):").unwrap();
    let PULL_RE = RegexStorage{
        green_re : Regex::new(r"([\d]*) green").unwrap(),
        red_re : Regex::new(r"([\d]*) red").unwrap(),
        blue_re : Regex::new(r"([\d]*) blue").unwrap(),
    };

    let mut total = 0;

    for line in reader.lines() {

        let mut read_line = line.unwrap();
        let game_nbr = get_game_nbr(&mut read_line, &GAME_RE);
        let elf_pulls = recursive_extract_elf_pulls(&mut read_line, &PULL_RE);
        let game_possible: Vec<bool> = elf_pulls.into_iter().map(|x| x.is_valid_game()).collect();

        let mut is_possible = true;
        for is_valid in game_possible.into_iter() {

            is_possible = is_possible && is_valid;
        }

        if is_possible {
            total += game_nbr;
        }
    }

    println!("The toatal is: {}", total);
}

fn get_game_nbr(line: &mut String, re: &Regex) -> i32{

    let mut game_nbr = 0;

    if let Some(cap) = re.captures(&line){

        if let Some(grp_1) = cap.get(1){

            game_nbr = grp_1.as_str().parse::<i32>().unwrap();
        }
        if let Some(grp_0) = cap.get(0){

            line.drain(..grp_0.end());
            println!("line is now {}", &line);
            line.push(';');
        }
    } else {
        panic!("Failed to capture game for line {}", &line);
    }
    
    game_nbr
}

struct ElfPull{
    n_green : i32,
    n_red : i32,
    n_blue : i32
}

struct RegexStorage{
    green_re : Regex,
    red_re : Regex,
    blue_re : Regex
}

fn recursive_extract_elf_pulls(line: &mut String, re: &RegexStorage) -> Vec<ElfPull> {

    let mut ret_vec = Vec::<ElfPull>::new();

    if line.is_empty(){
        return ret_vec
    }

    if let Some(pos) = line.find(';') {

        let elf_pull_str: String = line.drain(..(pos + 1)).collect();
        ret_vec.push(string_to_elf_pull(elf_pull_str, re));
        ret_vec.append(&mut recursive_extract_elf_pulls(line, re));

    } else {
        panic!("Failed recursion for line {}", &line);
    }


    ret_vec
}

fn string_to_elf_pull(elf_pull_str: String, re: &RegexStorage) -> ElfPull {

    let mut n_green = 0;
    let mut n_red = 0;
    let mut n_blue = 0;

    if let Some(cap) = re.green_re.captures(&elf_pull_str){

        if let Some(grp_1) = cap.get(1){

            n_green = grp_1.as_str().parse::<i32>().unwrap();
        }
    }

    if let Some(cap) = re.red_re.captures(&elf_pull_str){

        if let Some(grp_1) = cap.get(1){

            n_red = grp_1.as_str().parse::<i32>().unwrap();
        }
    }

    if let Some(cap) = re.blue_re.captures(&elf_pull_str){

        if let Some(grp_1) = cap.get(1){

            n_blue = grp_1.as_str().parse::<i32>().unwrap();
        }
    }

    ElfPull { n_green: n_green, n_red: n_red, n_blue: n_blue}
}

impl ElfPull {

    fn is_valid_game(self) -> bool{

        return self.n_red <= 12 && self.n_green <= 13 && self.n_blue <= 14
    }
}