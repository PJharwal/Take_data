use std::fs::File; // Correct import for File
use std::io::{self, Write}; // Include Write for writeln! and write!
use csv::Writer; // Keep this if you are using it to write CSV
use std::path::Path; // Keep this since you're checking file existence

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create variables to hold the user input
    let mut name = String::new();
    let mut mobile = String::new();
    let mut email = String::new();

    // Taking input from the user
    println!("Enter your name:");
    io::stdin().read_line(&mut name).expect("Failed to read name");
    println!("Enter your mobile number:");
    io::stdin().read_line(&mut mobile).expect("Failed to read mobile number");
    println!("Enter your Gmail address:");
    io::stdin().read_line(&mut email).expect("Failed to read email");

    // Remove the newline characters
    name = name.trim().to_string();
    mobile = mobile.trim().to_string();
    email = email.trim().to_string();

    // Save to CSV
    save_to_csv(&name, &mobile, &email)?;

    // Save to text file
    save_to_text_file(&name, &mobile, &email)?;

    println!("Data saved successfully.");
    Ok(())
}

fn save_to_csv(name: &str, mobile: &str, email: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = "user_data.csv";
    
    // Check if the file exists, if not create a new one with headers
    let file_exists = Path::new(path).exists();
    
    // Open the file in append mode
    let file = File::options().append(true).open(path).or_else(|_| File::create(path))?;
    let mut wtr = Writer::from_writer(file);

    // If file doesn't exist, write the header
    if !file_exists {
        wtr.write_record(&["Name", "Mobile", "Email"])?;
    }

    // Write the user's data to the CSV file
    wtr.write_record(&[name, mobile, email])?;
    wtr.flush()?;
    
    Ok(())
}

fn save_to_text_file(name: &str, mobile: &str, email: &str) -> std::io::Result<()> {
    let mut file = File::create("user_data.txt")?;
    
    // Write data to a text file
    writeln!(file, "Name: {}", name)?;
    writeln!(file, "Mobile: {}", mobile)?;
    writeln!(file, "Email: {}", email)?;
    
    Ok(())
}
