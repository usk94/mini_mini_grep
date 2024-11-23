use std::io::{self, Write};

fn main() {
    print!("検索の対象となる文字列を入力してください: ");
    io::stdout().flush().unwrap();

    let mut target_string = String::new();
    io::stdin().read_line(&mut target_string).unwrap();

    print!("検索ワードを入力してください: ");
    io::stdout().flush().unwrap();

    let mut grep_word = String::new();
    io::stdin().read_line(&mut grep_word).unwrap();

    let is_contain = target_string.contains(&grep_word);

    let message = if is_contain {
        "含まれていました"
    } else {
        "含まれていませんでした"
    };
    println!("{}", message);
}
