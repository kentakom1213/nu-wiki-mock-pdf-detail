# nu-wiki-mock-pdf-detail
[nu-wiki](https://github.com/NU-wiki-web)のモックサーバ

## How To Use

### APIのURL
```
https://nu-wiki-mock-pdf-detail.shuttleapp.rs
```

### pdfリスト

- `/list`：pdfのリストを返します

<detail><summary>例</summary>
<div>

```
$ curl https://nu-wiki-mock-pdf-detail.shuttleapp.rs/list
[
  {
    "file_id": 0,
    "name": "微分積分学1"
  },
  {
    "file_id": 1,
    "name": "線形代数学"
  },
  {
    "file_id": 2,
    "name": "システム数学及び演習1"
  },
  {
    "file_id": 3,
    "name": "シミュレーション"
  },
  {
    "file_id": 4,
    "name": "物理基礎2"
  }
]
```

</div>

### pdf詳細

- `/detail/:file_id`：与えられたidを持つpdfファイルの詳細情報を返します

<detail><summary>例</summary>
<div>

```
$ curl https://nu-wiki-mock-pdf-detail.shuttleapp.rs/detail/0
[
  {
    "file_id": 0,
    "name": "微分積分学1"
  },
  {
    "file_id": 1,
    "name": "線形代数学"
  },
  {
    "file_id": 2,
    "name": "システム数学及び演習1"
  },
  {
    "file_id": 3,
    "name": "シミュレーション"
  },
  {
    "file_id": 4,
    "name": "物理基礎2"
  }
]
```

</div>
