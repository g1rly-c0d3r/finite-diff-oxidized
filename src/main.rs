    
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

    let filename = String::from("block");

    let _ = block.write(filename);

}
