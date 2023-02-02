use crate::app::{State, config::Config};

use super::Command;


#[derive(Clone)]
pub struct NullCommand {

}

impl NullCommand {
    pub fn new() -> Self {
        Self{}
    }
}

impl Command for NullCommand {
    fn handle(&self, _state: &mut State, _config: &mut Config, _command: &str, _args: &[&str]) {
    }

    fn get_name(&self) -> String {
        return "null".to_string();
    }

    fn get_help(&self) -> String {
        return "This command does nothing.".to_string();
    }

    fn get_subcommands(&self) -> Vec<Box<dyn Command>> {
        vec![]
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}