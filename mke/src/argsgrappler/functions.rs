

//// take the args from the command line
pub fn args_grappler()->Vec<String> {
   // take the args from the command line
   let args: Vec<String> = std::env::args().collect();

   // check if the user has provided the correct number of arguments
   if args.len() != 2 {
       eprintln!("Usage: {} <number>", args[0]);
       std::process::exit(1);
   }

    return args;
   
}

//// pick the second argument from the args as the invoke command
pub fn invoke_command_picker()->String{
   let args = args_grappler();
    // pick the second argument from the args as the command to run
    let command = &args[1];
    return command.to_string();
}