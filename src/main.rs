use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    fetch_thread(7831278321).await;
}

fn make_sign(post_body: &BTreeMap<&str, &str>) -> String {
    let mut sign = String::new();
    for (k, v) in post_body {
        sign.push_str(k);
        sign.push_str("=");
        sign.push_str(v);
    }
    sign.push_str("tiebaclient!!!");
    format!("{:X}", md5::compute(sign))
}
