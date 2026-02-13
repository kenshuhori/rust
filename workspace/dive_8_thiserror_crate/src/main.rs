mod person;

use person::Person;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO の問題が発生しました")]
    Io(#[from] std::io::Error),
}

fn read_file() -> Result<String, Error> {
    let content = std::fs::read_to_string("nonexist.txt")?;
    Ok(content)
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
}
