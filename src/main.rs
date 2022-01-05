use aws_signing_request::request::{
    CanonicalRequestBuilder, AUTHORIZATION, X_AMZ_CONTENT_SHA256, X_AMZ_DATE,
};
use aws_signing_request::request::{AWS_JSON_CONTENT_TYPE, X_AWZ_TARGET};
use chrono::Utc;

#[tokio::main]
async fn main() {
    let host = "ingest.timestream.us-east-1.amazonaws.com";
    let body = "{}";
    let aws_access_key_id = env!("AWS_ACCESS_KEY_ID");
    let aws_secret_access_key = env!("AWS_SECRET_ACCESS_KEY");

    let canonical_request = CanonicalRequestBuilder::new(
        host,
        "POST",
        "/",
        aws_access_key_id,
        aws_secret_access_key,
        "us-east-1",
        "timestream",
    )
    .header("Content-Type", AWS_JSON_CONTENT_TYPE)
    .header(X_AWZ_TARGET, "Timestream_20181101.DescribeEndpoints")
    .body(body)
    .build(Utc::now());

    let client = reqwest::Client::new();

    let endpoint = client
        .post(format!("https://{}", host))
        .header(X_AMZ_DATE, &canonical_request.date.iso_8601)
        .header("Content-Type", AWS_JSON_CONTENT_TYPE)
        .header(X_AWZ_TARGET, "Timestream_20181101.DescribeEndpoints")
        .header(X_AMZ_CONTENT_SHA256, &canonical_request.content_sha_256)
        .header(
            AUTHORIZATION,
            &canonical_request
                .calculate_authorization()
                .expect("Authorization creation failed"),
        )
        .body(body)
        .send()
        .await
        .expect("Service error")
        .text()
        .await
        .unwrap();

    println!("{:?}", endpoint);
}
