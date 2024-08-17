/// Commands for the CLI Arguments
/// 

use fs::File;
use std::io::prelude::*;
use std::path::Path;

enum Cmds {
    prog,
    verbose,
    yes,
    brief,
    help,

}

impl Cmds {
    pub async fn prog() -> Result<(), Error> {
        // how to run system file 

    }

    pub async fn verbose() -> Result<(), Error> {
        // how to improve verbosity
    }
    
    pub async fn yes_flag() -> Result<(), Error> {
        // how to automatically answer yes to options
    }
    
    pub async fn brief_flag() -> Result<(), Error> {
        // how to print brief help information
    }
    
    pub async fn help_flag() -> Result<(), Error> {
        // how to print brief help information
    }



}

