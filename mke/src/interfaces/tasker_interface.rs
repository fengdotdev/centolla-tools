pub trait TaskerInterface {
    fn execute_command(&self, string: &str);
}
