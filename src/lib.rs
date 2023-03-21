pub mod colours;
pub mod data;
pub mod engine;
pub mod model;
pub mod parameters;
pub mod particle;
pub mod run;

pub use self::colours::*;
pub use self::data::*;
pub use self::engine::*;
pub use self::model::*;
pub use self::parameters::*;
pub use self::particle::*;
pub use self::run::*;

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Runs a simulation.
#[pyfunction]
fn simulate(
    num_threads: usize,
    num_steps: usize,
    colour_map: Vec<String>,
    num_neutrons: usize,
    block_size: usize,
    bump_dist: f64,
    min_weight: f64,
    gun_pos: [f64; 3],
    gun_target: [f64; 3],
    gun_spread: f64,
    scat_coeff: f64,
    abs_coeff: f64,
    mins: [f64; 3],
    maxs: [f64; 3],
    num_voxels: [usize; 3],
) -> PyResult<String> {
    let params = parameters::Parameters {
        num_threads,
        num_steps,
        colour_map,
        num_neutrons,
        block_size,
        bump_dist,
        min_weight,
        gun_pos,
        gun_target,
        gun_spread,
        scat_coeff,
        abs_coeff,
        mins,
        maxs,
        num_voxels,
    };

    println!("Running!");

    Ok("Simulation complete.".to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn neutron(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simulate, m)?)?;
    Ok(())
}
