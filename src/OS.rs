use homedir::my_home;
use std::path::Path;
use std::fs::{self, File};
use std::io::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OS {
    MACOS,
    LINUX,
    WINDOWS,
    NOTSUPPORTED,
    VOID,
}

static mut OSGLOBAL: OS = OS::VOID;

pub fn log_os() -> OS {
    let get_os_string =  std::env::consts::OS;

    if get_os_string == "macos" {
     
     unsafe { OSGLOBAL = OS::MACOS };

    } 
    else if get_os_string == "linux" {

    unsafe { OSGLOBAL = OS::LINUX };

    }
    else if get_os_string == "windows" {
        unsafe { OSGLOBAL = OS::WINDOWS }
    }
    else {
        unsafe { OSGLOBAL = OS::NOTSUPPORTED}
    }


    let home = my_home()
    .ok()
    .flatten()
    .map(|p| p.display().to_string())
    .unwrap_or_else(|| "Unknown".to_string());
//println!("{}", home);

let path_to_log = home + "/.log_sxsv";
//println!("{}", path_to_log);

let log_present: bool = Path::new(&path_to_log).exists();

//println!("{:?}", log_present);

if log_present == false {
    if let Ok(mut file) = File::create(&path_to_log) {
        let _ = file.write(get_os_string.as_bytes());
    }
}
else if log_present == true {
   // println!("...");
}

let get_os = unsafe { &raw const OSGLOBAL };

unsafe { *get_os }
}



pub fn create_sxsv_files_folder_os(os: OS) -> bool {

  let home = my_home()
    .ok()
    .flatten()
    .map(|p| p.display().to_string())
    .unwrap_or_else(|| "Unknown".to_string());
//println!("{}", home);

let path_to_folder = home + "/.sxsv";

if os == OS::MACOS || os == OS::LINUX {

let folder = fs::create_dir(&path_to_folder);
let log_present: bool = Path::new(&path_to_folder).exists();
return true;
} 
else if os == OS::WINDOWS {

return false;

}

true
}

pub fn sxsv_setup() {
    let value_os = log_os();
    let value_debug_sxsv: bool = create_sxsv_files_folder_os(value_os);

   // println!("{:?}", value_os);
   //println!("{:?}", value_debug_sxsv);
}

