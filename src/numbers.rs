use std::collections::HashMap;

pub fn average_of_list(list: &[i32]) -> f32 {
    list.iter().sum::<i32>() as f32 / list.len() as f32
}

pub fn median_of_list(list: &Vec<i32>) -> f32 {
    let mut list: Vec<i32> = list.clone();
    list.sort();

    // list.sort();

    if list.len() % 2 == 0 {
        let right_number = list[list.len() / 2] as f32;
        let left_number = list[(list.len() / 2) - 1] as f32;

        (right_number + left_number) / 2.0
    } else {
        list[list.len() / 2] as f32
    }
}

pub fn mode_of_list(list: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for &value in list {
        *map.entry(value).or_insert(0) += 1
    }

    *map.iter()
        .max_by_key(|&(_, v)| v)
        .map(|(k, _v)| k)
        .expect("Error get mode")
}
