pub(crate) fn example2() {
    match read_file_ext() {
        Ok(_) => println!("成功"),
        Err(e) => println!("エラー: {e}"),
    }
}

fn read_file_ext() -> anyhow::Result<String> {
    let text = read_file()?;
    Ok(text)
}

fn read_file() -> std::result::Result<String, std::io::Error> {
    std::fs::read_to_string("not_found.txt")
}
