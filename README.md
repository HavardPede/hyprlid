# hyprlid
A daemon used with hyprland to disable and enable laptop screen when docked

## Requirements
- hyprland
- rust

## Installation
Setup is simple. call the following, to build and install the daemon:
```
./setup.sh
```
It will 
- build the binary and copy it to ~/.local/bin/hyprlid
- copy the systemd user service to ~/.config/systemd/user/hyprlid.service
- enable the service

## How does it work?
This daemon spins up after the graphical session starts; hyprland.
It will check if the lid monitor (eDP by default) is connected. You can check your monitor names by running:
```
hyprctl monitors
```


Then it will listen to the login1 manager for lid events, and enable/disable the lid monitor accordingly.
The toggling is done by using "hyprctl keyword monitor" command.
