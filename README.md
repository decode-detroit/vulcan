# Vulcan
#### Realtime Remote-Controlled DMX Controller

This dmx controller is designed for realtime control of dmx signals in theatrical and interactive applications. Control a dmx universe over http, either on the same computer or from the web.

## Getting Started

If you're on a 64-bit GNU/Linux system, you can use the the [binary release here](https://github.com/decode-detroit/apollo/releases).

Binary releases for other systems are a work in progress. In particular, a bug in Tokio Serial prevents this program from working properly on Windows. In the meantime, you can compile Vulcan from source:

### Prerequisites

You'll need Rust to compile and run Vulcan.

* Installation of Rust: https://www.rust-lang.org/

Follow the directions to download and install Rust before you proceed.

### Compiling

Once you have installed the prerequities above, clone or download this repository. Then compile and run the program using Cargo (included with Rust):
```
cargo run -- -p /dev/ttyUSB0
```

This will take several minutes to download all the components. You'll be left with a running Vulcan instance in the background. You may need to change the specified path to the hardward to something other than /dev/ttyUSB0 to match the location of your hardware.

You can use
```
cargo run -- -p /dev/ttyUSB0
```

to run Vulcan again (it will not recompile this time). This is a debug version (larger file, but otherwise perfectly functional).

To compile a finished copy for deployment, use
```
cargo build --release
```

The completed binary will be located in the automatically generated "target/release" folder with the name "vulcan".

## Usage

To cue DMX changes on Vulcan, you need to specify a path to the DMX hardware interface. Vulcan supports the DMX King USB hardware interface. Support for other hardware will likely be added in the future.

Once the program is started, you can control the interface with two commands (more coming in the future):
1. Play a DMX fade (fading from the current value to a specified future value over a set time)
2. Set the value of all the channels at once (useful for initial setting or resuming)

### Play Fade Options

Here are the play fade options:
* channel: the DMX channel (out of 512) that will be modified by the fade.
* value: the final 8-bit value of the channel (for a light fixture, typically 0 is off and 255 is full brightness)
* duration: a two element field that specifies the seconds and nano seconds (secs and nanos are field names) that the controller should take to arrive at this new value. The controller will fade from the current value of the channel to this new value linearly. More elaborate fades and animations may be available in the future.

### Load Universe Options

The load universe specifies a value for every channel in a DMX universe at once. This option expects an array of 512 values.

### RESTful API

You can cue fades and load DMX values using the two available POST commands on localhost port 88522 (V-U-L-C-A). An example interaction might look like this:

```
curl -H "Content-Type: application/json" -X POST -d '{ "universe", [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}' http://localhost:88522/loadUniverse
curl -H "Content-Type: application/json" -X POST -d '{ "channel": 1, "value": 255, duration { "secs": 1, "nanos": 0 }}' http://localhost:88522/playfade
```

The DMX channels are initialized to zero, so the first command above does nothing on a new instance of Vulcan.

The port number (and listening location) can be adjusted with the '-a' or '--address' commandline option, and log level can be set via the '-l' or '--logLevel' option. Log levels are Trace, Info, Debug, Warn, Error (listed in decreasing level of verbosity).

Remember that you always need to specify a path to the DMX hardware (with option '-p' or '--path') for the program to load.

If you need to make Vulcan available to the open internet, we recommend [Caddy](https://caddyserver.com/). Follow the instructions for setting up a reverse proxy (it will take less than 60 seconds).

In the future, additional fade animations and other features will be added based on our own needs. If you are using Vulcan and have a specific feature you need, feel free to send us an email and we'll do our best to make it a priority.

## Realtime Backup

If you would like realtime backup of the dmx controller for intant recovery, install a Redis server on your machine. The most up-to-date instructions for installing Redis can be found here: https://redis.io/.

The default configuration should work just fine, and Vulcan will update the settings to make sure every change is written to the disk. To connect to the backup server, use the commandline option '-b' or '--backup'. The typical server location is redis://127.0.0.1:6379.

## Raspberry Pi-like Systems (ARM)

It's possible to run vulcan on less-capible systems! It should be fully featured on all systems it will compile for, but has only been tested on a full size PC.

Note: These instructions are written for *compiling* the software on Ubuntu 22.04.

### Cross-Compiling for Raspberry Pi (armhf, 32bit)

To cross-compile, install the correct rust target and install the linker.
```
rustup target add armv7-unknown-linux-gnueabihf
sudo apt install gcc-arm-linux-gnueabihf
```
You'll also need to add the armhf architecture to dpkg.
```
sudo dpkg --add-architecture armhf
```
And add these sources to the end of /etc/apt/sources.list.
```
deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports/ jammy main restricted
deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports/ jammy-updates main restricted+
deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports/ jammy universe
deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports/ jammy-updates universe
deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports/ jammy multiverse
deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports/ jammy-updates multiverse
```
Make sure to add `[arch=amd64]` to the original sources while you're at it.

When you compile, pass several environment variables to the compilation.
```
env PKG_CONFIG_ALLOW_CROSS=1 PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig/ cargo build_armhf
```

### Cross-Compiling for Raspberry Pi (aarch64/arm64, 64bit)

To cross-compile, install the correct rust target and install the linker.
```
rustup target add aarch64-unknown-linux-gnu
sudo apt install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu 
```
You'll also need to add the arm64 architecture to dpkg.
```
sudo dpkg --add-architecture arm64
```
And add these sources to the end of /etc/apt/sources.list (or if also using 32 bit, combine the two like ```[arch=armhf,arm64]```).
```
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports/ jammy main restricted
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports/ jammy-updates main restricted
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports/ jammy universe
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports/ jammy-updates universe
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports/ jammy multiverse
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports/ jammy-updates multiverse
```
Make sure to add `[arch=amd64]` to the original sources while you're at it.

When you compile, pass several environment variables to the compilation.
```
env PKG_CONFIG_ALLOW_CROSS=1 PKG_CONFIG_PATH=/usr/lib/aarch64-linux-gnu/pkgconfig/ cargo build_arm64
```

## License

This project is licensed under the GNU GPL Version 3 - see the [LICENSE](LICENSE) file for details. This project is closely connected to [Minerva](https://github.com/decode-detroit/minerva).

Thanks to all the wonderful free and open source people out there who have made this project possible, especially Mozilla et al. for a beautiful language and the folks at Gnome, GTK, and GStreamer for their ongoing efforts advance multimedia in open source software.
