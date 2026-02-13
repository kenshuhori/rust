mod person;

use std::error::Error;

use person::Person;

#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("IO の問題が発生しました")]
    Io(#[from] std::io::Error),
    #[error("パースに失敗しました")]
    InvalidFormat {
        #[source]
        source: std::num::ParseIntError,
    },
}

fn read_file() -> Result<String, MyError> {
    let content = std::fs::read_to_string("nonexist.txt")?;
    Ok(content)
}

fn parse_number(s: &str) -> Result<i32, MyError> {
    s.parse::<i32>()
        .map_err(|e| MyError::InvalidFormat { source: e })
}

fn main() {
    // #[error("…")] の例
    match Person::new("田中 太郎", -1) {
        Ok(_) => {
            println!("成功しました");
        }
        Err(e) => {
            println!("エラーが発生しました: {}", e);
        }
    }

    // #[from] の例
    match read_file() {
        Ok(content) => {
            println!("ファイルの読み取りに成功しました: {}", content);
        }
        Err(e) => {
            println!("ファイルの読み取りに失敗しました。原因: {}", e);
        }
    }

    // #[source] の例
    match parse_number("abc") {
        Ok(_) => {
            println!("パースに成功しました");
        }
        Err(e) => {
            match e.source() {
                Some(source) => println!("パースに失敗しました。根本のエラー: {}", source),
                None => println!("パースに失敗しました。根本のエラー情報はありません"),
            }
        }
    }
}