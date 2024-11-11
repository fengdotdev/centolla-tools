use crate::models::task_model::TaskModel;

pub struct MakefileModel {
    pub tasks: Vec<MakeFileCommand>,
}

impl MakefileModel {
    pub fn new() -> MakefileModel {
        MakefileModel { tasks: Vec::new() }
    }
    pub fn add_task(&mut self, task: MakeFileCommand) {
        self.tasks.push(task);
    }

    pub fn run(&self, invoke_cmd: &str) {
        for task in &self.tasks {
            if task.invoke_cmd == invoke_cmd {
                task.task.run();
            }
        }
    }
}

pub struct MakeFileCommand {
    pub invoke_cmd: String,
    pub task: Box<TaskModel>,
}
