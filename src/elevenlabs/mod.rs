pub mod requests;
pub mod responses;
pub mod types;

use bytes::Bytes;
use responses::VoiceList;
use serde::Serialize;
use types::DEFAULT_OUTPUT_FORMAT;

type Error = Box<dyn std::error::Error + Send + Sync>;

const API_BASE: &str = "https://api.elevenlabs.io/";

pub struct ElevenLabs {
    api_key: String,
    client: reqwest::Client,
}

impl serenity::prelude::TypeMapKey for ElevenLabs {
    type Value = ElevenLabs;
}

impl ElevenLabs {
    pub fn new_from_key(api_key: String) -> Self {
        let client = reqwest::Client::new();
        Self { api_key, client }
    }

    fn get_base_request(
        &self,
        endpoint: &str,
        query: Vec<(&str, &str)>,
    ) -> reqwest::RequestBuilder {
        self.client
            .get(format!("{}{}", API_BASE, endpoint))
            .header("xi-api-key", &self.api_key)
            .query(&query)
    }

    fn post_base_request(
        &self,
        endpoint: &str,
        query: Vec<(&str, &str)>,
    ) -> reqwest::RequestBuilder {
        self.client
            .post(format!("{}{}", API_BASE, endpoint))
            .header("xi-api-key", &self.api_key)
            .query(&query)
    }

    async fn execute_request_json_response<ResultType: serde::de::DeserializeOwned>(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<ResultType, Error> {
        let built_req = req.build()?;
        println!("Sending request to {}", built_req.url());
        let resp = self.client.execute(built_req).await?;
        let status = resp.status();
        let text = resp.text().await?;
        if status.is_success() {
            // Successful HTTP request, parse json here...
            let parsed = serde_json::from_str::<ResultType>(&text);
            match parsed {
                Ok(p) => Ok(p),
                Err(e) => {
                    // Parsing error
                    println!("Failed to parse body! {text}");
                    Err(e)?
                }
            }
        } else {
            // HTTP error here...
            println!("HTTP request failed! Body: {text}");
            Err(format!("HTTP {}", status.as_str()))?
        }
    }

    async fn execute_request_cursor_response(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<impl futures_core::Stream<Item = reqwest::Result<Bytes>>, Error> {
        let built_req = req.build()?;
        println!("Sending request (getting raw body) to {}", built_req.url());
        let resp = self.client.execute(built_req).await?;
        let status = resp.status();
        if status.is_success() {
            Ok(resp.bytes_stream())
        } else {
            // HTTP error here...
            let text = resp.text().await?;
            println!("HTTP request failed! Body: {text}");
            Err(format!("HTTP {}", status.as_str()))?
        }
    }

    async fn run_json_request_with_body<
        ResultType: serde::de::DeserializeOwned,
        BodyType: Serialize + Sized,
    >(
        &self,
        base_req: reqwest::RequestBuilder,
        body: Option<BodyType>,
    ) -> Result<ResultType, Error> {
        self.execute_request_json_response(base_req.json(&body))
            .await
    }

    async fn run_json_request_no_body<ResultType: serde::de::DeserializeOwned>(
        &self,
        base_req: reqwest::RequestBuilder,
    ) -> Result<ResultType, Error> {
        self.execute_request_json_response(base_req).await
    }

    async fn run_cursor_request_with_body<BodyType: Serialize + Sized>(
        &self,
        base_req: reqwest::RequestBuilder,
        body: Option<BodyType>,
    ) -> Result<impl futures_core::Stream<Item = reqwest::Result<Bytes>>, Error> {
        self.execute_request_cursor_response(base_req.json(&body))
            .await
    }

    async fn run_cursor_request_no_body(
        &self,
        base_req: reqwest::RequestBuilder,
    ) -> Result<impl futures_core::Stream<Item = reqwest::Result<Bytes>>, Error> {
        self.execute_request_cursor_response(base_req).await
    }

    #[allow(dead_code)]
    pub async fn get_voice_list(&self) -> Result<VoiceList, Error> {
        self.run_json_request_no_body(self.get_base_request("v2/voices", Vec::new()))
            .await
    }

    #[allow(dead_code)]
    pub async fn generate_voice(
        &self,
        voice_id: String,
        text: String,
        media_format: Option<&types::OutputFormat>,
    ) -> Result<impl futures_core::Stream<Item = reqwest::Result<Bytes>>, Error> {
        let final_format = match media_format {
            Some(format) => format,
            None => {
                let format = DEFAULT_OUTPUT_FORMAT;
                println!(
                    "No media format provided, using default {}",
                    format.to_string()
                );
                format
            }
        };
        let mut query = Vec::new();
        query.push(("output_format", final_format.to_string()));

        self.run_cursor_request_with_body(
            self.post_base_request(&format!("v1/text-to-speech/{}", voice_id), Vec::new()),
            Some(requests::CreateSpeechRequest {
                text,
                model_id: None,
                voice_settings: None,
            }),
        )
        .await
    }
}
