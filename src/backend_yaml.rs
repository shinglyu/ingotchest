extern crate serde;
extern crate serde_json;
extern crate serde_value;
extern crate serde_yaml;

use std::path::{Path, PathBuf};
use std::fs::{File, remove_file};
use std::io::{Read, Write};
use std::error::Error;

//TODO: Extract this trait when we have more backend type
//trait FileBackend {
//    fn get(&str) -> serde_value::Value;
//    fn put(serde_value::Value) -> Result;
//    fn delete(&str) -> Result;
//}
fn get_path(key: &str) -> PathBuf {
    return Path::new(key).with_extension("yaml");
}

pub fn get(key: &str) -> Result<serde_value::Value, String> {
    let key_path = get_path(key);
    let mut contents = String::new();
    let mut f = match File::open(&key_path) {
        Ok(f) => f,
        Err(why) => return Err(why.description().to_string()),
    };
    // TODO: We crash if the file is corrupted, find better way to handle this
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    match serde_yaml::from_str(&contents) {
        Ok(yaml) => Ok(yaml),
        Err(_) => panic!("File is not valid. Crash to avoid losing the data.")
    }
}

pub fn put(key: &str, value: serde_value::Value) -> Result<(), String> {
    let key_path = get_path(key);
    let mut file = match File::create(&key_path) {
        // TODO: Return 500 Server Error
        Err(why) => return Err(format!("couldn't create {}: {}",
                                   key_path.display(),
                                   why.description())),
        Ok(file) => file,
    };

    let value_str = serde_yaml::to_string(&value).unwrap();

    match file.write_all(value_str.as_bytes()) {
        Err(why) => Err(format!("couldn't write to {}: {}", 
                            key_path.display(), 
                            why.description())),
        Ok(_) => Ok(())
    }
}

pub fn delete(key: &str) -> Result<(), String> {
    let key_path = get_path(key);
    match remove_file(key_path) {
        // TODO: Return 500 Server Error
        Err(why) => Err(why.description().to_string()),
        Ok(_) => Ok(())
    }
}

#[cfg(test)]
mod test {
    extern crate serde_value;
    extern crate serde_json;

    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    use super::{get, put, delete};

    #[test]
    fn test_get() {
        let value: serde_value::Value = serde_json::from_str("{\"hello\": \"world\"}").unwrap(); 

        assert_eq!(get("test_get"), Ok(value));
        assert_eq!(get("test_get_doesnt_exist"), Err("entity not found".to_string()));
    }

    #[test]
    fn test_put() {
        let value: serde_value::Value = serde_json::from_str("{\"bar\": 1, \"hey\": 2}").unwrap(); 

        assert_eq!(put("foo", value), Ok(()));
        let mut contents = String::new();
        let mut f = File::open("foo.yaml").expect("file not found");
        f.read_to_string(&mut contents).expect("something went wrong reading the file");
        assert_eq!(contents, "---\nbar: 1\nhey: 2");
    }

    #[test]
    fn test_delete() {
        let test_path = Path::new("test_delete.yaml");
        File::create(test_path).unwrap();
        assert!(test_path.exists());
        assert_eq!(delete("test_delete"), Ok(()));
        assert!(!test_path.exists());
    }

}
