#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate hue;
#[macro_use]
extern crate clap;

use std::error::Error;
use std::process;
use clap::App;
use clap::ArgMatches;
use serde_yaml;

use hue::Client;
use hue::lights::*;

fn main() {
    let yaml = load_yaml!("bin-cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Err(e) = run(matches) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(matches: ArgMatches) -> Result<(), Box<Error>> {
    let ref client = Client::new(
        String::from("192.168.200.1"),
        String::from("asdf"),
    );

    match matches.subcommand() {
        ("light", Some(sub_m)) => return run_light(client, sub_m),
        ("group", Some(sub_m)) => return run_group(client, sub_m),
        ("scene", Some(sub_m)) => return run_scene(client, sub_m),
        (_, _) => return Ok(()),
    }

    Ok(())
}

fn run_light(client: &Client, matches: &ArgMatches) -> Result<(), Box<Error>> {
    match matches.subcommand() {
        ("list", _) => return run_light_list(client),
        ("on", Some(sub_m)) => return run_light_power(client, sub_m, true),
        ("off", Some(sub_m)) => return run_light_power(client, sub_m, false),
        (_, _) => return Ok(()),
    }
}

fn run_light_list(client: &Client) -> Result<(), Box<Error>> {
    let light_list = Light::get_lights(&client);

    //if let Ok(lights) = light_list {
    //    let light_list_yml = serde_yaml::to_string(&lights).unwrap();
    //    println!("{}", light_list_yml);
    //} else if let Err(e) = light_list {
    //    format!("{}", e);
    //}

    for light in light_list.unwrap().values() {
        light.test();
    }

    Ok(())
}

fn run_light_power(client: &Client, matches: &ArgMatches, power: bool) -> Result<(), Box<Error>> {
    if let Some(lights) = matches.values_of("light") {
        let vals: Vec<&str> = lights.collect();

        let state = LightStateBuilder::default().on(power).build()?;
        for val in vals {
            let r = Light::set_state(client, val.parse()?,  &state);
            println!("{}", r.unwrap());
        }
    } else {
        //TODO: do this for all the lights
    }
    Ok(())
}

fn run_group(client: &Client, matches: &ArgMatches) -> Result<(), Box<Error>> {
    Ok(())
}

fn run_scene(client: &Client, matches: &ArgMatches) -> Result<(), Box<Error>> {
    Ok(())
}
