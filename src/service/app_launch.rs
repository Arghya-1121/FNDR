use std::{io::Error, os::unix::process::CommandExt, process::Command};

use crate::service::search::Applications;

pub fn app_exec(app: &Applications) -> Error {
    let mut parts = app.exec.split_whitespace();

    let Some(program) = parts.next() else {
        return Error::other("Empty Exec field");
    };

    let args: Vec<&str> = parts
        .filter(|arg| !matches!(*arg, "%f" | "%F" | "%u" | "%U" | "%i"))
        .collect();

    Command::new(program).args(args).exec()
}
