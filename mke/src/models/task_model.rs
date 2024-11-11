use crate::interfaces::tasker_interface::TaskerInterface;

pub struct TaskModel {
    pub command: String,
    tasker: Box<dyn TaskerInterface>,
}

impl TaskModel {
    pub fn new(command: String, tasker: Box<dyn TaskerInterface>) -> TaskModel {
        TaskModel { command, tasker }
    }
    pub fn run(&self) {
        let command: String = self.command.clone();
        self.tasker.execute_command(&command);
    }
}
