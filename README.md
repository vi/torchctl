# torchctl

A program I use to control LED on my Google Pixel 3 XL smartphone.

* Requires root access
* Probably works only on Google Pixel 3 XL. Checked in Android 9 to 12.
* Allows changing brightness from ultra-low (lower than usual sysfs `brightness=1`) to almost maximal
* Uses CPU-driven PWM (pulse width modulation) for ultra-dim mode
* Auto-switches from maximum brightness back to usual torch mode after 5 seconds (may fail to due to missing wakelock in that state).
* Executable size is only 22 kilobytes (see Github Releases for pre-built one)
* Buildable on stable Rust 1.34 (`Xargo.toml` just for fun). Also checked that it builds fine on 1.63.
* Unfortuantely, on Pixel 3 XL brightness stops being controlled by Android at all after using this tool (or sysfs's brightness knob in general). Reboot the device to make usual torch control work.

## Usage

    Usage: torchctl {serve|up|down|quit}

Start service (does not go background) using `torchctl serve`, then issue commands with `torchctl up` and `torchctl down`.

