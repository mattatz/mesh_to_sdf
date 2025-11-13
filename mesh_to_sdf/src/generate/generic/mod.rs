//! Module containing the different `generate_sdf` functions, one for each acceleration structure.
//! Note: These modules use rayon for parallelization and are not available on wasm32.

#[cfg(not(target_arch = "wasm32"))]
pub mod bvh;
#[cfg(not(target_arch = "wasm32"))]
pub mod default;
#[cfg(not(target_arch = "wasm32"))]
pub mod rtree;
#[cfg(not(target_arch = "wasm32"))]
pub mod rtree_bvh;
