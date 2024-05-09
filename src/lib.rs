use colored::*;

pub struct Rustbar {
    bar_content: String,
    runner: Runner,
    commands: Vec<String>,
    decoy_time: f64,
    label: String,
    color: Color,
}

pub enum Color {
    Red,
    Yellow,
    Blue,
    Black,
    Green,
    BrightGreen,
    BrightYellow,
    BrightRed,
    BrightBlue,
    BrightBlack,
}

pub enum Runner {
    Shell,
    Rust,
    Default, // is shell
    Decoy,
}

impl Runner {
    pub fn clone(&self) -> Self {
        let x = match self {
            Runner::Decoy => Runner::Decoy,
            Runner::Default => Runner::Default,
            Runner::Rust => Runner::Rust,
            Runner::Shell => Runner::Shell,
        };

        x
    }
}

impl Rustbar {
    pub fn new() -> Self {
        Rustbar{bar_content:String::from("▓"), runner:Runner::Default, commands:Vec::new(), decoy_time:0.1, label:"Progress".to_string(), color:Color::Yellow}
    }

    pub fn runner(self, runner: Runner) -> Self{
        Self{
            bar_content: self.bar_content.clone(),
            runner,
            commands: self.commands.clone(),
            decoy_time: self.decoy_time.clone(),
        }
    }

    pub fn add_command(&mut self, command: &str) -> Self{
        let mut cmd = self.commands.clone();
        cmd.push(command.to_string());
        Self {
            bar_content:self.bar_content.clone(),
            runner:self.runner.clone(),
            commands:cmd,
            decoy_time:self.decoy_time.clone(),
        }
    }

    pub fn decoy(&mut self, decoy_time: f64) -> Self {
        Self {
            bar_content:self.bar_content.clone(),
            runner:self.runner.clone(),
            commands:self.commands.clone(),
            decoy_time:decoy_time,
        }
    }

    pub fn run(&self) -> Self {
        let x = match self.runner {
            Runner::Decoy => "decoy".to_string(),
            Runner::Default => "default".to_string(),
            Runner::Rust => "rust".to_string(),
            Runner::Shell => "shell".to_string(),
        };

        if x == "decoy" {
            for y in 0..=100 {
                print
            }
        }

        Self
    }

}