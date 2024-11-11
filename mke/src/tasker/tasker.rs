pub trait Tasker {
    fn run(&self);
}



pub impl Tasker for Task {
    fn run(&self) {
        // execute the command in the shell
        println!("Running command: {}", self.fdfdsfd);
    }
}



