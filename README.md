![crates.io](https://img.shields.io/crates/v/roid "crates.io")

``` bash
               (
 (        (    )\ )
 )(    (  )\  (()/(
(()\   )\((_)  ((_))
 ((_) ((_)(_)  _| |
| '_|/ _ \| |/ _` |
|_|  \___/|_|\__,_|

roid 0.1.2
Android Developer Toolkit

USAGE:
    roid [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build      Build an Android project
    device     Android device management
    help       Prints this message or the help of the given subcommand(s)
    install    Install an APK on a device or emulator
```

### Guide

Install with [cargo](https://github.com/rust-lang/cargo):

``` bash
$ cargo install roid
```

Set up a Roid.toml file @ `~/.config/Roid.toml`:

``` toml
# Path to gradle
# defaults to: gradle
gradle = "/usr/bin/gradle"

# Path to Android SDK emulator cli tool: 
# defaults to: emulator
emulator = "/home/{user}/Android/Sdk/emulator/emulator"

# Path to ADB
# defaults to: adb
adb = "/usr/bin/adb"
```

#### Build (compile) project in debug mode

``` bash
$ roid build --debug
```

#### List all devices (both physical and emulators)

``` bash
$ roid device --list
```

#### Install apk on a device

``` bash
$ roid install --apk ./build/outputs/apk/debug/app-debug.apk
```
