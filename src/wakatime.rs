use crate::utils::seconds_to_minutes;
use influxdb2::models::{data_point::DataPointError, DataPoint};

#[derive(serde::Deserialize)]
struct WakatimeUserSummaryGrandTotal {
    total_seconds: f64,
}

/// NOTE: the struct includes a lot more fields,
/// but total_seconds is the only one we need, since everything else can be computed
#[derive(serde::Deserialize)]
struct WakatimeStats {
    name: String,
    total_seconds: f64,
}

#[derive(serde::Deserialize)]
struct WakatimeStatsRange {
    /// "2022-10-10",
    date: String,
}

#[derive(serde::Deserialize)]
struct WakatimeUserSummaryData {
    /// Time range
    range: WakatimeStatsRange,
    grand_total: WakatimeUserSummaryGrandTotal,
    categories: Vec<WakatimeStats>,
    projects: Vec<WakatimeStats>,
    languages: Vec<WakatimeStats>,
}

#[derive(serde::Deserialize)]
pub struct WakatimeUserSummaryResponse {
    data: Vec<WakatimeUserSummaryData>,
}

pub async fn fetch_metrics(
    client: &reqwest::Client,
) -> Result<WakatimeUserSummaryResponse, Box<dyn std::error::Error>> {
    let metrics_response = client
        .get("https://wakatime.com/api/v1/users/current/summaries?range=last_14_days")
        .send()
        .await?
        .json::<WakatimeUserSummaryResponse>()
        .await?;

    Ok(metrics_response)
}

fn create_metric(
    measurements: &str,
    timestamp: i64,
    tag: &str,
    tag_value: &str,
    field: &str,
    field_value: i64,
) -> Result<DataPoint, DataPointError> {
    DataPoint::builder(measurements)
        .timestamp(timestamp)
        .tag(tag, tag_value)
        .field(field, field_value)
        .build()
}

pub fn parse_metrics(metrics: WakatimeUserSummaryResponse) -> Vec<DataPoint> {
    let mut parsed_metrics: Vec<DataPoint> = vec![];

    for metric in metrics.data {
        let date = chrono::DateTime::parse_from_rfc3339(
            format!("{}T00:00:00.000Z", metric.range.date).as_str(),
        )
        .unwrap();

        let timestamp = date.timestamp_nanos();

        // Total time working
        let total_working_minutes = seconds_to_minutes(metric.grand_total.total_seconds);
        if total_working_minutes > 0 {
            let datapoint = DataPoint::builder("total")
                .timestamp(timestamp)
                .field("minutes", total_working_minutes)
                .build();

            if let Ok(datapoint) = datapoint {
                parsed_metrics.push(datapoint);
            }
        }

        // Languages
        if !metric.languages.is_empty() {
            for language in metric.languages {
                let minutes = seconds_to_minutes(language.total_seconds);

                if minutes > 0 {
                    let datapoint = create_metric(
                        "languages",
                        timestamp,
                        "language",
                        &language.name.to_lowercase(),
                        "minutes",
                        minutes,
                    );

                    if let Ok(datapoint) = datapoint {
                        parsed_metrics.push(datapoint);
                    }
                }
            }
        }

        // Projects
        if !metric.projects.is_empty() {
            for project in metric.projects {
                let minutes = seconds_to_minutes(project.total_seconds);

                if minutes > 0 {
                    let datapoint = create_metric(
                        "projects",
                        timestamp,
                        "project",
                        &project.name.to_lowercase(),
                        "minutes",
                        minutes,
                    );

                    if let Ok(datapoint) = datapoint {
                        parsed_metrics.push(datapoint);
                    }
                }
            }
        }

        // Typed of work (coding, debugging etc.)
        if !metric.categories.is_empty() {
            for work_type in metric.categories {
                let minutes = seconds_to_minutes(work_type.total_seconds);

                if minutes > 0 {
                    let datapoint = create_metric(
                        "work_type",
                        timestamp,
                        "work_type",
                        &work_type.name.to_lowercase(),
                        "minutes",
                        minutes,
                    );

                    if let Ok(datapoint) = datapoint {
                        parsed_metrics.push(datapoint);
                    }
                }
            }
        }
    }

    parsed_metrics
}
