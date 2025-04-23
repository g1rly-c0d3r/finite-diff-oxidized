use std::io;
use std::io::prelude::*;
use std::time::Instant;

use clap::Parser;
use std::path::PathBuf;

extern crate yaml_rust;

use yaml_rust::YamlLoader;
use yaml_rust::Yaml::*;
use std::string::String;

mod object;

use object::Object;

#[derive(Parser)]
#[command(version, about,
          long_about = "A finite-difference solver of the 3-D heat equation written in rust.")]
struct Cli {
    /// Path to the YAML configuration file
    config_file: PathBuf,

    /// Number of threads,
    #[arg(short, long, default_value_t = 1)]
    threads: u8,

    /// Quiet output
    #[arg(short, long)]
    quiet: bool,
}

#[allow(non_snake_case)]
#[derive(Debug)]
struct Config {
    ambient_temp: f64,

    //largest and smallest timesteps
    max_delta_t: f64,
    min_delta_t: f64,

    //largest tempchange for a given timestep
    // panic!'s if tempchange is too large for min timestep
    max_delta_T: f64,

    // total simulation time in seconds
    sim_time: f64,

    // times to plot the temperature
    plot_times: Vec<f64>,
}


#[allow(non_snake_case)]
fn main() {
    let argv = Cli::parse();

    print_init();

    if !argv.quiet {
        print!("Parsing config file: {} ... ", &argv.config_file.display());
        let _ = io::stdout().flush();
    }

    let sim_config = match parse_config(argv.config_file) {
        Ok(config) => {
            println!("finished.");
            config
        }

        Err(msg) => panic!("Error Parsing Config File: {:?}", msg),
    };


    let proc_start = Instant::now();
    // convert m to um
    let c = 1e6;

    let position = [0.0, 0.0, 0.0];

    // create a 10 cm side length cube
    let length = (0.10 * c) as u64;
    let width = (0.10 * c) as u64;
    let hight = (0.10 * c) as u64;

    let h = 1_000;
    let temperature = 20.0; // degrees C
    let k = 237.0;

    if !argv.quiet {
        print!("Building the object... ");
    };

    //create a new object
    // a block of aluminum at room temp
    // thermal conductivity:K = 237 W/m/K
    let start = Instant::now();
    let mut block = Object::new(position, [length, width, hight], h, temperature, k);
    let finish = start.elapsed();
    if !argv.quiet {
        println!("done\nTook {finish:?}");
    }

    // the first argument is the time interval, and the second is the ambient temperature
    // tamb is constant for now
    let ambient_temp = sim_config.ambient_temp;
    let ttotal: f64 = sim_config.sim_time;
    let mut dt = (sim_config.max_delta_t - sim_config.min_delta_t) / 2.0;

    let print_times: Vec<f64> = sim_config.plot_times;

    let mut curr_time: f64 = 0.0;
    while curr_time < sim_config.sim_time {
        if !argv.quiet {
            print!(
                "\n+++++++++++++++++++++++++++++++++++++++++++++++++++++++\nTime step, dt = {0:.9} s ... ",
                curr_time
            );
        }
        let start = Instant::now();
        let largest_tempchange = block.compute_dt(dt, ambient_temp);
        let elapsed = start.elapsed();

        if largest_tempchange == sim_config.max_delta_T {
            if dt < sim_config.min_delta_t {
                panic!("Maximum temperature change not achiveable with minumum timestep");
            }
            dt -= dt * 0.1;
            continue;
        }


        if !argv.quiet {
            println!("done.\ndt = {dt} s\nTook {:?}.", elapsed);
        };

        dt += dt * 0.1;
        if dt > sim_config.max_delta_t {
            dt = sim_config.max_delta_t;
        }
        curr_time += dt;

        // TODO: make sure all of the print_times get printed
        // (using iter's and curr_time +/- dt
        if print_times.contains(&curr_time)   {
            if !argv.quiet {
                print!("Printing object to file ... ");
                let _ = io::stdout().flush();
            }
            let print_start = Instant::now();
            let mut filename = String::from("output/block_");
            filename.push_str(&(format!("{:.9}", curr_time)));

            if let Err(msg) = block.write(filename) {
                panic!("Error printing object to file: {msg:?}")
            }
            let print_time = print_start.elapsed();
            if !argv.quiet {
                println!("done\nTook {print_time:?}.");
            }
        }
        if !argv.quiet {
            println!("-------------------------------------------------------");
        }
    }

    let proc_ttol = proc_start.elapsed();
    print!("\nFinishing up.\nTotal elaped time: {proc_ttol:?}\n");
}

fn print_init() {
    print!("\nFinite Difference Oxidized. \nA simple numerical solver for the heat equation.\n\n");
}

#[allow(non_snake_case)]
fn parse_config(config_file: PathBuf) -> std::result::Result<Config, String> {
    let contents = match std::fs::read_to_string(config_file) {
        Ok(conts) => conts.to_string(),
        Err(msg) => return Err(msg.to_string()),
    };


    let docs = match YamlLoader::load_from_str(&contents) {
        Ok(yaml) => yaml,
        Err(msg) => return Err(msg.to_string()),
    };

    let ambient_temp: f64 = docs[0]["ambient_temp"].as_f64().ok_or(
        "ambient_temp not valid",
    )?;
    let max_delta_t: f64 = docs[0]["max_timestep"].as_f64().ok_or(
        "max timstep not valid",
    )?;
    let min_delta_t: f64 = docs[0]["min_timestep"].as_f64().ok_or(
        "min timestep not valid",
    )?;
    let max_delta_T: f64 = docs[0]["max_step_tempchange"].as_f64().ok_or(
        "max tempchange not valid",
    )?;

    let mut sim_time: f64 = 0.0;
    let mut plot_times = Vec::<f64>::new();

    for (i, item) in docs[1].clone().into_iter().enumerate() {
        match item {
            Real(num) => {
                return Err(String::from(
                    format!("Not a valid control key at {i}: {num}"),
                ))
            }
            Integer(num) => {
                return Err(String::from(
                    format!("Not a valid control key at {i}: {num}"),
                ))
            }
            String(string) => {
                if string == "plot" {
                    plot_times.push(sim_time);
                } else {
                    return Err(String::from(
                        format!("Not a valid control key at {i}: {string:?}"),
                    ));
                }
            }

            Boolean(tf) => {
                return Err(String::from(
                    format!("Not a valid control key at {i}: {tf:?}"),
                ))
            }
            Array(arr) => {
                return Err(String::from(
                    format!("Not a valid control key at {i}: {arr:?}"),
                ))
            }
            Hash(ref hash) => {
                if hash.keys().nth(0).unwrap().as_str().unwrap() == "wait" {
                    sim_time += docs[1][i]["wait"].as_f64().ok_or("Wait time not valid")?;
                } else {
                    return Err(String::from(
                        format!("Not a valid control key at {i}: {hash:?}"),
                    ));
                }
            }

            Alias(alias) => {
                return Err(String::from(
                    format!("Not a valid control key at {i}: {alias:?}"),
                ))
            }
            Null => {
                return Err(String::from(
                    format!("Not a valid control key at {i}: Null"),
                ))
            }
            BadValue => {
                return Err(String::from(
                    format!("Not a valid control key at {i}: BadValue"),
                ))
            }
        }
    }


    Ok(Config {
        ambient_temp,
        max_delta_t,
        min_delta_t,
        max_delta_T,
        sim_time,
        plot_times,
    })
}
