## About
![img-2023-06-20-165418](https://github.com/imsnif/multitask/assets/795598/9877c93c-60a8-45ad-a113-354440741fd9)

This [Zellij][zellij] plugin is a "mini-ci". It opens your default `$EDITOR` in a new tab pointed to a `.multitask` file. Any command pasted to this file will run upon save. 

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
3. From within Zellij, run `zellij action start-or-reload-plugin file:~/zellij-plugins/multitask.wasm`

## Development

Load the `dev.kdl` layout from inside zellij: `zellij action new-tab -l dev.kdl` or from outside Zellij with `zellij -l dev.kdl`
