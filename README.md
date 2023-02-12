# Displays
Simple helper script that will reorient screens in a 3 monitor setup for an Apple laptop.

It assumes that you have displays physically arranged like so:

```
  +--------------+--------------------------+
  |              |                          |
  |              |                          |
  |              |                          |
  +-------+------+------+-------------------+
          |             |
          | laptop      |
          +-------------+
```

It automatically rearrange the displays so that the laptop screen appears on the bottom
and the two external monitors are side by side on top of the laptop screen. If run 
multiple the two external monitors wil be swapped. 

This script is useful if you have two external displays with the exact same size and resolution.
In this case the OS cannot tell them apart and will randomly assign one to be the top left external 
monitor. 