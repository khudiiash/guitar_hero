use std::{collections::HashMap, fs};
use::bevy::prelude::*;

use crate::song::resources::*; 

const SPOTS: usize = 6;

pub fn parse_song_data(
    mut song: ResMut<Song>,
    song_choice: Res<SongChoice>, 
) {
    println!("Parsing song data");
    let contents = fs::read_to_string(format!("assets/songs/{}/{}.txt", song_choice.artist, song_choice.title)).expect("Should have been able to read the file");
    let data: Vec<&str> = contents.lines().collect();

    let mut riffs: HashMap<String, Vec<String>> = HashMap::new(); 
    let mut parts: HashMap<String, Vec<String>> = HashMap::new();
    
    for (i, line) in data.iter().enumerate() {
        if line.starts_with("|") || line.is_empty() {
            continue;
        }
        let mut split = line.split(": ");
        let header = split.next().unwrap();
        let content = match split.next() {
            Some(content) => content.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>(), 
            None => "".to_string(),
        };

        //init song map
        for _ in 0..SPOTS {
            song.map.push("".to_string());
        }

        match header {
            "title" => song.title = content.to_string(), 
            "artist" => song.artist = content.to_string(),
            "bpm" => song.bpm = content.parse::<u32>().unwrap(), 
            "delay" => song.delay = content.parse::<f32>().unwrap(),
            "beats" => song.beats = content.parse::<u32>().unwrap(),
            "riff" => {
                let riff_name = content.to_string();
                let mut riff_lines: Vec<String> = Vec::new();
                for (_, line) in data[i+1..i+7].iter().enumerate() {
                    riff_lines.push(line.to_string());
                }
                riffs.insert(riff_name, riff_lines);
            } 
            "part" => {
                let part_name = content.to_string();
                let next_line = data[i+1];
                let part_riffs = next_line.split_whitespace().collect::<Vec<&str>>();
                let mut part_lines: Vec<String> = Vec::new();
                for _ in 0..6 {
                    part_lines.push("".to_string());
                }

                for riff_name in part_riffs.iter() {
                    let riff_lines = riffs.get(riff_name.clone()).unwrap();
                    for (j, line) in riff_lines.iter().enumerate() {
                        part_lines[j] += line;
                    }
                }
               parts.insert(part_name, part_lines);
            },
            "song" => {
               let next_line = data[i+1];
               let song_parts = next_line.split_whitespace().collect::<Vec<&str>>();
               for part_name in song_parts.iter() {
                   let part_lines = parts.get(part_name.clone()).unwrap();
                   for (j, line) in part_lines.iter().enumerate() {
                       song.map[j] += line;
                   }
               }  
            }
            _ => {}
        }
    }
    song.speed = 360. / song.bpm as f32;
    song.spacing = (60., 60. * song.speed);
}
