


pub struct Makefile {
    pub tasks: Vec<MakeFileCommand>,
}





pub struct MakeFileCommand {
    pub invoke_cmd: String,
    pub task: Task,
}