use futures::prelude::*;

pub fn build_influx_client() -> influxdb2::Client {
    let organization = dotenv::var("INFLUXDB_ORGANIZATION").unwrap();
    let influx_url = dotenv::var("INFLUXDB_URL").unwrap();
    let token = dotenv::var("INFLUXDB_TOKEN").unwrap();

    influxdb2::Client::new(influx_url, organization, token)
}

pub async fn write_metrics(
    client: &influxdb2::Client,
    metrics: Vec<influxdb2::models::DataPoint>,
) -> Result<(), influxdb2::RequestError> {
    let bucket = dotenv::var("INFLUXDB_BUCKET").unwrap();

    client.write(&bucket, stream::iter(metrics)).await?;

    Ok(())
}
