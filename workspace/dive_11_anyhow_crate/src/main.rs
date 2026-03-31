use anyhow::Context;

fn main() {
    match std() {
        Ok(content) => println!("ファイルの内容: {}", content),
        Err(e) => eprintln!("std: {}", e),
    }

    match anyhow_context() {
        Ok(content) => println!("ファイルの内容: {}", content),
        Err(e) => eprintln!("anyhow_context: {}", e),
    }

    match anyhow_with_context() {
        Ok(content) => println!("ファイルの内容: {}", content),
        Err(e) => eprintln!("anyhow_with_context: {}", e),
    }

    match anyhow_anyhow_macro() {
        Ok(content) => println!("ファイルの内容: {}", content),
        Err(e) => eprintln!("anyhow_anyhow_macro: {}", e),
    }

    match anyhow_bail_macro() {
        Ok(content) => println!("ファイルの内容: {}", content),
        Err(e) => eprintln!("anyhow_bail_macro: {}", e),
    }

    match anyhow_ensure_macro() {
        Ok(_) => println!("ファイルの内容: "),
        Err(e) => eprintln!("anyhow_ensure_macro: {}", e),
    }
}

fn std() -> Result<String, std::io::Error> {
    std::fs::read_to_string("nonexist.txt")
}

fn anyhow_context() -> anyhow::Result<String> {
    std::fs::read_to_string("nonexist.txt").context("ファイルの読み込みに失敗しました")
}

fn anyhow_with_context() -> anyhow::Result<String> {
    std::fs::read_to_string("nonexist.txt").with_context(|| "ファイルの読み込みに失敗しました")
}

fn anyhow_anyhow_macro() -> anyhow::Result<String> {
    let content = std::fs::read_to_string("nonexist.txt");
    match content {
        Ok(content) => Ok(content),
        Err(e) => Err(anyhow::anyhow!("ファイルの読み込みに失敗しました: {}", e)),
    }
}

fn anyhow_bail_macro() -> anyhow::Result<String> {
    let content = std::fs::read_to_string("nonexist.txt");
    match content {
        Ok(content) => Ok(content),
        Err(e) => anyhow::bail!("ファイルの読み込みに失敗しました: {}", e),
    }
}

fn anyhow_ensure_macro() -> anyhow::Result<String> {
    let content = std::fs::read_to_string("exist.txt")?;
    anyhow::ensure!(
        content.len() > 0,
        "ファイルの中身が空です"
    );
    Ok(content)
}
