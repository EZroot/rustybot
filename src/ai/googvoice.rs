use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
struct TextToSpeechInput {
    text: String,
}

#[derive(Debug, Serialize)]
struct TextToSpeechVoice {
    language_code: String,
    ssml_gender: String,
}

#[derive(Debug, Serialize)]
struct TextToSpeechAudioConfig {
    audio_encoding: String,
}

#[derive(Debug, Serialize)]
struct TextToSpeechRequest {
    input: TextToSpeechInput,
    voice: TextToSpeechVoice,
    audio_config: TextToSpeechAudioConfig,
}

#[derive(Debug, Deserialize)]
struct TextToSpeechResponse {
    audio_content: String,
}

pub async fn list_voices() -> Result<(), Box<dyn Error>> {
    let url = "http://localhost:3000/listVoices";
    let response = reqwest::get(url).await?.text().await?;
    println!("{}", response);
    Ok(())
}

pub async fn synthesize_text(text: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("http://localhost:3000/synthesize?text={}&outputFile={}", text, output_file);
    let response = reqwest::get(&url).await?.text().await?;
    println!("{}", response);

    Ok(())
}


pub async fn synthesize_bytes(text: &str, voice_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "http://localhost:3000/synthesizebytes?text={}&voiceName={}",
        text, voice_name
    );
    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?;
    println!("{:02X?}", bytes);
    Ok(())
}
