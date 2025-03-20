use ndarray::{prelude::*, Array3};

// what we are simulating
pub struct Object {
    // the discretization value of the object, lower values mean finer discretization
    // must be a positive integer
    // physically represents voxel size in microns
    h: u64,
    //the 0'th point in the x,y, and z range
    position: [f64; 3],
    //the "size" of the object, in microns, or whatever units h is in
    lengths: [u64; 3],
    // the object itself, represented as a 3D array
    // the indicies represent a position
    // the value at an index represent temperature
    object: Array3<f64>,
}

impl Object {
    pub fn new(position: [f64; 3], size: [u64; 3], h:u64) -> Object{
        if h < 1 {
            panic!("Discretization can not be finer than 1 um");
        }

        let x_dim = size[0] / h;
        let y_dim = size[1] / h;
        let z_dim = size[2] / h;

        let object = Array3::<f64>::default( (z_dim as usize, y_dim as usize, x_dim as usize).f());
        
        Object{ h, position, lengths: size, object }
    }
}


