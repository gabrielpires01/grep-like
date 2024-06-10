use std::{env, fs, io::BufRead};
use regex::Regex;

fn main() {
    let passed_args = env::args().skip(1);

    if passed_args.len() < 2 {
        panic!("Precisa conter o path e o termo de busca")
    };

    let vector: Vec<String> = passed_args.collect();     
    
    let (path, term) = (&vector[0], &vector[1]);
    let reg = Regex::new(term).unwrap();
    println!("\nPath '{}', Term '{}'\n", path, term);

    let file = fs::read(path).unwrap();

    let lines: Vec<String> = file.lines().enumerate().filter_map(|(i, line)| {
        let line_string = line.unwrap().to_string();

        if reg.is_match(&line_string) {  Some(format!("Linha {}: {}", i.to_string(), line_string)) }  else { None }

    }).collect();

    if lines.len() == 0 {
        println!("Nenhuma linha encontrada com esse termo")
    } else {
        println!("Resultado: {:?}\n", lines)
    }
}
