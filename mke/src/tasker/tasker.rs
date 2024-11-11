use crate::interfaces::tasker_interface::TaskerInterface;
pub struct TaskerDebug {}

impl TaskerInterface for TaskerDebug {
    fn execute_command(&self, command: &str) {
        // execute the command in the shell
        println!("Executing command: {}", command);
    }
}
