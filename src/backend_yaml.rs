extern crate serde;
extern crate serde_json;
extern crate serde_value;
extern crate serde_yaml;

use std::path::{Path, PathBuf};
use std::error::Error;
use std::fs::{File, remove_file, copy};
use std::io::{Read, Write};

//TODO: Extract this trait when we have more backend type
//trait FileBackend {
//    fn get(&str) -> serde_value::Value;
//    fn put(serde_value::Value) -> Result;
//    fn delete(&str) -> Result;
//}
fn get_path(key: &str) -> PathBuf {
    return Path::new(key).with_extension("yml");
}

pub fn get(key: &str) -> Result<serde_value::Value, String> {
    let key_path = get_path(key);
    let mut contents = String::new();
    let mut f = File::open(&key_path).map_err(|e| e.to_string())?;    
    // TODO: We crash if the file is corrupted, find better way to handle this
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    match serde_yaml::from_str(&contents) {
        Ok(yaml) => Ok(yaml),
        Err(e) => panic!("File is not valid YAML. Crash to avoid losing the data.\n{} at line {} column {}",
                         e.description(), e.location().unwrap().line(), e.location().unwrap().column())
    }
}

pub fn put(key: &str, value: serde_value::Value) -> Result<(), String> {
    let key_path = get_path(key);

    let value_str = serde_yaml::to_string(&value).unwrap();

    let mut file = File::create(&key_path).map_err(|e| e.to_string())?;
    file.write_all(value_str.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn delete(key: &str) -> Result<(), String> {
    let key_path = get_path(key);
    remove_file(key_path).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn backup(key: &str) -> Result<(), String> {
    let key_path = get_path(key);
    if !key_path.exists() {
        return Ok(());
    }
    let backup_path = key_path.with_extension("yml.bk");
    copy(key_path, backup_path).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod test {
    extern crate serde_value;
    extern crate serde_json;

    use std::fs::{File, remove_file};
    use std::io::{Read, Write};
    use std::path::Path;

    use super::{get, put, delete, backup};

    #[test]
    fn test_get() {
        let test_path = "test_get.yml";
        let value: serde_value::Value = serde_json::from_str("{\"hello\": \"world\"}").unwrap(); 

        let mut file = File::create(test_path).expect("Fail to create test file");
        file.write_all(serde_json::to_string(&value).expect("Fail to serialize").as_bytes())
            .expect("Fail to write the test file");

        assert_eq!(get("test_get"), Ok(value));
        assert_eq!(get("test_get_doesnt_exist"), Err("No such file or directory (os error 2)".to_string()));
        remove_file(test_path).expect("Fail to remove test file");
    }

    #[test]
    fn test_put() {
        let test_path = "foo.yml";
        let value: serde_value::Value = serde_json::from_str("{\"bar\": 1, \"hey\": 2}").unwrap(); 

        assert_eq!(put("foo", value), Ok(()));
        let mut contents = String::new();
        let mut f = File::open(test_path).expect("file not found");
        f.read_to_string(&mut contents).expect("something went wrong reading the file");
        assert_eq!(contents, "---\nbar: 1\nhey: 2");
        remove_file(test_path).expect("Fail to remove test file");
    }

    #[test]
    fn test_delete() {
        let test_path = Path::new("test_delete.yml");
        File::create(test_path).unwrap();
        assert!(test_path.exists());
        assert_eq!(delete("test_delete"), Ok(()));
        assert!(!test_path.exists());
    }

    #[test]
    fn test_backup() {
        let orig_path = "orig.yml";
        let value_str = "Hello world";
        let mut file = File::create(orig_path).expect("Original file failed to be created");
        file.write_all(value_str.as_bytes()).expect("Fail to write the original file");


        backup("orig").expect("Backup failed");

        let mut f = File::open("orig.yml.bk").expect("Backup file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Something went wrong reading the file");
        assert_eq!(contents, value_str);
    }

    #[test]
    fn test_backup_non_existant() {
        backup("non-existant").expect("Should just continue if the file don't exist");
    }
}
