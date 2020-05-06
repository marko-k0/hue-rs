extern crate hue;
extern crate slog;
extern crate slog_term;
#[macro_use]
extern crate clap;

use clap::App;
use clap::ArgMatches;
use serde_yaml;
use std::process;

use hue::groups::*;
use hue::lights::*;
use hue::scenes::*;
use hue::*;

fn main() {
    let yaml = load_yaml!("bin-cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Err(e) = run(matches) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(matches: ArgMatches) -> Res<()> {
    let ref client = Client::new(matches.value_of("config"));

    match matches.subcommand() {
        ("light", Some(sub_m)) => return run_light(client, sub_m),
        ("group", Some(sub_m)) => return run_group(client, sub_m),
        ("scene", Some(sub_m)) => return run_scene(client, sub_m),
        _ => return Ok(()),
    }
}

fn run_light(client: &Client, matches: &ArgMatches) -> Res<()> {
    match matches.subcommand() {
        ("list", _) => return run_light_list(client),
        ("on", Some(sub_m)) => return run_light_power(client, sub_m, true),
        ("off", Some(sub_m)) => return run_light_power(client, sub_m, false),
        _ => return Ok(()),
    }
}

fn run_light_list(client: &Client) -> Res<()> {
    let light_list = Light::get_lights(client);

    if let Ok(lights) = light_list {
        let light_list_yml = serde_yaml::to_string(&lights).unwrap();
        println!("{}", light_list_yml);
    } else if let Err(e) = light_list {
        return Result::Err(e.into());
    }

    Ok(())
}

fn run_light_power(client: &Client, m: &ArgMatches, power: bool) -> Res<()> {
    if let Some(lights) = m.values_of("light") {
        let vals: Vec<&str> = lights.collect();

        for val in vals {
            let mut light = Light::get_light(client, val.parse()?)?;
            light.state().set_on(power);
            light.update_state()?;
        }
    } else {
        let lights = Light::get_lights(client)?;
        for (_, mut light) in lights {
            light.state().set_on(power);
            light.update_state()?;
        }
    }

    Ok(())
}

fn run_group(client: &Client, m: &ArgMatches) -> Res<()> {
    match m.subcommand() {
        ("list", _) => return run_group_list(client),
        ("on", Some(sub_m)) => return run_group_power(client, sub_m, true),
        ("off", Some(sub_m)) => return run_group_power(client, sub_m, false),
        (_, _) => return Ok(()),
    }
}

fn run_group_list(client: &Client) -> Res<()> {
    let group_list = Group::get_groups(client);

    if let Ok(groups) = group_list {
        let group_list_yml = serde_yaml::to_string(&groups).unwrap();
        println!("{}", group_list_yml);
    } else if let Err(e) = group_list {
        return Result::Err(e.into());
    }

    Ok(())
}

fn run_group_power(client: &Client, m: &ArgMatches, power: bool) -> Res<()> {
    if let Some(groups) = m.values_of("group") {
        let vals: Vec<&str> = groups.collect();
        for val in vals {
            let mut group = Group::get_group(client, val.parse()?)?;
            group.action().set_on(power);
            group.update_state()?;
        }
    } else {
        let groups = Group::get_groups(client)?;
        for (_, mut group) in groups {
            group.action().set_on(power);
            group.update_state()?;
        }
    }

    Ok(())
}

fn run_scene(client: &Client, m: &ArgMatches) -> Res<()> {
    match m.subcommand() {
        ("list", _) => return run_scene_list(client),
        ("on", Some(sub_m)) => return run_scene_power(client, sub_m, true),
        (_, _) => return Ok(()),
    }
}

fn run_scene_list(client: &Client) -> Res<()> {
    let scene_list = Scene::get_scenes(client);

    if let Ok(scenes) = scene_list {
        let scene_list_yml = serde_yaml::to_string(&scenes).unwrap();
        println!("{}", scene_list_yml);
    } else if let Err(e) = scene_list {
        return Result::Err(e.into());
    }

    Ok(())
}

fn run_scene_power(client: &Client, m: &ArgMatches, power: bool) -> Res<()> {
    Ok(())
}
