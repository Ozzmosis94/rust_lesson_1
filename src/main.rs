use std::{collections::HashMap, io};

use crate::{
    latin::format,
    numbers::{average_of_list, median_of_list, mode_of_list},
    organization::parse_str_lifetime,
};

mod latin;
mod numbers;
mod organization;

fn main() {
    let arr = vec![1, 3, 3, 3, 5, 5, 5, 5];

    println!("Average value of list {}", average_of_list(&arr));
    println!("Median value of list {}", median_of_list(&arr));
    println!("Mode value of list {}", mode_of_list(&arr));

    println!("Pig latin {}", format("a"));

    let mut map = HashMap::new();

    // Пример строки Add {name} to {corp} 
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    parse_str_lifetime(&mut map, &input);

    // Пример строки Add {name} to {corp} 
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    parse_str_lifetime(&mut map, &input);

    dbg!(map);
}
