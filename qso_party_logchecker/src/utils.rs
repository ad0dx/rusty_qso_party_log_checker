

    use std::process;

    pub fn exit_process(retcode: i32) -> () {
        process::exit(retcode);
    }
