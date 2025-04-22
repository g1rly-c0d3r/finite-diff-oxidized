use std::io;
use std::io::prelude::*;
use std::time::Instant;

use clap::Parser;
use std::path::PathBuf;

extern crate yaml_rust;

use yaml_rust::YamlLoader;


mod object;

use object::Object;

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "A finite-difference solver of the 3-D heat equation written in rust."
)]
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
        },

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
    let ambient_temp = 0.0;
    let ttotal: f64 = 100.0;
    let N = 1_000;
    let dt = ttotal / (N as f64);
    let print_times: Vec<f64> = (1..=10).map(|x| x as f64 * 100.0).collect();

    let filename = String::from("output/block_0.000000000");


    for i in 1..=N {
        if !argv.quiet {
            print!("\n+++++++++++++++++++++++++++++++++++++++++++++++++++++++\nComputing timestep {i} ({0:.9} s) ... ", i as f64*dt);
        }
        let start = Instant::now();
        block.compute_dt(dt, ambient_temp);
        let elapsed = start.elapsed();

        if !argv.quiet {
            println!("done.\ndt = {dt} s\nTook {:?}.", elapsed);
        };

        if print_times.contains(&(i as f64 * dt)) {
            if !argv.quiet {
                print!("Printing object to file ... ");
                let _ = io::stdout().flush();
            }
            let print_start = Instant::now();
            let mut filename = String::from("output/block_");
            filename.push_str(&(format!("{:.9}", (i as f64 * dt))));

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
    let contents = match std::fs::read_to_string(config_file){
        Ok(conts) => conts.to_string(),
        Err(msg) => return Err(msg.to_string()),
    };


    let docs = match YamlLoader::load_from_str(&contents){
        Ok(yaml) => yaml,
        Err(msg) => return Err(msg.to_string()),
    };

    let ambient_temp: f64 = docs[0]["ambient_temp"];
    let max_delta_t: f64 = docs[0]["max_timestep"];
    let min_delta_t: f64 = docs[0]["min_timestep"];
    let max_delta_T: f64 = docs[0]["max_step_tempchange"];

    let mut sim_time: f64 = 0.0;
    let mut plot_times = Vec::<f64>::new();

    for (i, item) in docs[1].into_iter().enumerate() {
        if item == "plot" {
            plot_times.push(sim_time);
        }
        else if item == "wait" {
            sim_time += docs[1][i][item];
        }
        else{
            return Err(String::from("invalid key in control script"));
        }
    }
    Ok(
        Config{ 
            ambient_temp,
            max_delta_t,
            min_delta_t,
            max_delta_T,
            sim_time,
            plot_times,
        } )
}







