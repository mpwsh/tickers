use actix_web::{error, get, middleware, web, web::Data, App, HttpResponse, HttpServer};
use anyhow::Result;
use awc::{http::header, Client, Connector};
use rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::str;
use std::sync::Arc;
use std::time::Duration;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolData {
    pub symbol: String,
    pub pair: String,
    pub price: String,
    pub dir: String,
    pub change: String,
}

impl SymbolData {
    fn build(res: serde_json::Value, tick: String) -> Self {
        let index = &res["data"]
            .as_array()
            .unwrap()
            .iter()
            .position(|x| x["instId"].as_str() == Some(&tick))
            .unwrap_or(0);
        let v: Vec<&str> = tick.split("-").collect();
        let symbol = v[0].to_string();
        let pair = v[1].to_string();
        let open: f32 = res["data"][index]["open24h"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap();
        let price = res["data"][index]["last"]
            .as_str()
            .unwrap()
            .parse::<f32>()
            .unwrap();
        let dir = match open > price {
            true => "⬇",
            false => "⬆",
        };
        let change = price - open;
        let change = format!("{:.2}", (change / open) * 101.00).parse().unwrap();
        Self {
            symbol,
            pair,
            price: price.to_string(),
            dir: dir.to_string(),
            change,
        }
    }
}
#[get("/price/{inst_ids}")]
async fn tickers(
    client: web::Data<Client>,
    inst_ids: web::Path<String>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    if &inst_ids.to_string() == "" {
        let msg = format!("Please provide one or more instrument_ids.\nSingle: XCH-USDT\nMultiple: SOL-USDT,XCH-USDT,ETH-USDT,BTC-USDT,FTM-USDT");
        log::warn!("{}", msg);
        return Ok(HttpResponse::BadRequest().body(msg));
    };

    //Get ticker data
    let mut res = client
        .get("https://www.okx.com/api/v5/market/tickers?instType=SPOT")
        .append_header(("Accept", "application/json"))
        .send()
        .await
        .map_err(error::ErrorInternalServerError)?;
    let body = &res.body().await?;
    let data: Value = serde_json::from_str(str::from_utf8(body)?)?;
    //Split string and build payload
    let v: Vec<&str> = inst_ids.split(",").collect();
    let mut payload = String::new();
    for i in v.iter() {
        let parsed = &SymbolData::build(data.clone(), i.to_string());
        payload.push_str(&format!(
            "{}: {} {}% {} {}\n",
            parsed.symbol, parsed.dir, parsed.change, parsed.price, parsed.pair
        ));
    }
    Ok(HttpResponse::Ok().body(payload))
}

#[get("/")]
async fn no_params() -> &'static str {
    r#"======= Tickers Simple =======
Github: https://github.com/mpwsh/tickers
Author: mpw <x@mpw.sh>
==============================
Available endpoints:
  Single:
  - /price/<inst_id> -- Example: /price/XCH-USDT
  Multiple:
  - /price/<inst_id>,<inst_id>,<inst_id> -- Example: /price/SOL-USDT,XCH-USDT
"#
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let port: u16 = 3030;
    let user_agent = "n9-proxy/0.1.0";
    let client_tls_config = Arc::new(rustls_config());
    log::info!("starting HTTP server at http://0.0.0.0:{}", port);

    HttpServer::new(move || {
        let client = Client::builder()
            .add_default_header((header::USER_AGENT, user_agent.clone()))
            .connector(
                Connector::new()
                    .timeout(Duration::from_secs(30))
                    .rustls(Arc::clone(&client_tls_config)),
            )
            .finish();
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))
            .wrap(middleware::Compress::default())
            .app_data(Data::new(client))
            .wrap(middleware::Logger::default().log_target("http_log"))
            .service(tickers)
            .service(no_params)
        //.service(web::resource("/test1.html").to(|| async { "Test\r\n" }))
    })
    .bind(("0.0.0.0", port))?
    .workers(1)
    .run()
    .await
}

fn rustls_config() -> ClientConfig {
    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth()
}
