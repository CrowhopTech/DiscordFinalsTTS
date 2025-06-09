pub mod media;
pub mod requests;
pub mod responses;
pub mod types;

use bytes::Bytes;
use log::{debug, error, info};
use media::DEFAULT_OUTPUT_FORMAT;
use responses::{UserInfo, VoiceList};
use serde::Serialize;
use types::VoiceSettings;
use chrono::DateTime;

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

    #[allow(dead_code)]
    async fn execute_request_json_response<ResultType: serde::de::DeserializeOwned>(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<ResultType, Error> {
        let built_req = req.build()?;
        debug!(url = built_req.url().as_str(); "Sending request");
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
                    error!(error = e.to_string().as_str(), text = text.as_str(); "Failed to parse response");
                    Err(e)?
                }
            }
        } else {
            // HTTP error here...
            error!(response_body = text.as_str(), status = status.as_str(); "HTTP request failed");
            Err(format!("Error HTTP {}: {}", status.as_str(), text))?
        }
    }

    #[allow(dead_code)]
    async fn execute_request_cursor_response(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<impl futures_core::Stream<Item = reqwest::Result<Bytes>>, Error> {
        let built_req = req.build()?;
        debug!(url = built_req.url().as_str(); "Sending request (getting raw body)");
        let resp = self.client.execute(built_req).await?;
        let status = resp.status();
        if status.is_success() {
            Ok(resp.bytes_stream())
        } else {
            // HTTP error here...
            let text = resp.text().await?;
            error!(response_body = text.as_str(), status = status.as_str(); "HTTP request failed");
            Err(format!("Error HTTP {}: {}", status.as_str(), text))?
        }
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    async fn run_json_request_no_body<ResultType: serde::de::DeserializeOwned>(
        &self,
        base_req: reqwest::RequestBuilder,
    ) -> Result<ResultType, Error> {
        self.execute_request_json_response(base_req).await
    }

    #[allow(dead_code)]
    async fn run_cursor_request_with_body<BodyType: Serialize + Sized>(
        &self,
        base_req: reqwest::RequestBuilder,
        body: Option<BodyType>,
    ) -> Result<impl futures_core::Stream<Item = reqwest::Result<Bytes>>, Error> {
        self.execute_request_cursor_response(base_req.json(&body))
            .await
    }

    #[allow(dead_code)]
    async fn run_cursor_request_no_body(
        &self,
        base_req: reqwest::RequestBuilder,
    ) -> Result<impl futures_core::Stream<Item = reqwest::Result<Bytes>>, Error> {
        self.execute_request_cursor_response(base_req).await
    }

    pub async fn get_usage(&self) -> Result<(i64, i64, Option<DateTime<chrono::Utc>>), Error> {
        let user_info = self
            .run_json_request_no_body::<UserInfo>(self.get_base_request("v1/user", Vec::new()))
            .await?;

        Ok((
            user_info.subscription.character_count,
            user_info.subscription.character_limit,
            chrono::DateTime::from_timestamp(user_info.subscription.next_character_count_reset_unix, 0),
        ))
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
        voice_settings: Option<VoiceSettings>,
        media_format: Option<&media::OutputFormat>,
    ) -> Result<impl futures_core::Stream<Item = reqwest::Result<Bytes>>, Error> {
        let final_format = match media_format {
            Some(format) => format,
            None => {
                let format = DEFAULT_OUTPUT_FORMAT;
                info!(
                    "No media format provided, using default {}",
                    format.to_string()
                );
                format
            }
        };
        let mut query = Vec::new();
        query.push(("output_format", final_format.to_string()));

        info!(
            voice_id = voice_id.as_str(), text = text.as_str();
            "Generating voice with options {:?}", voice_settings
        );

        self.run_cursor_request_with_body(
            self.post_base_request(&format!("v1/text-to-speech/{}", voice_id), Vec::new()),
            Some(requests::CreateSpeechRequest {
                text,
                voice_settings,
                model_id: None,
            }),
        )
        .await
    }
}
