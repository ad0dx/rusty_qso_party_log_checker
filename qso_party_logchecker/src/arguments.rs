    
    use crate::file_utils::file_exists;

    // Process the command line arguments and return the name of the config file
    // If an error occurs (config file not provided) then the process exits
    // If the config file does not exist, issue an error and exit the process 
    pub fn process_arguments(args: &Vec<String>) -> Result<String, i32> {
   
        let nargs = args.len();
        println!("Number of arguments: {}", nargs);
        let mut i = 1;
        for s in args {
            println!("{}) {}", i, s);
            i += 1;
        }
    
        if nargs > 2 {
            println!("Error: Too many arguments {}", nargs);
        }
    
        if nargs != 2 {
            println!("Usage: qso_party_logchecker config.txt");
            println!("       cargo run config.txt");

            return Err(1);
        }
        
        let config = &args[1];

        let exists: bool = file_exists(config);

        // if the configuration file does not exist, issue an error and exit the process
        if false == exists {
            println!("Error: File {} does not exist :-(", config);

            return Err(2);
        }

        Ok(config.to_string())
    }
