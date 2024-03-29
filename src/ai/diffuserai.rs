use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::string;

#[derive(Debug)]
pub enum ImageFilter {
    NoFilter = 0,
    RandomNoise = 1,
    GaussainNoise = 2,
    RandomNoiseFromPallette = 3,
}

impl From<ImageFilter> for i32 {
    fn from(filter: ImageFilter) -> Self {
        match filter {
            ImageFilter::NoFilter => 0,
            ImageFilter::RandomNoise => 1,
            ImageFilter::GaussainNoise => 2,
            ImageFilter::RandomNoiseFromPallette => 3,
        }
    }
}

impl From<i32> for ImageFilter {
    fn from(value: i32) -> Self {
        match value {
            0 => ImageFilter::NoFilter,
            1 => ImageFilter::RandomNoise,
            2 => ImageFilter::GaussainNoise,
            3 => ImageFilter::RandomNoiseFromPallette,
            _ => panic!("Invalid filter value"),
        }
    }
}

#[derive(Debug, Serialize)]
struct AIDiffuserRequest {
    prompt: String,
    height: i32,
    width: i32,
    num_inference_steps: i32,
    guidance_scale: f32,
    use_columns: bool,
}

#[derive(Debug, Deserialize)]
struct AIDiffuserResponse {
    image_path: String,
}

pub async fn generate_stable_diffuse_image(
    prompt: &str,
    negative_prompt: &str,
    height: i32,
    width: i32,
    num_inference_steps: i32,
    img_count: i32,
    use_columns: bool,
    first_image_noise: i32,
    image_filter_enum: ImageFilter,
    upscaled_image_generation_strength: f64,
    upscale_original_bool: bool,
) -> Result<String, Box<dyn Error>> {
    let mut url = String::new();
    let first_image_generation_strength = 1;
    let num_inference_steps = num_inference_steps;
    println!("Current Upscale Bool: {}", upscale_original_bool);
    let image_filter_enum_value: i32 = image_filter_enum.into();
    //squared
    let width_as_f32:f32= width as f32;
    let height_as_f32:f32= height as f32;
    
    let upscaled_width = (width_as_f32 as i32);// + 256;//* 1.5) as i32; //if this isnt a multiple of 8 this will throw a error
    let upscaled_height = (height_as_f32 as i32);// + 256;//* 1.5) as i32;

    if upscale_original_bool == false {
    let chunk_size = 32;
    let blur_size = 16;
    let edge_blur_size = 64; //was 32
        url = format!(
            "http://localhost:6969/stablediffusion?prompt={}&height={}&width={}&num_inference_steps={}&img_count={}&use_columns={}&negative_prompt={}&first_image_strength={}&resized_image_strength={}&chunk_size={}&blur_radius={}&edge_radius={}&upscaled_size_width={}&upscaled_size_height={}&first_image_noise={}&image_filter_enum={}&upscale_original_bool={}",
            prompt, height, width, num_inference_steps,img_count,use_columns, negative_prompt, first_image_generation_strength, upscaled_image_generation_strength, chunk_size,blur_size,edge_blur_size,upscaled_width,upscaled_height,first_image_noise,image_filter_enum_value, upscale_original_bool
            );
    } else {
    let chunk_size = 1;
    let blur_size = 1;
    let edge_blur_size = 1; //was 32
        url = format!(
                "http://localhost:6969/stablediffusion?prompt={}&height={}&width={}&num_inference_steps={}&img_count={}&use_columns={}&negative_prompt={}&first_image_strength={}&resized_image_strength={}&chunk_size={}&blur_radius={}&edge_radius={}&upscaled_size_width={}&upscaled_size_height={}&first_image_noise={}&image_filter_enum={}",
                prompt, height, width, num_inference_steps,img_count,use_columns, negative_prompt, first_image_generation_strength, upscaled_image_generation_strength, chunk_size,blur_size,edge_blur_size,upscaled_width,upscaled_height,first_image_noise,image_filter_enum_value
                );
    }
    println!("Sent Url: {}", url);

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;
    if !response.status().is_success() {
        return Err(format!(
            "Request to {} failed with status code: {}",
            url,
            response.status()
        )
        .into());
    }

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to retrieve response body: {}", e))?;
    let json: Value =
        serde_json::from_str(&response_text).map_err(|e| format!("Failed to parse JSON: {}", e))?;
    println!("Json: {:?}", &json);

    let image_path = json["image_path"]
        .as_str()
        .ok_or("image_path not found or not a string")?;
    println!("image_path: {}", &image_path);
    let win_path = image_path.replace("/mnt/c/", "C:/");
    println!("Windows Path: {:?}", win_path);
    let filtered_path = win_path;
    println!("filtered_path: {}", &filtered_path);
    Ok(filtered_path)
}

fn get_image_path(json_str: &str) -> Option<String> {
    // Parse the JSON string
    let parsed: Result<Value, _> = serde_json::from_str(json_str);
    match parsed {
        Ok(Value::Object(obj)) => {
            // Check if the object contains an "image_path" key
            match obj.get("image_path") {
                Some(Value::String(path)) => Some(path.to_string()),
                _ => None,
            }
        }
        _ => None,
    }
}

fn filter_path(linux_path: &str) -> Option<String> {
    if let Some(index) = linux_path.rfind('/') {
        if let Some(filename) = linux_path.get(index + 1..) {
            return Some(format!("./artificialintelligence/gen_pics/{}", filename));
        }
    }
    None
}

fn convert_to_windows_path(linux_path: &str) -> String {
    let path = Path::new(linux_path);
    let mut buf = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::RootDir => buf.push("\\"),
            std::path::Component::Normal(os_str) => {
                if let Some(s) = os_str.to_str() {
                    buf.push(s);
                }
            }
            _ => (),
        }
    }
    buf.to_string_lossy().to_string()
}
