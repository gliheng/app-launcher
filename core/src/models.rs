use std::{io::Read, collections::HashMap};
use serde::{Serialize, Deserialize};
use nanoid::nanoid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub cmds: Vec<String>,
}

#[derive(Debug)]
pub struct User {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub template: Workflow,
    pub git: String,
    pub workflow: String,
    pub workflow_args: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Workflow {
    pub cmds: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputParams {
    pub task_id: Option<String>,
    pub project: Project,
}

#[derive(Debug)]
pub struct Context {
    pub task_id: String,
    pub user: User,
    pub project: Project,
}

impl Context {
    pub fn from_stdin() -> Self {
        let mut input = String::new();
        let mut stdin = std::io::stdin();
        stdin.read_to_string(&mut input).unwrap();
        let params: InputParams = serde_yaml::from_str(&input).unwrap();

        let user = User { token: "".to_owned() };
        let mut ctx = Self::new(user, params.project);
        if let Some(task_id) = params.task_id {
            ctx.task_id = task_id;
        }
        ctx
    }
    pub fn new(user: User, project: Project) -> Self {
        Self {
            task_id: nanoid!(),
            user: user,
            project: project,
        }
    }
}
