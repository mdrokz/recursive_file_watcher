use notify::DebouncedEvent::{
    Chmod, Create, Error, NoticeRemove, NoticeWrite, Remove, Rename, Rescan, Write,
};
use notify::{watcher, RecursiveMode, Watcher};
use std::env;
use std::ffi::OsStr;
use std::fs::{self};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let read_path = match env::args().nth(1) {
        Some(text) => text,
        None => {
            println!("sorry you didnt provide the read path in the arguments.");
            return;
        }
    };

    let write_path = match env::args().nth(2) {
        Some(text) => text,
        None => {
            println!("sorry you didnt provide the write path in the arguments.");
            return;
        }
    };

    println!("READ:{},WRITE:{}", read_path, write_path);
    let _path = String::from(&read_path);
    let (sender, channel) = channel();

    let mut watcher = watcher(sender, Duration::from_millis(500)).unwrap();

    watcher.watch(read_path, RecursiveMode::Recursive).unwrap();
    loop {
        match channel.recv() {
            Ok(event) => match event {
                NoticeWrite(path) => println!("NoticeWrite:{:?}", path),
                NoticeRemove(path) => println!("NoticeRemove:{:?}", path),
                Create(_path) => {}
                Chmod(path) => println!("Chmod:{:?}", path),
                Error(err, path) => println!("Error:{:?},path:{:?}", err, path),
                Remove(_path) => {}
                Rename(_old_path, _new_path) => {}
                Write(path) => {
                    println!("Write: {:?}", path);
                    read_directory(PathBuf::from(&_path), &write_path, &_path);
                }
                Rescan => println!("Rescan: Directory is now being rescanned"),
            },
            Err(e) => println!("watch error {:?}", e),
        }
    }
}

fn read_directory(path: PathBuf, write_path: &String, read_path: &String) {
    let items = fs::read_dir(&path).unwrap();
    let compare_path = &path.into_os_string().into_string().unwrap();
    let new_path = &compare_path.replace("\\", "/");
    let dir_path: Option<&String> = if compare_path != read_path {
        let s = new_path;
        Some(s)
    } else {
        None
    };
    for entry in items {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type() {
                if !file_type.is_dir() {
                    read_and_write(
                        entry.file_name().as_os_str(),
                        &write_path,
                        &read_path,
                        dir_path,
                    );
                } else {
                    read_directory(entry.path(), &write_path, &read_path);
                }
            }
        }
    }
}

fn read_and_write(
    file_name: &OsStr,
    write_path: &String,
    read_path: &String,
    dir_path: Option<&String>,
) {
    match file_name.to_str() {
        Some(txt) => match dir_path {
            Some(val) => {
                println!("{}", val);
                let mut path_name = String::from(val.as_str());
                path_name.push_str("/");
                path_name.push_str(txt);
                let mut pathds = String::from(write_path);
                pathds.push_str("/");
                pathds.push_str(txt);
                match fs::copy(path_name, pathds) {
                    Ok(res) => println!("SUCCESS_CODE:{:?}", res),
                    Err(err) => println!("Error:{:?}", err),
                }
            }
            None => {
                let mut path_name = String::from(read_path);
                let mut pathds = String::from(write_path);
                pathds.push_str("/");
                pathds.push_str(txt);
                path_name.push_str("/");
                path_name.push_str(txt);
                match fs::copy(path_name, pathds) {
                    Ok(res) => println!("SUCCESS_CODE:{:?}", res),
                    Err(err) => println!("Error:{:?}", err),
                }
            }
        },
        None => panic!("ERROR:no files found!"),
    }
}
