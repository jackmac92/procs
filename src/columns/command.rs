use crate::process::ProcessInfo;
use crate::util::format_command;
use crate::{column_default, Column};
use std::cmp;
use std::collections::HashMap;

pub struct Command {
    header: String,
    unit: String,
    fmt_contents: HashMap<i32, String>,
    raw_contents: HashMap<i32, String>,
    width: usize,
    abbr_path: bool,
}

impl Command {
    pub fn new(header: Option<String>, abbr_path: bool) -> Self {
        let header = header.unwrap_or_else(|| String::from("Command"));
        let unit = String::new();
        Self {
            fmt_contents: HashMap::new(),
            raw_contents: HashMap::new(),
            width: 0,
            header,
            unit,
            abbr_path,
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
impl Column for Command {
    fn add(&mut self, proc: &ProcessInfo) {
        let base_content = if let Ok(cmd) = &proc.curr_proc.cmdline() {
            if !cmd.is_empty() {
                let mut cmd = cmd
                    .iter()
                    .cloned()
                    .map(|mut x| {
                        x.push(' ');
                        x
                    })
                    .collect::<String>();
                cmd.pop();
                cmd = cmd.replace(['\n', '\t'], " ");
                cmd
            } else {
                format!("[{}]", proc.curr_proc.stat().comm)
            }
        } else {
            proc.curr_proc.stat().comm.clone()
        };
        let raw_content = base_content.clone();
        let fmt_content = format_command(base_content, self.abbr_path);

        self.fmt_contents.insert(proc.pid, fmt_content);
        self.raw_contents.insert(proc.pid, raw_content);
    }

    column_default!(String);
}

#[cfg(target_os = "macos")]
impl Column for Command {
    fn add(&mut self, proc: &ProcessInfo) {
        let fmt_content = if let Some(path) = &proc.curr_path {
            if !path.cmd.is_empty() {
                let mut cmd = path
                    .cmd
                    .iter()
                    .cloned()
                    .map(|mut x| {
                        x.push(' ');
                        x
                    })
                    .collect::<String>();
                cmd.pop();
                cmd = cmd.replace(['\n', '\t'], " ");
                cmd
            } else {
                String::from("")
            }
        } else {
            String::from("")
        };
        let raw_content = fmt_content.clone();

        self.fmt_contents.insert(proc.pid, fmt_content);
        self.raw_contents.insert(proc.pid, raw_content);
    }

    column_default!(String);
}

#[cfg(target_os = "windows")]
impl Column for Command {
    fn add(&mut self, proc: &ProcessInfo) {
        let fmt_content = proc.command.clone();
        let raw_content = fmt_content.clone();

        self.fmt_contents.insert(proc.pid, fmt_content);
        self.raw_contents.insert(proc.pid, raw_content);
    }

    column_default!(String);
}

#[cfg(target_os = "freebsd")]
impl Column for Command {
    fn add(&mut self, proc: &ProcessInfo) {
        let command = if proc.curr_proc.arg.is_empty() {
            let comm = crate::util::ptr_to_cstr(proc.curr_proc.info.comm.as_ref());
            if let Ok(comm) = comm {
                format!("[{}]", comm.to_string_lossy())
            } else {
                String::from("")
            }
        } else {
            let mut x = String::from("");
            for arg in &proc.curr_proc.arg {
                x.push_str(&arg);
                x.push_str(" ");
            }
            x
        };
        let fmt_content = command;
        let raw_content = fmt_content.clone();

        self.fmt_contents.insert(proc.pid, fmt_content);
        self.raw_contents.insert(proc.pid, raw_content);
    }

    column_default!(String);
}
