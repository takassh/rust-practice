use dotenv::dotenv;
use std::{env, process::id};
use tracing::info;
use tracing_loki::url::Url;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    dotenv().ok();

    // Set up loki
    let host = std::env::var("GRAFANA_HOST").unwrap();
    let user = std::env::var("GRAFANA_USER").unwrap();
    let password = std::env::var("GRAFANA_PASSWORD").unwrap();
    let url = Url::parse(&format!("https://{user}:{password}@{host}")).unwrap();
    let (layer, task) = tracing_loki::builder()
        .label("application", "test-application-logs")
        .unwrap()
        .extra_field("pid", format!("{}", id()))
        .unwrap()
        .build_url(url)
        .unwrap();

    // Set up the tracing subscriber
    tracing_subscriber::registry()
        .with(EnvFilter::builder().from_env().unwrap())
        .with(tracing_subscriber::fmt::Layer::new())
        .with(layer)
        .init();

    tokio::spawn(task);

    let mut count = 0;
    loop {
        count += 1;
        info!("Hello, world! This is a test log message. count:{}", count);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
