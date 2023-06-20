use zellij_tile::prelude::PaneManifest;

use std::fmt;

#[derive(Default, Debug)]
pub struct ParallelTasks {
    pub run_tasks: Vec<RunTask>
}

#[derive(Default, Debug)]
pub struct RunTask {
    pub command: String,
    pub args: Vec<String>,
    pub terminal_pane_id: Option<u32>,
    pub is_complete: bool,
    pub succeeded: bool,
}

impl ParallelTasks {
    pub fn new(run_tasks: Vec<RunTask>) -> Self {
        ParallelTasks {
            run_tasks,
        }
    }
    pub fn all_tasks_completed_successfully(&self) -> bool {
        self.run_tasks.iter().all(|t| t.succeeded())
    }
    pub fn tasks_failed(&self) -> bool {
        self.run_tasks.iter().any(|t| t.failed())
    }
    pub fn pane_ids(&self) -> Vec<u32> {
        let mut pane_ids = vec![];
        for task in &self.run_tasks {
            if let Some(terminal_pane_id) = task.terminal_pane_id {
                pane_ids.push(terminal_pane_id);
            }
        }
        pane_ids
    }
    pub fn update_task_status(&mut self, pane_manifest: &PaneManifest) {
        for (_tab_id, panes) in &pane_manifest.panes {
            for pane in panes {
                for task in &mut self.run_tasks {
                    if !task.is_complete() {
                        let stringified_task = task.to_string();
                        if Some(stringified_task) == pane.terminal_command && pane.exited {
                            task.mark_pane_id(pane.id);
                            task.mark_complete(pane.exit_status);
                            break;
                        }
                    }
                }
            }
        }
    }
}

impl fmt::Display for RunTask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.args.is_empty() {
            write!(f, "{}", self.command)
        } else {
            write!(f, "{} {}", self.command, self.args.join(" "))
        }
    }
}

impl RunTask {
    pub fn new<T: AsRef<str>>(mut command_and_args: Vec<T>) -> Self {
        RunTask {
            command: command_and_args.remove(0).as_ref().to_owned(),
            args: command_and_args.iter().map(|c| c.as_ref().to_owned()).collect(),
            ..Default::default()
        }
    }
    pub fn from_file_line(file_line: &str) -> Self {
        Self::new(vec!["bash", "-c", file_line])
    }
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }
    pub fn succeeded(&self) -> bool {
        self.is_complete && self.succeeded
    }
    pub fn failed(&self) -> bool {
        self.is_complete && !self.succeeded
    }
    pub fn mark_pane_id(&mut self, pane_id: u32) {
        self.terminal_pane_id = Some(pane_id);
    }
    pub fn mark_complete(&mut self, exit_status: Option<i32>) {
        self.is_complete = true;
        match exit_status {
            Some(exit_status) => {
                self.succeeded = exit_status == 0;
            },
            None => {
                self.succeeded = true;
            }
        }
    }
}

