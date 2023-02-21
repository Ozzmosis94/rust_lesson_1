pub mod pig_latin {
    const VOWELS: [char; 10] = ['а', 'я', 'у', 'ю', 'о', 'е', 'ё', 'э', 'и', 'ы'];

    pub fn convert(str: &String) -> String {
        let vec_chars: Vec<char> = str.to_lowercase().chars().collect();

        let is_vowels = VOWELS.iter().find(|&&char| char == vec_chars[0]);

        match is_vowels {
            Some(_) => {
                let mut str = String::from(str);
                str.push_str("hay");
                str
            }
            None => {
                let mut str: String = vec_chars[1..].iter().collect();
                str.push(vec_chars[0]);
                str.push_str("ay");
                str
            }
        }
    }
}

pub fn format(word: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    let mut char_iter = word.chars();

    let first_letter = char_iter.next().expect("Not a latter");

    if vowels.contains(&first_letter) {
        format!("{}-hay", &word)
    } else {
        format!("{}-{}ay", char_iter.collect::<String>(), first_letter)
    }
}
