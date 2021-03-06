#[cfg(feature = "parallel")]
extern crate rayon;

pub mod world_2d;
pub mod world_2d_climate_simulation;

pub mod prelude {
    pub use crate::world_2d::*;
    pub use crate::world_2d_climate_simulation::*;
}
