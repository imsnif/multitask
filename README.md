![img-2023-06-20-165418](https://github.com/imsnif/multitask/assets/795598/9877c93c-60a8-45ad-a113-354440741fd9)

## About
This [Zellij][zellij] plugin is a "mini-ci". It allows you to specify commands that will run in parallel, keeping track of completed commands and their exit status. Only progressing to the next step if all the commands in the previous step succeeded.

Did one command fail? No problem! Fix the issue, re-run it with `ENTER` and the pipeline will continue.

## How does it work?

Multitask opens your default `$EDITOR` in a new tab pointed to a `.multitask` file. Any commands pasted to this file will run when you save it. 

Multitask will divide tasks into "steps" separated by an empty (whitespace) line. Running all commands in each step in parallel and only moving on to the next step if all commands exited successfully. You can re-run failed commands by pressing `ENTER` while focused on their pane, and if they exit successfully, the next step will be run.

### Example
```
echo "I will be run first"
echo "I will be run simultaneously to the line above" && sleep 2

echo "I will be run 2 seconds later"
echo "so will I!"
```

[zellij]: https://github.com/zellij-org/zellij

## Installation
1. Download the `multitask.wasm` file from the release matching your installed Zellij version
2. Place it in `$HOME/zellij-plugins`
3. Set a keybind to run `multitask`. This must be done in your configuratino file for zellij (often found at `~/.config/zellij/config.kdl`). The keybind should look something like this:
```
normal {
    bind "Alt 1" {
        MessagePlugin {
            payload "multitask_run"
        }
    }
}
```
this example binds `Alt 1` to run multitask, but you can use whatever you like.

4. From within Zellij, run ``zellij action start-or-reload-plugin file:$HOME/zellij-plugins/multitask.wasm --configuration "shell=$SHELL,ccwd=`pwd`"``

## Configuration options
| Option | Description |
| :--- | :--- |
| shell | Sets the shebang of multitask file. `$SHELL` is the recommended value. |
| ccwd | Sets the current working directory for `multitask` commands. If no value is given, then the `host` of the `zellij` session is used. |
| layout | Sets the layout used for multitask. This should be a string. For example, if you have your layout in a file at `$HOME/.config/zellij/multitask_layout.kdl` then use ``layout=`cat $HOME/.config/zellij/multitask_layout.kdl` `` when invoking the plugin. The layout you use should open `.multitask` in an edit pane (this filename will be replaced with `multitask_file_name` automatically if applicable, see below for details). The default layout can be found at `src/assets/multitask_layout.kdl` for inspiration. |
| multitask\_file\_name | The name of the multitask file. This can be used to point to a multitask file that already exists or provide a specific name for a new multitask file. If no name is provided, then the multitask file name will be `.multiask#` where `#` is the plugin ID number.

## Starting from a keybind
It may be desirable to start `multitask` via a keybind. However, since users often want ``ccwd=`pwd` `` when starting the plugin, it can't be done with a simple keybind. Instead, it is recommended you put the command you use to launch the plugin into a bash script, and then invoke that bash script via a keybind. An example bash script may look like:
```
#!/bin/bash

zellij action start-or-reload-plugin file:$HOME/.zellij_plugins/multitask.wasm --configuration shell="/bin/zsh,ccwd=`pwd`,layout=`cat $HOME/.config/zellij/multitask_layout.kdl`"
zellij action close-pane
```
Suppose this script is called `open_multitask` and is located somewhere on your `PATH`, e.g., `~/.local/bin`, then you can add a keybind like
```
normal {
    bind "Alt m" { Run "open_multitask"; }
}
```
to start multitask.

## Development
Load the `dev.kdl` layout from inside zellij: `zellij action new-tab -l dev.kdl` or from outside Zellij with `zellij -l dev.kdl`
