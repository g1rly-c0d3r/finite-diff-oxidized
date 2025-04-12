use ndarray::{prelude::*, Array3};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// what we are simulating
pub struct Object {
    // the discretization value of the object, lower values mean finer discretization
    // must be a positive integer
    // physically represents voxel size in microns
    h: u64,
    // Thermal conductivity of the object, in W/m/K
    k: f64,
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
    pub fn new(position: [f64; 3], size: [u64; 3], h:u64, temperature: f64, k: f64) -> Object{
        if h < 1 {
            panic!("Discretization can not be finer than 1 um");
        }

        let x_dim = size[0] / h;
        let y_dim = size[1] / h;
        let z_dim = size[2] / h;

        let object = Array3::<f64>::from_elem( (z_dim as usize, y_dim as usize, x_dim as usize).f(), temperature);
        
        Object{ h, k, position, lengths: size, object }
    }

    pub fn compute_dt(&mut self, timestep: f64, ambient_temperature: f64) -> (){ //SMUT (my wife
                                                                                 //thinks &mut
                                                                                 //looks like smut)
        // This function "computes" one time step (given by the parameter) of the heat equation of
        // the object using the finite difference method:
        // u_tt = u_xx + u_yy + u_zz
        // The book i am using uses u and v for the analytic solution and numerical approximation,
        // respectively.


        // iterate over each voxel in the object
        for i in 0..self.object.shape()[0]{
            for j in 0..self.object.shape()[1]{
                for k in 0..self.object.shape()[2]{
                    // The finite difference second derivatives for each spacial dimension.
                    // Bounds checked so that the "outside" of the object is treated as a sink.
                   let v_x = match i {
                       0 => (ambient_temperature - 2.0*self.object[[i,j,k]] + self.object[[i+1,j,k]]) / (self.h*self.h) as f64,
                       xmax if xmax == self.object.shape()[0]-1  => (ambient_temperature - 2.0*self.object[[i,j,k]] + self.object[[i-1,j,k]]) / (self.h*self.h) as f64,
                       _ => (self.object[[i+1, j, k]] - 2.0*self.object[[i,j,k]] + self.object[[i-1,j,k]]) / (self.h*self.h) as f64,
                   };

                   let v_y = match j {
                       0 => (ambient_temperature - 2.0*self.object[[i,j,k]] + self.object[[i,j+1,k]]) / (self.h*self.h) as f64,
                       ymax if ymax == self.object.shape()[1]-1  => (ambient_temperature - 2.0*self.object[[i,j,k]] + self.object[[i,j-1,k]]) / (self.h*self.h) as f64,
                       _ => (self.object[[i, j+1, k]] - 2.0*self.object[[i,j,k]] + self.object[[i,j-1,k]]) / (self.h*self.h) as f64,
                   };

                   let v_z = match k {
                       0 => (ambient_temperature - 2.0*self.object[[i,j,k]] + self.object[[i,j,k+1]]) / (self.h*self.h) as f64,
                       zmax if zmax == self.object.shape()[2]-1  => (ambient_temperature - 2.0*self.object[[i,j,k]] + self.object[[i,j,k-1]]) / (self.h*self.h) as f64,
                       _ => (self.object[[i, j, k-1]] - 2.0*self.object[[i,j,k]] + self.object[[i,j,k-1]]) / (self.h*self.h) as f64,
                   };

                   // The multiplicitive scalars are the time step correction and the thermal
                   // parameters of the object (density, specific heat, thermal conductivity, etc.).
                   
                   self.object[[i,j,k]] += (self.k*self.k)*timestep*(v_x+v_y+v_z);
                }}}
    }
        

    pub fn write(&self, filename: String) -> std::io::Result<()>{
        let filename_ext = filename + ".txt";
        let path_to_file = Path::new( &filename_ext );

        let mut file = File::create(&path_to_file)?; 

        for x in self.object.outer_iter() {
            for y in x.outer_iter(){
                for z in y.outer_iter(){
                   //println!("{:.2?} ", z[[]]);
                   file.write( &z[[]].to_string().as_bytes() )?;  
                   file.write( " ".as_bytes() )?;
                }
                file.write("\n".as_bytes())?;
            }
            file.write("\n".as_bytes())?;
        }
     Ok(())       
    }


}
