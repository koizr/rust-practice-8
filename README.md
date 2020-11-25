[キーとそれに紐づいた値をハッシュマップに格納する \- The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/ch08-03-hash-maps.html#%E3%81%BE%E3%81%A8%E3%82%81)

まとめの練習問題

- 整数のリストが与えられ、ベクタを使ってmean(平均値)、median(ソートされた時に真ん中に来る値)、 mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)を返してください。
- 文字列をピッグ・ラテン(訳注: 英語の言葉遊びの一つ)に変換してください。各単語の最初の子音は、 単語の終端に移り、"ay"が足されます。従って、"first"は"irst-fay"になります。ただし、 母音で始まる単語には、お尻に"hay"が付け足されます("apple"は"apple-hay"になります)。 UTF-8エンコードに関する詳細を心に留めておいてください！
- ハッシュマップとベクタを使用して、ユーザに会社の部署に雇用者の名前を追加させられるテキストインターフェイスを作ってください。 例えば、"Add Sally to Engineering"(開発部門にサリーを追加)や"Add Amir to Sales"(販売部門にアミールを追加)などです。 それからユーザに、ある部署にいる人間の一覧や部署ごとにアルファベット順で並べ替えられた会社の全人間の一覧を扱わせてあげてください。

# 実行

テストコードを用意してあるので、テストを実行すれば正しく実装できているかがわかる

```sh
cargo test
```
