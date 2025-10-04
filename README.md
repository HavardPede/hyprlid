# hyprlid
A daemon used with hyprland to disable and enable laptop screen when docked.

## Why does this exist?
Hyprland does not disable the laptop screen when the lid is closed, even when an external monitor is connected.
This means, when plugged into an external monitor and closing the lid, there will be workspaces that are not visible untill you move them manually.

Ultimately, there is a way easier solution to this problem, by using built-in hyprland commands, as seen here: 
https://github.com/hyprwm/Hyprland/issues/5634

But this project allowed me to learn rust, systemd services, and dbus.

## Requirements
- hyprland
- rust

## Installation
Setup is simple. call the following, to build and install the daemon:
```
./setup.sh
```
It will build the binaries and enable the user service.

## How does it work?
This daemon spins up after the graphical session starts; hyprland.
It will check if the lid monitor (eDP by default) is connected. You can check your monitor names by running:
```
hyprctl monitors
```

Then it will listen to the login1 manager for lid events, and enable/disable the lid monitor accordingly.
The toggling is done by using "hyprctl keyword monitor" command.
