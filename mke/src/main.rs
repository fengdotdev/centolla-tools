mod argsgrappler;
mod interfaces;
mod models;
mod tasker;

fn main() {
    let invoke_command: String = argsgrappler::functions::invoke_command_picker();

    // search

    // read the command
    println!("Running command: {}", invoke_command);
}
