#![deny(elided_lifetimes_in_paths)]

use std::{
    io::{Read, Write},
};

/*
 * Writes Instance of PathManager to json file already handles all serialization*/
pub fn write_to_file<T: serde::Serialize>(object: &T, path: &std::path::Path) -> Result<i32, String> {
    /*
     * God forgive me for my sins of being too much of a nester*/
    match path.parent() {
            Some(p) => {
                match std::fs::create_dir_all(p) {
                    Ok(_) => {
                        match std::fs::File::create(path) {
                            Ok(mut f) => {
                                match serde_json::to_string(object) {
                                    Ok(data_string) => {
                                        match f.write_all(data_string.as_bytes()) {
                                            Ok(_) => Ok(0),
                                            Err(e) => Err("Error at writing data to file: "
                                                .to_owned()
                                                + &e.to_string()),
                                        } //Match 5 End
                                    }
                                    Err(e) => Err("Error at serialization of PathManager: "
                                        .to_owned()
                                        + &e.to_string()),
                                } //Match 4 End
                            }
                            Err(e) => Err("Error at file creation: ".to_owned() + &e.to_string()),
                        } //Match 3 End
                    }
                    Err(e) => Err("Error at recurive directory creation after getting the parent directory: ".to_owned() + &e.to_string()),
                } //Match 2 End
            }
            None => Err("Error at parent directory creation: There is no parent directory specified in given path".to_owned()),
        } //Match 1 End
}

pub fn read_from_file<'a, T: serde::Deserialize<'a>>(path: &std::path::Path, strbuf: &'a mut String) -> Result<T, String> {
        /*
         * These match statements are horrible*/
        match std::fs::File::open(path) {
            Ok(mut f) => {
                match f.read_to_string(strbuf) {
                    Ok(_) => match serde_json::from_str(strbuf) {
                        Ok(t) => t,
                        Err(e) => Err("Error deserializing json: ".to_owned() + &e.to_string()),
                    }, // Match 3 End
                    Err(e) => Err("Error reading string from file: ".to_owned() + &e.to_string()),
                } //Match 2 End
            }
            Err(e) => Err("Error opening file: ".to_owned() + &e.to_string()),
        } //Match 1 End
    }
