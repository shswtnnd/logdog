use std::io::Write;
use std::fs::{OpenOptions};
use std::io::{self, Read};
use clap::{Parser, Subcommand};
mod commands;
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
    Init,
    Step {
        description:String,
    },
    Fetch,
    View,
}
fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init => {
            commands::init::execute();
        }
        Commands::Step{description}=>{
            let mut file = match OpenOptions::new().append(true).open("report.md") {
                Ok(f)=> f,
                Err(_)=>{
                    eprint!("❌ Error: 'report.md' not found. Run 'logdog init' first!");
                    return;
                }
            };
            let formatted_step = format!("- {}\n",description);
            file.write_all(formatted_step.as_bytes()).expect("Failed to write step");
            println!("✅ Step logged: {}", description);
        }
        Commands::Fetch => {
            let mut input_data = String::new();
            io::stdin().read_to_string(&mut input_data).expect("Failed to read terminal output");
            //Open the file to append
            let mut file = match OpenOptions::new().append(true).open("report.md"){
                Ok(f)=>f,
                Err(_)=>{
                    eprintln!("❌ Error: 'report.md' not found. Run 'logdog init' first!");
                    return;
                }
            };
            let formatted_code_block = format!("\n**Terminal Evidence:**\n```bash\n{}\n```\n", input_data.trim());
            //Write to the file
            file.write_all(formatted_code_block.as_bytes()).expect("Failed to write to report");
            println!("LogDog fetched your terminal output.");
        }
        Commands::View => {
            match std::fs::read_to_string("report.md") {
                //take the massive string of text and print it.
                Ok(content) => {
                    println!("\n==================================");
                    println!("REPORT PREVIEW");
                    println!("==================================\n");
                    println!("{}", content);
                    println!("==================================\n");
                }
                Err(_) => {
                    eprintln!("❌ Error: 'report.md' not found. Run 'logdog init' first!");
                }
            }
        }
    }
}