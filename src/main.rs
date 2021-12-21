use std::{env, time::Duration};
use futures::{stream, StreamExt};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all="PascalCase")]
struct DominosResponse {
  status: i32,
  code: Option<String>,
  name: Option<String>,
  price: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(2))
    .build()?;

  let concurrency: usize = match env::var("CONCURRENCY") {
    Ok(val) => val.parse().expect("invalid concurrency value"),
    _ => 50
  };
  let store_id = env::var("STORE_ID").expect("no store id environment variable");

  let responses = stream::iter(1000..10000)
    .map(|i| {
      let client = &client;
      let url = format!("https://order.dominos.ca/power/store/{}/coupon/{}?lang=en", store_id, i);
      async move {
        client.get(url)
          .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/11.1.2 Safari/605.1.15")
          .header("Referer", "https://order.dominos.ca/assets/build/xdomain/proxy.html")
          .send()
          .await?
          .json::<DominosResponse>()
          .await
      }
    })
    .buffer_unordered(concurrency);

  println!("{: <4} | {: <7} | Name", "Code", "Price");
  responses
    .for_each(|resp| async {
      if let Ok(data) = resp {
        if data.status != -404 {
          let mut price = data.price.unwrap();
          if price.is_empty() { price = "N/A".to_string(); }
          println!("{: <4} | {: <7} | {}", data.code.unwrap(), format!("${}", price), data.name.unwrap());
        }
      }
    })
    .await;

  Ok(())
}
