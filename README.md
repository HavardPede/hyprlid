# hyprlid
A daemon used with hyprland to disable and enable laptop screen when docked

Setup is simple. call the following, to move the files and enable the daemon.
```
./setup.sh
```

If for some reason your laptop monitor has a different name, change the LAPTOP variable in the hyprlid.sh file.
Default is _eDP-1_. You can check by running 
```
hyprctl monitors
```
