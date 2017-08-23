extern crate chrono;

use std::fs;
use std::time::UNIX_EPOCH;
use chrono::{Utc, TimeZone};



fn main() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let entry = path.unwrap();
        let path = entry.path();
        let metadata = fs::metadata(&path).unwrap();
        println!("{} {}", get_modification_date(metadata), path.display());
    }
}

fn get_modification_date(metadata: fs::Metadata) -> String {
    let latest_mod = metadata.modified().unwrap().duration_since(UNIX_EPOCH).unwrap();
    let date = Utc.timestamp((latest_mod.as_secs() as i64), 0);
    let date_formated = date.format("%d %b %y %H:%M").to_string();
    return date_formated;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::main()
    }
}
