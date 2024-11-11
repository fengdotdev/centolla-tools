


pub struct Task {
    pub command: String,
}


impl Task {
    pub fn new(command: String) -> Task {
        Task {
            command,
        }
    }
    pub fn run(&self,tasker: &Tasker) {
        // execute the command in the shell

        tasker.run();
    }
}