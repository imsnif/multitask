mod parallel_tasks;
mod multitask_file;
use zellij_tile::prelude::*;

use std::collections::{VecDeque, BTreeMap};

use std::path::PathBuf;
use std::time::{Instant, Duration};

use parallel_tasks::ParallelTasks;
use multitask_file::{parse_multitask_file, create_file_with_text};

const DEBOUNCE_TIME_MS: u64 = 400;

#[derive(Default)]
struct State {
    tasks: VecDeque<ParallelTasks>,
    running_tasks: Option<ParallelTasks>,
    multitask_file: PathBuf,
    multitask_file_name: String,
    completed_task_ids: Vec<u32>,
    edit_pane_id: Option<u32>,
    last_run: Option<Instant>,
    is_hidden: bool,
    plugin_id: Option<u32>,
    shell: String,
    ccwd: Option<PathBuf>,
}

impl ZellijPlugin for State {
    fn load(&mut self, config: BTreeMap<String, String>) {
        request_permission(&[PermissionType::ReadApplicationState, PermissionType::ChangeApplicationState, PermissionType::RunCommands, PermissionType::OpenFiles]);
        subscribe(&[EventType::PaneUpdate, EventType::FileSystemUpdate, EventType::FileSystemCreate, EventType::Key]);
        self.plugin_id = Some(get_plugin_ids().plugin_id);

        self.multitask_file_name = match config.get("multitask_file_name") {
            Some(s) => format!("{}", s),
            _ => format!(".multitask{}",get_plugin_ids().plugin_id.to_string()),
        };

        self.shell = match config.get("shell") {
            Some(s) => String::from(s),
            _ => String::from("bash")
        };

        self.ccwd = match config.get("ccwd") {
            Some(s) => Some(PathBuf::from(s)),
            _ => None
        };

        self.multitask_file = PathBuf::from("/host").join(self.multitask_file_name.clone());

        watch_filesystem();
        show_self(true);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::FileSystemUpdate(paths) => {
                if self.multitask_file_was_updated(&paths) {
                    self.stop_run_and_reparse_file();
                }
            }
            Event::FileSystemCreate(paths) => {
                if self.multitask_file_was_updated(&paths) {
                    self.stop_run_and_reparse_file();
                }
            }
            Event::PaneUpdate(pane_manifest) => {
                if self.gained_focus(&pane_manifest) {
                    // whenever the plugin gains focus, eg. with the `LaunchOrFocusPlugin` keybind
                    // we clean up our state and start over, allowing the plugin to be triggered by
                    // a keybinding
                    hide_self();
                    self.start_multitask_env();
                } else if let Some(running_tasks) = &mut self.running_tasks {
                    running_tasks.update_task_status(&pane_manifest);
                    if running_tasks.all_tasks_completed_successfully() {
                        self.progress_running_tasks();
                    }
                }
            }
            _ => (),
        };
        return false; // this plugin never renders
    }
    fn render(&mut self, _rows: usize, _cols: usize) {} // no ui, no problems!
}

impl State {
    pub fn start_current_tasks(&mut self) {
        if let Some(running_tasks) = &self.running_tasks {
            for task in &running_tasks.run_tasks {
                let cmd = CommandToRun {
                    path: (&task.command).into(), 
                    args: task.args.clone(),
                    cwd: self.ccwd.clone()
                };
                open_command_pane_floating(cmd, None, BTreeMap::<String, String>::new());
            }
        }
    }
    pub fn progress_running_tasks(&mut self) {
        if let Some(running_tasks) = self.running_tasks.as_ref() {
            for task in &running_tasks.run_tasks {
                if let Some(terminal_pane_id) = task.terminal_pane_id {
                    focus_terminal_pane(terminal_pane_id as u32, true);
                    toggle_pane_embed_or_eject();
                    self.completed_task_ids.push(terminal_pane_id);
                }
            }
            if let Some(edit_pane_id) = self.edit_pane_id {
                focus_terminal_pane(edit_pane_id as u32, false);
            }
        }
        self.running_tasks = None;
        if let Some(tasks) = self.tasks.remove(0) {
            self.running_tasks = Some(tasks);
            self.start_current_tasks();
        }
    }
    pub fn stop_run(&mut self) {
        let mut all_tasks = vec![];
        if let Some(running_tasks) = self.running_tasks.as_mut() {
            all_tasks.append(&mut running_tasks.pane_ids());
        }
        all_tasks.append(&mut self.completed_task_ids.drain(..).collect());
        for pane_id in all_tasks {
            close_terminal_pane(pane_id as u32);
        }
        self.running_tasks = None;
        self.completed_task_ids = vec![];
    }
    pub fn parse_file(&mut self) -> bool {
        match parse_multitask_file(self.multitask_file.clone(), self.shell.as_str()) {
            Ok(new_tasks) => {
                self.tasks = new_tasks.into();
                return true;
            },
            Err(e) => {
                eprintln!("Failed to parse multitask file: {}", e);
                return false;
            }
        };
    }
    pub fn stop_run_and_reparse_file(&mut self) {
        self.stop_run();
        let file_changed = self.parse_file();
        if file_changed {
            self.last_run = Some(Instant::now());
            self.progress_running_tasks();
        }
    }
    pub fn start_multitask_env(&mut self) {
        self.stop_run();
        create_file_with_text(
            &self.multitask_file,
            &format!("{}{}\n#\n{}\n{}\n{}\n{}\n",
                "#!", self.shell,
                "# Hi there! Anything below these lines will be executed on save.",
                "# One command per line.",
                "# Place empty lines between steps that should run in parallel.",
                "# Enjoy!"
            )
        );
        let dark = include_str!("assets/multitask_layout.kdl");
        let stringified_layout_for_new_tab = &dark.replace(".multitask",self.multitask_file_name.as_str());
        new_tabs_with_layout(stringified_layout_for_new_tab);
    }
    pub fn gained_focus(&mut self, pane_manifest: &PaneManifest) -> bool {
        if let Some(own_plugin_id) = self.plugin_id {
            for (_tab_id, panes) in &pane_manifest.panes {
                for pane in panes {
                    let is_own_plugin_pane = pane.is_plugin && own_plugin_id == pane.id;
                    if is_own_plugin_pane && pane.is_focused && !self.is_hidden {
                        self.is_hidden = true;
                        return true;
                    }
                }
            }
        }
        return false;
    }
    pub fn multitask_file_was_updated(&mut self, changed_paths: &Vec<(PathBuf, Option<FileMetadata>)>) -> bool {
        for path in changed_paths {
            if &path.0 == &self.multitask_file {
                if self.last_run
                    .map(|l| l.elapsed() > Duration::from_millis(DEBOUNCE_TIME_MS))
                        .unwrap_or(true) {
                    return true;
                }
            }
        }
        return false;
    }
}

register_plugin!(State);
