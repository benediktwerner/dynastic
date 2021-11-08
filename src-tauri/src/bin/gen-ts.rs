use dynastic::model::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <schema>.json", args[0]);
        std::process::exit(1);
    }

    let schema = schemars::schema_for!(Person);
    let schema = serde_json::to_string_pretty(&schema).unwrap();
    std::fs::write(&args[1], schema).unwrap();
}
