mod letter_set;
use letter_set::Letters;

fn main() {
    let words = include_str!("sorted_words.txt");
    let letters = get_possible_letters();
    eprintln!("Here are your results:");
    words
        .lines()
        .filter(|&word| word.len() >= 3)
        .filter(|&word| letters.has(word))
        .for_each(|word| {
            println!("{}", word);
        });
    // let letters = Letters::from_string("tons").unwrap();
    // assert!(letters.has("ton"));
    // assert!(letters.has("not"));
    // assert!(letters.has("snot"));
    // assert!(letters.has("onts"));
    // assert!(!letters.has("ants"));
}

fn get_possible_letters() -> Letters {
    eprintln!("What letters are available in this puzzle? (Example: amlion)");
    let mut input = String::new();
    ::std::io::stdin().read_line(&mut input).expect("reading input shouldn't fail");
    Letters::from_string(&input).expect("input shouldn't be too long")
}
