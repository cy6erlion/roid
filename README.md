``` bash
               (
 (        (    )\ )
 )(    (  )\  (()/(
(()\   )\((_)  ((_))
 ((_) ((_)(_)  _| |
| '_|/ _ \| |/ _` |
|_|  \___/|_|\__,_|

roid 0.1.0
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

- Android Studio
- Android SDK Platform 30
- Gradle
- Java
- Git
- Linux
- Computer

### Guide

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
