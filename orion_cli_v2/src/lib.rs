#![allow(clippy::cognitive_complexity)]
//!
//!       ____   _____   _____  ____   __
//!/ __ \ |  __ \ |_   _|/ __ \ | \ | |
//!| |  | || |__) |  | | | |  | ||  \| |
//!| |  | ||  _  /   | | | |  | || . ` |
//!| |__| || | \ \  _| |_| |__| || |\  |
//!\____/ |_|  \_\|_____|\____/ |_| \_|
//!_____  _       _____
//! / ____|| |     |_   _|
//!| |     | |       | |
//!| |     | |       | |
//!| |____ | |____  _| |_
//! \_____||______||_____|
//! 

//! # ORION CLI Custom Project
//! **Welcoming people to the Orion ecosystem**
//! 
//! ## Features:
//! - Import into any library or project
//! - Run your main function from the CLI
//! - More to come this is mostly an experimental project
//! 
//! Example: 
//! use yourcrate::your_module::main;
//! use orion_cli::orion_cli::main;
//! 
mod utils;
mod cmds;
mod logger;


use clap::{arg, command};
use utils::display_welcome;;
use cmds::*;

logger::init();




fn main() {
    log::info("Running the main function!");
    display_welcome();
    let matches = command!()
        .arg(arg!(<prog> "Archive to extract").required(true)) // Positional argument
        .arg(arg!(--verbose -v "Increase output verbosity").takes_value(false)) // Flag for verbosity
        .arg(arg!(--yes -y "Automatically say yes to everything").takes_value(false)) // Flag for yes
        .arg(arg!(--brief -b "Suppress standard output").takes_value(false)) // Flag for brief output
        .arg(arg!(--help -h "Provide help information").takes_value(false)) // Help flag (usually handled by default)
        .arg(arg!(--strip -s "Strip the first component of the archive").takes_value(false)) // Strip flag
        .get_matches();

    let prog = matches.get_one::<String>("prog").expect("Required argument 'prog' not provided");
    let verbose = matches.get_flag("verbose");
    let yes = matches.get_flag("yes");
    let brief = matches.get_flag("brief");
    let help = matches.get_flag("help");
    let _strip = usize::from(matches.get_flag("strip"));
    let _to = matches.get_one::<String>("out").expect("required");
}
    
    
    async fn flag_logic() -> Result<(), Error> {
        
        /// If logic to determine flag usage
        if verbose {
            println!("Verbose mode enabled.");
        }
        
        if yes {
            println!("All prompts will automatically be answered with 'yes'.");
        }
        
        if brief {
            println!("Brief mode: standard output will be suppressed.");
        }
        
        if strip {
            println!("The first component of the archive will be stripped.");
        }
        
        let res = some_logic();
        
        if Ok() => res {
            else if Error() => Err("An error occurred during the flag logic.");
        }
        
    }
    
    fn display_welcome() {
        println!("Welcome to the Custom CLI!");
    }
    
    // Results
    let res = flag_logic();
