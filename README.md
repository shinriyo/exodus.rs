exodus.rs(エクソダスアールエス)
====

CRUDのScaffoldingを生成する
npmを入れなくてもいいです。
Rust + PostgreSQL + AngularJS

## テンプレートはこのようになるようにしています
https://github.com/shinriyo/nickel-helloworld

# 最低限

カレントディレクトリに　
src/main.rs
argo.toml
がいる

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
gもOK
```
exodus g item name:string price:integer description:text
```

## テーブル生成
```

```

## テーブル削除
```

```


ここにプルリクやアドバイス次第で変わります

TODO:
-------
日本語は基本、英語、韓国語、も今後対応させます。
AngularJSではなくReactにします。

自分メモ
-------
`cargo run`
`target/debug/exodus`に生成される

```
cp target/debug/exodus ~/hoge
```
