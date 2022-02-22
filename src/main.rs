use chrono::{Duration, Local};

#[tokio::main]
async fn main() {
    let caldas = "https://portalturismoapi.sescgo.com.br/api/v1/reservas/disponibilidade/{s}/{e}/1,1,/null/N/1/1/%7B%7D/%7B%7D?SescUnidade=1&ReservaId=0".to_string();
    let piri = "https://portalturismoapi.sescgo.com.br/api/v1/reservas/disponibilidade/{s}/{e}/1,1,/null/N/2/1/%7B%7D/%7B%7D?SescUnidade=2&ReservaId=0";
    let day_in = Local::today();
    let day_out = day_in + Duration::days(2);
    for i in 0..=90 {
        let start = (day_in + Duration::days(i)).format("%Y-%m-%d").to_string();
        let end = (day_out + Duration::days(i)).format("%Y-%m-%d").to_string();
        let url = caldas.clone().replace("{s}", &start).replace("{e}", &end);
        // let url = piri.clone().replace("{s}", &start).replace("{e}", &end);
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
