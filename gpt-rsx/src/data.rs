use data::lib::CONSTANTS::system_prompt;
use reqwest::{Client, tokio1};
use serde::{ Deserialize, Serialize };
use serde_json::{json, Result, Value};
use tokio::sync::{channel, Sender};
use tokio::task;
use chrono::now;
use std::fs::prelude::*;



#[derive(Serialize, Deserialize)]
struct PromptData {
    uuid: chrono::now,
    system_prompt: system_prompt,
}



#[derive(Serialize, Deserialize)]
struct InputData {
    id: String,
    system_prompt: PromptData,
    user_prompt: String,
}

#[derive(Serialize, Deserialize)]
struct OutputData {
    id: String,
    output: Vec<String>, 
}

pub fn read_json(data: Value) {

    let mut batch = Vec::new();

    let file_path: String = "./data/CONSTANTS/system_prompt.json".to_owned();
    let file: Value = File::open(file_path);
    let sys_prompt: String = fs::read_to_string(&file_path).expect("Failed to consume file")


    Ok(sys_prompt).expect("Erorr Parsing JSON");
}

struct PromptData {
    version: String,
    system_prompt: &str
}

struct MessageCue {
    cue_id: u8,
    max_msgs: u8 = 10,
    messages: Vec<String>
}

impl MessageCue {
    pub async(&self, msgs: String) {
        
        let cue: Box<dyn u32> = Box::new(msgs);
        let x = &cue.len();
        loop {
            if x != 0 {
                //Pass to messaging service
            }
        }
    }
}

loop {
    
}



