use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct YouTubeSong {
    pub title: String,
    pub url: String,
    pub priority: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SongList {
    pub songs: HashMap<String, YouTubeSong>, // Key is a combination of title and url
}

pub fn load_or_initialize_songs(file_path: &str) -> io::Result<SongList> {
    if Path::new(file_path).exists() {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(serde_json::from_str(&contents).unwrap_or_else(|_| SongList { songs: HashMap::new() }))
    } else {
        Ok(SongList { songs: HashMap::new() })
    }
}

pub fn save_songs(file_path: &str, song_list: &SongList) -> io::Result<()> {
    let json = serde_json::to_string(song_list)?;
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn add_or_update_song(song_list: &mut SongList, title: &str, url: &str) {
    let key = format!("{}|{}", title, url);
    let song = song_list.songs.entry(key.clone()).or_insert(YouTubeSong { title: title.to_string(), url: url.to_string(), priority: 0 });
    song.priority += 1;
}

pub fn display_top_priority_songs(song_list: &SongList) -> String {
    let mut songs: Vec<&YouTubeSong> = song_list.songs.values().collect();
    songs.sort_by(|a, b| b.priority.cmp(&a.priority));

    let mut output = String::new();
    let mut counter = 0;
    for song in songs.iter().take(10) {
        counter+=1;
        output.push_str(&format!("#{} Title: {}, URL: <{}>, Count: {}\n", counter, song.title, song.url, song.priority));
    }

    output
}
