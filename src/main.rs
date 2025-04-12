use std::time::Instant;
use std::io;
use std::io::prelude::*;
    
mod object;

use object::Object;

fn main() {
    print_init();
    let proc_start = Instant::now();
    // convert m to um
    let c = 1e6;

    let position = [0.0, 0.0, 0.0];

    // create a 10 cm side length cube
    let length = (0.10 * c) as u64;
    let width = (0.10 * c) as u64;
    let hight = (0.10 * c) as u64;

    let h = 10_000;
    let temperature = 20.0; // degrees C
    let k = 237.0;

    print!("Building the object... ");
    //create a new object 
    // a block of aluminum at room temp
    // thermal conductivity:K = 237 W/m/K
    let start = Instant::now();
    let mut block = Object::new(position, [length, width, hight], h, temperature, k);
    let finish = start.elapsed();
    println!("done\nTook {finish:?}");


    let filename = String::from("output/block_0.000000000");

    if let Err(msg) = block.write(filename) {
      panic!("Error printing object to file: {msg:?}")
    }

    // the first argument is the time interval, and the second is the ambient temperature
    // tamb is constant for now
    let ambient_temp = 0.0;
    let ttotal: f64 = 100.0;
    let N = 100;
    let dt = ttotal/(N as f64);
    let print_times: Vec<f64> = (1..=10).map(|x| x as f64 * 10.0).collect();


    for i in 1..=N {
        print!("\n+++++++++++++++++++++++++++++++++++++++++++++++\nComputing timestep {i} ({0:.9} s) ...", i as f64*dt);
        let start = Instant::now();
        block.compute_dt(dt, ambient_temp);
        let elapsed = start.elapsed();
        println!("done.\nTook {:?}.", elapsed);

        if print_times.contains(&(i as f64*dt)){
            print!("Printing object to file ... ");
            io::stdout().flush();
            let print_start = Instant::now();
            let mut filename = String::from("output/block_" );
            filename.push_str( &( format!("{:.9}", (i as f64*dt)) ));
        

            if let Err(msg) = block.write(filename) {
              panic!("Error printing object to file: {msg:?}")
            }
            let print_time = print_start.elapsed();
            println!("done\nTook {print_time:?}.");
        }
        println!("-----------------------------------------------");
    }

    let proc_ttol = proc_start.elapsed();
    print!("\nFinishing up.\nTotal elaped time: {proc_ttol:?}\n");
}

fn print_init(){
    print!("\nFinite Difference Oxidized. \nA simple numerical solver for the heat equation.\n\n");
}
