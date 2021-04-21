<h1 align="center">
    Diamant
</h1>

<p align="center">
    <img alt="Diamant" src="./logo.png?raw=true" width="96">
</p>

<p align="center">
  Diamant is a both CLI tool and REST API for GTFS-JP
</p>

<p align="center">
  <a href="https://github.com/tadashi-aikawa/diamant/releases/latest">
    <img alt="Version" src="https://img.shields.io/github/v/release/tadashi-aikawa/diamant?label=%F0%9F%93%A6release" />
  </a>
  <a href="https://github.com/tadashi-aikawa/diamant/actions/workflows/release.yaml">
    <img alt="Release" src="https://github.com/tadashi-aikawa/diamant/actions/workflows/release.yaml/badge.svg" />
  </a>
</p>


Use Diamant as CLI
------------------

Show usages in help.

```shell
diamant -h
```

Use Diamant as API
------------------

```shell
diamant serve
```

### Support APIs

| Path    | Queries | Description        |
| ------- | ------- | ------------------ |
| /config |         | Show version.. etc |


Support files
-------------

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
