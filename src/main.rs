use std::io::{self, Write};
use std::fs;
use std::collections::HashMap;

use serde::Deserialize;


#[derive(Debug, Deserialize )]
struct Scene {
    description: String,
    choices: HashMap<String, String>
}

type Story = HashMap<String, Scene>;

fn main(){
    let story_data = fs::read_to_string("story.json").expect("Unable to read story file");
    let story: Story = serde_json::from_str(&story_data).expect("Invalid story format");

    let mut current_scene_key  = String::from("start");
    let mut history: Vec<String> = Vec::new();

    loop {
        let scene = &story[&current_scene_key];
        println!("{}", scene.description);

        if scene.choices.is_empty(){
            println!("The End");
            break;
        }

        let options: Vec<(&String, &String)> = scene.choices.iter().collect();
        for(index, (choice_text, _target)) in options.iter().enumerate() {
            println!("{}) {}", index + 1, choice_text);
        }

        if !history.is_empty() {
            println!("{}) Go Back", options.len() + 1);
        }

        print!(">");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let choice_index = match input.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= options.len() + 1 => num - 1,
            _ => {
                println!("Invalid Choice! Please enter a number between 1 and {}", options.len());
                continue;
            }
        };
        

        if choice_index == options.len() {
            match history.pop() {
                Some(previous_scene) => {
                    current_scene_key = previous_scene;
                    continue;
                }
                None => {
                    println!("No previous scene to go back to!");
                    continue;
                }
            }
            
        }

        history.push(current_scene_key.clone());

        let (_choice_text, target_scene_key) = options[choice_index];
        current_scene_key = target_scene_key.clone();

    }

}