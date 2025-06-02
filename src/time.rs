use chrono::{DateTime, Utc};
use std::{
    fs::File,
    io::Write,
    path::{self, Path},
};

use homedir::my_home;

pub fn sxsv_time() -> bool {
    let home = my_home()
        .ok()
        .flatten()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let path_to_time = home + "/.time_sxsv";

    if Path::new(&path_to_time).exists() == true {
        return true;
    } else {
        let mut file = File::create(path_to_time).unwrap();

        let now: DateTime<Utc> = Utc::now();
        let now_str = now.to_rfc3339(); // or  now.to_string()
        file.write_all(now_str.as_bytes()).unwrap();
        return false;
    }
}
