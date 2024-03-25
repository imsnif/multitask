use zellij_tile::prelude::*;

use std::path::{PathBuf, Path};
use std::io::prelude::*;

use crate::parallel_tasks::{ParallelTasks, RunTask};

pub fn create_file_with_text(path: &Path, text: &str) {
    if !path.exists() {
        // Only create the file if it does not already exists. Otherwise, use the file that is
        // already there.
        if let Err(e) = std::fs::File::create(PathBuf::from("/host").join(path)).and_then(|mut f| {
            f.write_all(text.as_bytes())
        }) {
            eprintln!("Failed to create file with error: {}", e);
        };
    }
}

pub fn parse_multitask_file(filename: PathBuf, shell: &str) -> Result<Vec<ParallelTasks>, std::io::Error> {
    let stringified_file = std::fs::read_to_string(filename)?;
    let mut parallel_tasks = vec![];
    let mut current_tasks = vec![];
    let mut current_step = 1;
    for line in stringified_file.lines() {
        let line = line.to_string();
        let line_is_empty = line.trim().is_empty();
        if !line.starts_with("#") && !line_is_empty {
            let task = RunTask::from_file_line(shell, &line, current_step);
            current_tasks.push(task);
        } else if line_is_empty && !current_tasks.is_empty() {
            parallel_tasks.push(ParallelTasks::new(current_tasks.drain(..).collect()));
            current_step += 1;
        }
    }
    if !current_tasks.is_empty() {
        parallel_tasks.push(ParallelTasks::new(current_tasks.drain(..).collect()));
    }
    Ok(parallel_tasks)
}
