use std::fs::File;
use std::io::Write;
use std::fs::{OpenOptions};
use std::io::{self, Read};
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "logdog")]
#[command(version = "1.0")]
#[command(about = "Automates bug bounty reporting from the terminal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    //intializes a new report.md file
    init,
    step {
        description:String,
    },
    fetch,
}
fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::init => {
            let mut file = File::create("report.md").expect("ERROR! Failed to create file");
            file.write_all(b"# Vulnerability Report\n\n## Steps to Reproduce:\n").expect("Failed to write");
        }
        Commands::step{description}=>{
            let mut file = OpenOptions::new()
                .append(true)
                .open("report.md")
                .expect("Error: Could not find report.md. Did you run 'init' first?");
            let formatted_step = format!("- {}\n", description);
            file.write_all(formatted_step.as_bytes()).expect("Failed to write step");
            println!("✅ Step logged: {}", description);
        }
        Commands::fetch => {
            let mut input_data = String::new();
            io::stdin().read_to_string(&mut input_data).expect("Failed to read terminal output");

            //Open the file to append
            let mut file = OpenOptions::new()
                .append(true)
                .open("report.md")
                .expect("Error: Could not find report.md. Try running logdog init");

            let formatted_code_block = format!("\n**Terminal Evidence:**\n```bash\n{}\n```\n", input_data.trim());

            //Write to the file
            file.write_all(formatted_code_block.as_bytes()).expect("Failed to write to report");
            
            println!("LogDog fetched your terminal output.");
        }
    }
}