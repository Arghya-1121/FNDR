use std::{env, fs, path::PathBuf};

use crate::service::filter::get_app_details;

struct Location;

#[derive(Debug)]
pub struct Applications {
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
    pub desktop_file: PathBuf,
}

#[derive(Debug)]
pub struct ApplicationDatabase {
    pub applications: Option<Vec<Applications>>,
    pub application_list: Option<Vec<PathBuf>>,
}

impl Location {
    fn applications() -> Vec<PathBuf> {
        let mut temp = Vec::new();

        //Push the home application into the vector buffer
        temp.push(
            env::var_os("XDG_DATA_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    PathBuf::from(env::var_os("HOME").expect("Can't settle the home applications"))
                        .join(".local/share")
                })
                .join("applications"),
        );

        //Push the system applications into the vector buffer
        //including global flatpak
        temp.extend(
            env::var_os("XDG_DATA_DIRS")
                .map(|v| env::split_paths(&v).collect::<Vec<_>>())
                .unwrap_or_else(|| {
                    vec![
                        PathBuf::from("/usr/local/share/"),
                        PathBuf::from("/usr/share/"),
                    ]
                })
                .into_iter()
                .map(|dir| dir.join("applications")),
        );

        println!("{:?}", temp);

        temp
    }
}

impl ApplicationDatabase {
    fn list_files() -> Vec<PathBuf> {
        let directories = Location::applications();
        let mut files: Vec<PathBuf> = Vec::new();

        for directory in directories {
            let entriers = match fs::read_dir(&directory) {
                Ok(entries) => entries,
                Err(err) => {
                    eprintln!("Couldn't read {}:\t{}", directory.display(), err);
                    continue;
                }
            };

            for entry in entriers {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_file() && path.extension().is_some_and(|x| x == "desktop") {
                            files.push(path);
                        } else {
                            eprintln!("Either not file of extension is not .desktop");
                        }
                    }
                    Err(err) => eprintln!("Failed to read entry:{}", err),
                }
            }
        }

        files
    }

    pub fn read_files(&mut self) {
        let list_of_files = Self::list_files();

        if list_of_files.is_empty() {
            self.applications = None;
            self.application_list = None;
        } else {
            self.applications = None;
            self.application_list = Some(list_of_files);
        }
    }

    pub fn app_data() -> Vec<Applications> {
        let list_files = ApplicationDatabase::list_files();
        get_app_details(list_files)
    }
}
