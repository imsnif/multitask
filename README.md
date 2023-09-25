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
2. Place it in `~/zellij-plugins`
3. From within Zellij, run `zellij action start-or-reload-plugin file:~/zellij-plugins/multitask.wasm --configuration "shell=$SHELL"`

## Development

Load the `dev.kdl` layout from inside zellij: `zellij action new-tab -l dev.kdl` or from outside Zellij with `zellij -l dev.kdl`
