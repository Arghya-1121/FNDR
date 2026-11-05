use std::{fs::read_to_string, path::PathBuf};

use crate::service::search::Applications;

fn parser(location: &PathBuf) -> Result<Applications, &str> {
    let content = read_to_string(location).expect("Failed to read it's content");

    let mut name: Option<String> = None;
    let mut exec: Option<String> = None;
    let mut icon: Option<String> = None;
    let mut in_desktop_entry = false;

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with('[') && line.ends_with(']') {
            in_desktop_entry = line == "[Desktop Entry]";
        }
        if !in_desktop_entry {
            continue;
        }

        if name.is_none() && line.starts_with("Name=") {
            name = Some(
                line.strip_prefix("Name=")
                    .unwrap_or_default()
                    .trim()
                    .to_owned(),
            );
        }

        if exec.is_none() && line.starts_with("Exec=") {
            exec = Some(
                line.strip_prefix("Exec=")
                    .unwrap_or_default()
                    .trim()
                    .to_owned(),
            );
        }

        if icon.is_none() && line.starts_with("Icon=") {
            icon = Some(
                line.strip_prefix("Icon=")
                    .unwrap_or_default()
                    .trim()
                    .to_owned(),
            );
        }

        if name.is_some() && exec.is_some() && icon.is_some() {
            break;
        }
    }

    Ok(Applications {
        name: name.unwrap_or("".to_owned()),
        exec: exec.unwrap_or("".to_owned()),
        icon,
        desktop_file: location.to_path_buf(),
    })
}

pub(crate) fn get_app_details(file_list: Vec<PathBuf>) -> Vec<Applications> {
    let mut applications: Vec<Applications> = vec![];

    for file in file_list {
        let app = parser(&file);
        if let Ok(data) = app {
            applications.push(data);
        } else {
            eprintln!("Failed to read the file content:\t{:?}", file);
        }
    }

    applications
}
