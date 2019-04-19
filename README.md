# Philips Hue Rust Library and CLI

**This is currently in development stage!**

## Hue-rs, let there be light!

Hue-rs is a *Rust* library that can be used for home automation on Philips Hue lights.
It offers a basic *CLI* with which you can control your lights from a terminal instead
of looking for your phone and making desired changes from official Philips Hue app.

## Installation and CLI use

```bash
$ cat <<EOF > ~/.huerc
debug = false

[hue]
ip = 192.168.0.1
username = <philips-hue-username>
EOF

$ hue help

hue 1.0
Marko Kosmerl <marko.kosmerl@gmail.com>
Philips Hue CLI

USAGE:
    hue [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Sets the level of verbosity

OPTIONS:
    -c, --config <FILE>    Sets a custom config file

SUBCOMMANDS:
    group    Controls a group of lights
    help     Prints this message or the help of the given subcommand(s)
    light    Controls the lights
    scene    Controls a scene
    
$ hue light on 1
[{"success":{"/lights/1/state/on":true}}]
```

## Library use

```rust
extern crate hue-rs;

use hue-rs::*;
use hue-rs::lights::*;

pub fn funhue() {
    let client = Client::new();

    let mut light = Light::get_light(&client, 1).unwrap();
    let &mut light_state = light.state();
    light_state.set_on(true);
    light.update()
}
```

## Todo

- [ ] Tests
- [ ] Finalize *light*, *group* and *scene* API
- [ ] *Let there be light* voice recognition
