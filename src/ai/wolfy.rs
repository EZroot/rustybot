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
    #[serde(default)]
    pods: Vec<Pod>,
}

#[derive(Deserialize)]
struct Pod {
    #[serde(default)]
    subpods: Vec<SubPod>,
}

#[derive(Deserialize)]
struct SubPod {
    plaintext: Option<String>,
}

pub async fn query(input: String) -> String {
    let query = WolframAlphaQuery {
        input: input,
        appid: "QWQHV9-J6XYP9G697".to_string(),
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

        let answer = response.queryresult.pods.iter()
        .flat_map(|pod| pod.subpods.iter())
        .filter_map(|subpod| subpod.plaintext.as_ref())
        .map(|text| text.replace("`", ""))
        .map(|text| format!("``` {} ```", text))
        .collect::<String>();

    format!("Sphinx of Knowledge:\n{}", answer)
}
