use anyhow::Context;

fn main() {
    match std() {
        Ok(content) => println!("ファイルの内容: {}", content),
        Err(e) => eprintln!("{}", e),
    }

    match anyhow_context() {
        Ok(content) => println!("ファイルの内容: {}", content),
        Err(e) => eprintln!("{}", e),
    }

    match anyhow_with_context() {
        Ok(content) => println!("ファイルの内容: {}", content),
        Err(e) => eprintln!("{}", e),
    }

    match anyhow_anyhow_macro() {
        Ok(content) => println!("ファイルの内容: {}", content),
        Err(e) => eprintln!("{}", e),
    }

    match anyhow_bail_macro() {
        Ok(content) => println!("ファイルの内容: {}", content),
        Err(e) => eprintln!("{}", e),
    }

    match anyhow_ensure_macro() {
        Ok(_) => println!("ファイルの内容: "),
        Err(e) => eprintln!("{}", e),
    }
}

fn std() -> Result<String, std::io::Error> {
    std::fs::read_to_string("config.json")
}

fn anyhow_context() -> anyhow::Result<String> {
    std::fs::read_to_string("config.json").context("設定ファイルの読み込みに失敗しました")
}

fn anyhow_with_context() -> anyhow::Result<String> {
    std::fs::read_to_string("config.json").with_context(|| "設定ファイルの読み込みに失敗しました")
}

fn anyhow_anyhow_macro() -> anyhow::Result<String> {
    let content = std::fs::read_to_string("config.json");
    match content {
        Ok(content) => Ok(content),
        Err(e) => Err(anyhow::anyhow!("設定ファイルの読み込みに失敗しました: {}", e)),
    }
}

fn anyhow_bail_macro() -> anyhow::Result<String> {
    let content = std::fs::read_to_string("config.json");
    match content {
        Ok(content) => Ok(content),
        Err(e) => anyhow::bail!("設定ファイルの読み込みに失敗しました: {}", e),
    }
}

fn anyhow_ensure_macro() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("config.json");
    let result = anyhow::ensure!(content.is_ok(), "設定ファイルの読み込みに失敗しました: {:?}", content.err());
    Ok(result)
}
