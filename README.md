<h1 align="center">
    Diamant
</h1>

<p align="center">
    <img alt="Diamant" src="./logo.png?raw=true" width="96">
</p>

<p align="center">
  DiamantはGTFS-JPをCLI、REST APIどちらの方式でも使えるツールです
</p>

<p align="center">
  <a href="https://github.com/tadashi-aikawa/diamant/releases/latest">
    <img alt="Version" src="https://img.shields.io/github/v/release/tadashi-aikawa/diamant?label=%F0%9F%93%A6release" />
  </a>
  <a href="https://github.com/tadashi-aikawa/diamant/actions/workflows/release.yaml">
    <img alt="Release" src="https://github.com/tadashi-aikawa/diamant/actions/workflows/release.yaml/badge.svg" />
  </a>
</p>

⚠ The descriptions for Diamant is only support for Japanese, sorry..


CLIとして使う
-------------

ヘルプを参照してください。

```shell
diamant -h
```

APIとして使う
-------------

`diamant db create`で作成したデータベースを`$(pwd)/db/<key>/gtfs.db`に配置されている必要があります。

`<key>`の値に制約はありません。以下は一例です。

```console
📂 .
└── 📂 db
    ├── 📂 company1
    │  └── gtfs.db
    └── 📂 company2
       └── gtfs.db   
```

`serve`コマンドを実行するとサーバーが起動します。

```shell
diamant serve
```


### サポートAPI

#### /config

バージョンなどを表示する

#### /{key}/stops

stopとメタ情報を取得する。

| Query              | 説明                                                   | 例      |
| ------------------ | ------------------------------------------------------ | ------- |
| `trip_ids`         | 指定したtripを通る情報を取得. カンマ区切りで複数指定可 | 100,200 |
| `stop_name_prefix` | stop_nameが前方一致する情報を表示                      | 市役所  |

#### TODO

- [ ] Swaggerにおける提供


サポートファイル一覧
--------------------

| file                  | insert |
| --------------------- | ------ |
| [agency.txt]          | 💎   |
| [agency_jp.txt]       | 💎   |
| [stops.txt]           | 💎   |
| [routes.txt]          | 💎   |
| [routes_jp.txt]       | 💎   |
| [trips.txt]           | 💎   |
| [office_jp.txt]       | 💎   |
| [stop_times.txt]      | 💎   |
| [calendar.txt]        | 💎   |
| [calendar_dates.txt]  | 💎   |
| [fare_attributes.txt] | 💎   |
| [fare_rules.txt]      | 💎   |
| [shapes.txt]          | 💎   |
| [frequencies.txt]     | 💎   |
| [transfers.txt]       | 💎   |
| [feed_info.txt]       | 💎   |
| [translations.txt]    | 💎   |

[agency.txt]: https://www.gtfs.jp/developpers-guide/format-reference.html#agency
[agency_jp.txt]: https://www.gtfs.jp/developpers-guide/format-reference.html#agency
[stops.txt]: https://www.gtfs.jp/developpers-guide/format-reference.html#stops
[routes.txt]: https://www.gtfs.jp/developpers-guide/format-reference.html#routes
[routes_jp.txt]: https://www.gtfs.jp/developpers-guide/format-reference.html#routes
[trips.txt]: https://www.gtfs.jp/developpers-guide/format-reference.html#trips
[office_jp.txt]: https://www.gtfs.jp/developpers-guide/format-reference.html#office_jp
[stop_times.txt]: https://www.gtfs.jp/developpers-guide/format-reference.html#stop_times
[calendar.txt]: https://www.gtfs.jp/developpers-guide/format-reference.html#calendar
[calendar_dates.txt]: https://www.gtfs.jp/developpers-guide/format-reference.html#calendar
[fare_attributes.txt]: https://www.gtfs.jp/developpers-guide/format-reference.html#fare
[fare_rules.txt]: https://www.gtfs.jp/developpers-guide/format-reference.html#fare
[shapes.txt]: https://www.gtfs.jp/developpers-guide/format-reference.html#shapes
[frequencies.txt]: https://www.gtfs.jp/developpers-guide/format-reference.html#frequencies
[transfers.txt]: https://www.gtfs.jp/developpers-guide/format-reference.html#transfers
[feed_info.txt]: https://www.gtfs.jp/developpers-guide/format-reference.html#feed_info
[translations.txt]: https://www.gtfs.jp/developpers-guide/format-reference.html#translations


開発者へ
--------

### 開発実行

```shell
cargo run -- -h
```

### リリース

1. `cargo.toml`のversionを更新
2. `📦 v0.1.2`のようにコミット
3. `v0.1.2`のようなgitのtagを作成
4. push

すると、GitHub Actionsが自動で`Release`アクションを実行します。
