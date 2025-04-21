use std::io::{self, Write};
use std::fs;
use std::collections::HashMap;

use serde::Deserialize;


#[derive(Debug, Deserialize )]
struct Scene {
    description: String,
    set: Option<Vec<String>>,
    unset: Option<Vec<String>>,
    choices: Option<HashMap<String, String>>,
    check: Option<HashMap<String, String>>
}

type Story = HashMap<String, Scene>;

fn main(){
    let story_data = fs::read_to_string("story.json").expect("Unable to read story file");
    let story: Story = serde_json::from_str(&story_data).expect("Invalid story format");

    let mut current_scene_key  = String::from("start");
    let mut history: Vec<String> = Vec::new();

    let mut variables = std::collections::HashSet::<String>::new();

    loop {
        let scene = &story[&current_scene_key];
        println!("{}", scene.description);

        if current_scene_key == "end" {
            break;
        }

        // keeps track what things you got ["first_collected_thing", "second_collected_thing"]
        if let Some(set) = &scene.set {
            for s in set {
                variables.insert(s.clone());
            }
        }

        if let Some(unset) = &scene.unset {
            for u in unset {
                variables.remove(&u.clone());
                println!("removing from variables {}", &u.clone())
            }
        }


        if scene.choices.as_ref().is_none() && scene.check.as_ref().is_none(){
            println!("The End");
            break;
        }

        // Options for a scene ["first key of choice", "second key of choice"]
        let options: Vec<(&String, &String)> = scene.choices.as_ref().map(|choices| choices.iter().collect()).unwrap_or_default();
        for(index, (choice_text, _target)) in options.iter().enumerate() {
            println!("{}) {}", index + 1, choice_text);
        }


        if let Some(checks) = &scene.check {
            let mut condition_met = false;

            for (check_key, target) in checks.iter() {
                if check_key != "else" && variables.contains(check_key) {
                    current_scene_key = target.clone();
                    condition_met = true;
                }
            }
            

            if !condition_met {
                if let Some(else_target) = checks.get("else") {
                    current_scene_key = else_target.clone();
                }
            }

            continue;
        }
        

        if !history.is_empty() {
            println!("{}) Go Back", options.len() + 1);
        }

        print!("> ");
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