# torchctl

A program I use to control LED on my Google Pixel 3 XL smartphone.

* Requires root access
* Probably works only on Google Pixel 3 XL. Checked in Android 9, 10 and 11.
* Allows changing brightness from ultra-low (lower than usual sysfs `brightness=1`) to almost maximal
* Uses CPU-driven PWM (pulse width modulation) for ultra-dim mode
* Auto-switches from maximum brightness back to usual torch mode after 5 seconds
* Executable size is only 22 kilobytes (see Github Releases for pre-built one)
* Buildable on stable Rust 1.34 (`Xargo.toml` just for fun).

## Usage

    Usage: torchctl {serve|up|down|quit}

Start service (does not go background) using `torchctl serve`, then issue commands with `torchctl up` and `torchctl down`.

