``` bash
               (
 (        (    )\ )
 )(    (  )\  (()/(
(()\   )\((_)  ((_))
 ((_) ((_)(_)  _| |
| '_|/ _ \| |/ _` |
|_|  \___/|_|\__,_|

roid 0.1.1
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
    new        Create a new Android project
```

### Requirements 
- Gradle
- Android emulator
- ADB

### Guide

Install with [cargo](https://github.com/rust-lang/cargo):

``` bash
$ cargo install roid
```

Set up a Roid.toml file @ `~/.config/Roid.toml`:

``` toml
# Path to Android Project Templates Repository
# You can use the default repository below, 
# or you can create your own Android project templates (with android studio)
templates = "https://github.com/cy6erlion/android-project-templates.git"

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

#### Creating a new Android project with no activity

```bash
$ roid new --none MyProject
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

⧉
