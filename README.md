# influxdb-wakatime-metrics

Quick and dirty job for sending Wakatime metrics to InfluxDB.

## Usage

To run the program the following env values are needed:

| key                   | description                                                                                           |
| --------------------- | ----------------------------------------------------------------------------------------------------- |
| WAKATIME_API_KEY      | Api key for Wakatime. Can be found in your [Wakatime settings](https://wakatime.com/settings/account) |
| INFLUXDB_TOKEN        | InfluxDB api token ([docs](https://docs.influxdata.com/influxdb/cloud/security/tokens/create-token/)) |
| INFLUXDB_URL          | URL of InfluxDB                                                                                       |
| INFLUXDB_ORGANIZATION | InfluxDB Organization                                                                                 |
| INFLUXDB_BUCKET       | Bucket to send metrics                                                                                |
