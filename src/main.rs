    
mod object;

use object::Object;

fn main() {
    // convert m to um
    let c = 1e6;

    let position = [0.0, 0.0, 0.0];

    // create a 10 cm side length cube
    let length = (0.10 * c) as u64;
    let width = (0.10 * c) as u64;
    let hight = (0.10 * c) as u64;

    let h = 10_000;

    //create a new object 
    let mut block = Object::new(position, [length, width, hight], h);

    // a block of aluminum at room temp
    // thermal conductivity units: M/m/K
    if let Err(msg) = block.initialize(20.0, 237.0){
        panic!("Error initializing object: {msg:?}");
    }
    

    let filename = String::from("block");

    if let Err(msg) = block.write(filename) {
      panic!("Error printing object to file: {msg:?}")
    }

    

}
