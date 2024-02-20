use std::fs;
use reqwest::{header::COOKIE, blocking::Client, StatusCode};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    year: usize,
    #[arg(short, long)]
    session: String,
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();
    let client = Client::new();
    let input_dir = format!("./examples/{}/input", args.year);

    if !exists(&input_dir) {
        fs::create_dir_all(&input_dir).expect("Could not create input directory");
    }


    println!("Getting input for each day.");
    for day in 1..=25 {
        print!(" - day {:02} ... ", day);
        let file_path = format!("{input_dir}/{day:02}");

        if !exists(&file_path) {
            let input_url = format!("https://adventofcode.com/{}/day/{day}/input", args.year);
            let res = client
                .get(input_url)
                .header(COOKIE, format!("session={}", args.session))
                .send()
                .expect(format!("Error requesting input from server. [day {}]", day).as_str());

            let input_body = match res.status() {
                StatusCode::OK => res.text().expect("Could not retrieve text from response."),
                _ => panic!("Server returned an error.\n  (Check that your session token is up to date)"),
            };

            fs::write(file_path, input_body).expect(format!("Could not write input file for {day}").as_str());
            println!("OK");
        } else {
            println!("skip");
        }
    }
    println!("Done. Happy Coding :-)");
    Ok(())
}

fn exists(path: &str) -> bool { fs::metadata(path).is_ok() }
