## 利用する

利用するために、クレートを追加してください。

```
$ cargo add tokio
```

## サンプル

```rust
use splatoon3_rs::client::SplaClient;

#[tokio::main]
async fn main() {
    let client = SplaClient::new().unwrap();
    let res = client.get_next_bankara_open_stages().await.unwrap();

    println!("{:?}", res)
}
```
