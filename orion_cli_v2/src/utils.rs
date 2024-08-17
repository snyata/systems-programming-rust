/// CLI Visual Adjustments
/// Customizable Logging and Display

fn display_welcome(project_name: &str, description: &str) {
    let welcome_message = r#"
         ____   _____   _____  ____   _   _
    / __ \ |  __ \ |_   _|/ __ \ | \ | |
   | |  | || |__) |  | | | |  | ||  \| |
   | |  | ||  _  /   | | | |  | || . ` |
   | |__| || | \ \  _| |_| |__| || |\  |
    \____/ |_|  \_\|_____|\____/ |_| \_|
     _____  _       _____
     | |     | |       | |
    / ____|| |     |_   _|
   | |     | |       | |
   | |____ | |____  _| |_
    \_____||______||_____| "

:: {project_name} ::
{description}

    ######### WELCOME ##########
"#;
    println!("{}", welcome_message.replace("{project_name}", project_name).replace("{description}", description, colorize_output(welcome_message, "green", project_name="yellow", description="yellow"), bold_heading(welcome_message)));
}

pub fn colorize_output(output: &str, color: &str) -> ColoredString {
    debug!("Colorizing output '{}' with color '{}'", output, color);
    match color {
        "error" => output.red(),
        "info" => output.green(),
        "debug" => output.blue(),
        "critical" => output.yellow(),
        _ => {
            debug!("Unknown color '{}', returning normal output.", color);
            output.normal()
        }
    }
}

pub fn bold_heading(heading: &str) -> ColoredString {
    debug!("Bolding heading '{}'", heading);
    heading.bold()
}