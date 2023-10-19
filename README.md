# rust
Rustの自己研鑽リポジトリ

## Tips

```console
$ # 実行
$ cargo run

$ # 他の Terminal などで実行
$ curl 'http://localhost:3002'
> Hello, world!

$ # テストの実行
$ mold -run cargo test

$ # 実行 (ファイルを監視して変更があれば再実行)
$ cargo watch -s 'mold -run cargo run'

$ # テストの実行 (ファイルを監視して変更があれば再実行)
$ cargo watch -s 'mold -run cargo test'
```
