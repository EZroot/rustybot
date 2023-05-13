use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct WolframAlphaQuery {
    input: String,
    appid: String,
    output: String,
}

#[derive(Deserialize)]
struct WolframAlphaResponse {
    queryresult: QueryResult,
}

#[derive(Deserialize)]
struct QueryResult {
    pods: Vec<Pod>,
}

#[derive(Deserialize)]
struct Pod {
    subpods: Vec<SubPod>,
}

#[derive(Deserialize)]
struct SubPod {
    plaintext: Option<String>,
}

pub async fn query(input: String) -> String {
    let query = WolframAlphaQuery {
        input: input,
        appid: "QWQHV9-GH5XTP2Q45".to_string(),
        output: "json".to_string(),
    };

    let response: WolframAlphaResponse = reqwest::Client::new()
        .get("http://api.wolframalpha.com/v2/query")
        .query(&query)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let distance = response
        .queryresult
        .pods
        .iter()
        .find(|pod| pod.subpods.iter().any(|subpod| subpod.plaintext.is_some()))
        .and_then(|pod| pod.subpods.iter().find_map(|subpod| subpod.plaintext.as_ref()))
        .unwrap();

    format!("Distance from moon to earth: {}", distance)
}
