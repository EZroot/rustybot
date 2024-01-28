use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub command_name: String,
    pub command_args: HashMap<String, u32>, // Each arg has a times_used count
    pub times_used: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_name: String,
    pub commands: Vec<Command>,
    pub level: u32,
    pub experience: u32,
}

pub fn load_or_initialize_stats(file_path: &str) -> io::Result<Vec<User>> {
    if Path::new(file_path).exists() {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new()))
    } else {
        Ok(Vec::new())
    }
}

pub fn save_stats(file_path: &str, users: &[User]) -> io::Result<()> {
    let json = serde_json::to_string(users).unwrap();
    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn add_or_update_user(users: &mut Vec<User>, user_name: &str, command_name: &str, new_command_args: Vec<String>, xp_reward: u32) {
    let user = users.iter_mut().find(|u| u.user_name == user_name);
    match user {
        Some(u) => {
            let command = u.commands.iter_mut().find(|c| c.command_name == command_name);
            match command {
                Some(c) => {
                    c.times_used += 1;
                    for arg in new_command_args {
                        let arg_count = c.command_args.entry(arg).or_insert(0);
                        *arg_count += 1; // Increment the count for each unique arg
                    }
                },
                None => {
                    let mut command_args_map = HashMap::new();
                    for arg in new_command_args {
                        command_args_map.insert(arg, 1);
                    }
                    u.commands.push(Command { command_name: command_name.to_string(), command_args: command_args_map, times_used: 1 });
                },
            }
            u.experience += xp_reward; // Increment XP by the given reward
            while u.experience >= xp_for_level(u.level + 1) {
                u.level += 1; // Level up if experience reaches the threshold
            }
        },
        None => {
            let mut command_args_map = HashMap::new();
            for arg in new_command_args {
                command_args_map.insert(arg, 1);
            }
            users.push(User {
                user_name: user_name.to_string(),
                commands: vec![Command { command_name: command_name.to_string(), command_args: command_args_map, times_used: 1 }],
                level: 1,
                experience: 0,
            });
        }
    }
}

pub fn print_user_stats(users: &[User]) {
    for user in users {
        println!("User: {}, Level: {}, XP: {}/{}", user.user_name, user.level, user.experience, xp_for_level(user.level+1));
        for command in &user.commands {
            println!("Command: {}, Used: {}", command.command_name, command.times_used);
            for (arg, count) in &command.command_args {
                println!("Arg: {}, Used: {}", arg, count);
            }
        }
    }
}

pub fn print_specific_user_stats(user_name: &str) -> io::Result<String> {
    let users = load_or_initialize_stats("stats.json")?;
    let mut result = String::new();

    if let Some(user) = users.iter().find(|u| u.user_name == user_name) {
        result += &format!("## __{}__\n**Lvl: {}\nXP: {}/{}**\n\n__**Stats**__\n", user.user_name.to_uppercase(), user.level, user.experience, xp_for_level(user.level + 1));
        for command in &user.commands {
            result += &format!("- **{} : {}**\n", command.command_name, command.times_used);
            // for (arg, count) in &command.command_args {
            //     result += &format!("Arg: {}, Used: {}\n", arg, count);
            // }
        }
    } else {
        result = format!("## User {} not found", user_name);
    }

    Ok(result)
}

pub fn xp_for_level(level: u32) -> u32 {
    let mut xp = 0;
    for lvl in 1..=level {
        xp += (lvl as f64 + 300.0 * 2f64.powf(lvl as f64 / 7.0)) as u32;
    }
    xp / 4 // The division by 4 adjusts the XP to match RuneScape's formula
}
