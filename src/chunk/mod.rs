use anyhow::Result;
// use pandoc_types::definition::{Inline, *};
use std::collections::VecDeque;
use std::path::Path;

pub const TOK_SEP: char = '𐑙';

pub mod pandoc_ast;

pub trait AST {
    fn import(&mut self, filepath: &Path) -> Result<()>;
    fn to_mipcs(&self) -> Tasks;
    fn apply_mipcs(&mut self, mipcs: Tasks) -> Result<()>;
    fn export(&self, filepath: &Path) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct Tasks {
    pub main: VecDeque<String>,
    pub sides: VecDeque<String>,
}

#[derive(Debug, Clone)]
pub enum TaskType {
    Main,
    Side,
}

impl Tasks {
    fn new() -> Self {
        Tasks {
            main: VecDeque::new(),
            sides: VecDeque::new(),
        }
    }
    fn add(&mut self, str: String, task_type: TaskType) {
        match task_type {
            TaskType::Main => self.main.push_back(str),
            TaskType::Side => self.sides.push_back(str),
        }
    }

    fn collect(&mut self, task_type: TaskType) -> String {
        match task_type {
            TaskType::Main => self.main.pop_front(),
            TaskType::Side => self.sides.pop_front(),
        }
        .expect("mismatch")
    }
}
