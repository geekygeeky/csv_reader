use std::error::Error;

fn read_csv(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    if let Err(e) = read_csv("./currency.csv") {
        eprintln!("{}", e);
    }
}
