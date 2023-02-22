use std::collections::HashMap;

// Вариант с преобразованием среза к String 
pub fn parse_str(map: &mut HashMap<String, Vec<String>>, string: &str) {
    let mut str_iter = string.split_whitespace();

    let method = str_iter.next().unwrap();
    let name = str_iter.next().unwrap();
    str_iter.next();
    let company = str_iter.next().unwrap();

    let vec = map.entry(company.to_string()).or_insert(vec![]);

    vec.push(name.to_string());
}

// Вариант с lifeteime тегами
pub fn parse_str_lifetime<'a>(map: &mut HashMap<&'a str, Vec<&'a str>>, string: &'a str) {
    let mut str_iter = string.split_whitespace();

    let method = str_iter.next().unwrap();
    let name = str_iter.next().unwrap();
    str_iter.next();
    let company = str_iter.next().unwrap();

    let vec = map.entry(company).or_insert(vec![]);

    vec.push(name);
}

