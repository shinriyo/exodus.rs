exodus.rs(エクソダスアールエス)
====

CRUDのScaffoldingを生成する
npmを入れなくてもいいです。
Rust + PostgreSQL + AngularJS

## 生成されるテンプレートはこのようになるようにしています
https://github.com/shinriyo/nickel-helloworld
※ここにプルリクしてみたら今後変える予定です。

# 最低限

カレントディレクトリに　
```
src/main.rs
argo.toml
```
が必要

# 最低限
`brew install exodus.rs`

# 予定コマンド

## AngularやCSSなどのセット初期化
```
exodus init
```

## 定義と同時に生成(generateがコマンド)
```
exodus generate item name:string price:integer description:text
```
`g`コマンドもOKである。
```
exodus g item name:string price:integer description:text
```

## テーブル初期化

現状は`localhost:6767/setup/item`へアクセスするだけ。
TODO:
```
exodus migrate item
```

## テーブル削除

TODO:
```
exodus delete item
```


ここにプルリクやアドバイス次第で変わります

TODO:
-------
日本語は基本、英語、韓国語、も今後対応させます。
AngularJSではなくReactにします。

自分メモ
-------
`cargo build`
`target/debug/exodus`に生成される

## 公開用
```
cp target/debug/exodus bin
```
