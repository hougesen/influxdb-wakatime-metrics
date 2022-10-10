mod influx;
mod utils;
mod wakatime;

use influx::{build_influx_client, write_metrics};
use utils::build_reqwest_client;
use wakatime::{fetch_metrics, parse_metrics};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let client = build_reqwest_client()?;

    let metrics = fetch_metrics(&client).await;

    if let Ok(metrics) = metrics {
        let parsed_metrics = parse_metrics(metrics);

        let influx_client = build_influx_client();

        write_metrics(&influx_client, parsed_metrics).await?;
    }

    Ok(())
}
