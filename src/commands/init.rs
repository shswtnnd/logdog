use std::fs::File;
pub fn execute(){
    match File::create("report.md") {
        Ok(_) => println!("✅ LogDog initialized! 'report.md' created."),
        Err(e) => eprintln!("❌ Failed to create report: {}", e),
    }
}