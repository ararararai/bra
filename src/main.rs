// src/main.rs
mod braille;
mod mapping;

use braille::BrailleConverter;
use std::io::{self, Write};

fn main() {
    let converter = BrailleConverter::new();

    loop {
        print!("文章を入力してください（終了するには「quit」と入力）: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("入力の読み込みに失敗しました");

        let input = input.trim();

        if input == "quit" {
            println!("プログラムを終了します");
            break;
        }

        // 未対応の文字をチェック
        let unsupported: Vec<char> = input
            .chars()
            .filter(|&c| !converter.is_supported(c) && !c.is_whitespace())
            .collect();

        if !unsupported.is_empty() {
            println!("未対応の文字が含まれています: {:?}", unsupported);
            continue;
        }

        // 文章を変換
        let patterns = converter.convert_text(input);

        // 結果を表示
        println!("\n{}", converter.display_text(&patterns));
    }
}
