// Used to parse command-line arguments
use clap::Parser;

// Used to convert between datetime and swatch time
use beats::Beat;

// Used to get the current datetime
use chrono::Local;

// Used to pause thread execution
use std::thread;

// Used to declare a time to pause execution for
use std::time::Duration;


#[derive(Parser)]
struct Args {

    #[clap(long="repeat", short='r', required=true)]
    repeat: String
}


// Fetch the current datetime
fn get_datetime() -> String {

    // Get the current datetime
    let datetime = Local::now();

    // Get the date, without offset and time
    let date = datetime.date_naive();

    // Convert from datetime to swatch time
    let time = Beat::from(datetime)
        .to_string();

    // Format the datetime as date and swatch time
    let now = format!("{} {}", date, time);

    return now;
}


fn main() {

    // Parse command-line arguments
    let args = Args::parse();

    if args.repeat == "true" {
        loop {
            let time = get_datetime();
            println!("{}", time);
            thread::sleep(Duration::new(1, 0));
        }
    } else {
        let time = get_datetime();
        println!("{}", time);
    }
}
