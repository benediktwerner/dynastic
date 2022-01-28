use std::path::PathBuf;

use dynastic::model::*;

macro_rules! write_schema {
    ($($t:ty),+ => $dir:expr) => {
        $(
            let schema = schemars::schema_for!($t);
            let schema = serde_json::to_string_pretty(&schema).unwrap();
            let name = std::any::type_name::<$t>().split("::").last().unwrap();
            let path = $dir.join(format!("{}.json", name));
            println!("{}", name);
            std::fs::write(path, schema).unwrap();
        )+
    };
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <schemas directory>", args[0]);
        std::process::exit(1);
    }

    let dir = PathBuf::from(&args[1]);
    std::fs::create_dir_all(&dir).unwrap();

    write_schema! {
        Person, Fact => (&dir)
    }
}
