pub fn format(word: &str) -> String {
    let vowels = vec!['а', 'я', 'у', 'ю', 'о', 'е', 'ё', 'э', 'и', 'ы'];

    let mut char_iter = word.chars();

    let first_letter = char_iter.next().expect("Not a latter");

    if vowels.contains(&first_letter) {
        format!("{}-hay", &word)
    } else {
        format!("{}-{}ay", char_iter.collect::<String>(), first_letter)
    }
}
