use chrono::{Duration, Local};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let unidade = env::var("UNIDADE").unwrap();
    let base_url = format!("https://portalturismoapi.sescgo.com.br/api/v1/reservas/disponibilidade/(s)/(e)/1,1,/null/N/{unidade}/1/%7B%7D/%7B%7D?SescUnidade={unidade}&ReservaId=0");
    let day_in = Local::today();
    let day_out = day_in + Duration::days(2);
    for i in 0..=90 {
        let start = (day_in + Duration::days(i)).format("%Y-%m-%d").to_string();
        let end = (day_out + Duration::days(i)).format("%Y-%m-%d").to_string();
        let url = base_url.clone().replace("(s)", &start).replace("(e)", &end);
        let res = reqwest::get(url).await.unwrap();
        let body = res.text().await.unwrap();
        println!(
            "{:?} -> {:?}: {:?}",
            start,
            end,
            if body.contains("Não existe disponibilidade") {
                "😭 😭"
            } else {
                "😎😎😎😎😎😎😎😎😎"
            }
        );
    }
}
