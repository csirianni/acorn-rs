//! This crate provides high-level bindings to Faiss, the
//! vector similarity search engine.
//! 
//! # Preparing
//! 
//! This crate requires Faiss and the C API to be built beforehand by the
//! developer. Please follow the instructions 
//! [here](https://github.com/facebookresearch/faiss/blob/master/INSTALL.md),
//! and build the dynamic library with the C API (instructions
//! [here](https://github.com/facebookresearch/faiss/blob/master/c_api/INSTALL.md))
//! 
//! This will result in the dynamic library `faiss_c` ("libfaiss_c.so" in Linux),
//! which needs to be installed in a place where your system will pick up. In
//! Linux, try somewhere in the `LD_LIBRARY_PATH` environment variable, such as
//! "/usr/lib", or try adding a new path to this variable.
//! 
//! ## GPU support
//! 
//! In order to have GPU support, the `gpufaiss_c` library from the main project
//! needs to be built instead. Then, enable the `gpu` Cargo feature for this crate.
//! 
//! ```toml
//! [dependencies]
//! "faiss" = {version = "0.1.0", features = ["gpu"]}
//! ```
//! 
//! # Examples
//!
//! The [`Index`] trait is one of the center-pieces of this library. Index
//! implementations can be requested using the [`index_factory`] function:
//!
//! [`Index`]: index/trait.Index.html
//! [`index_factory`]: index/fn.index_factory.html
//! 
//! ```no_run
//! use faiss::{Index, index_factory, MetricType};
//! # use faiss::error::Result;
//! # fn run() -> Result<()> {
//! let mut index = index_factory(8, "Flat", MetricType::L2)?;
//! # let my_data = unimplemented!();
//! index.add(my_data)?;
//! # let my_query = unimplemented!();
//! let result = index.search(my_query, 5)?;
//! for (i, (l, d)) in result.labels.iter()
//!     .zip(result.distances.iter())
//!     .enumerate()
//! {
//!     println!("#{}: {} (D={})", i + 1, *l, *d);
//! }
//! # Ok(())
//! # }
//! # run().unwrap()
//! ```
//! 
//! With GPU support, use the [`to_gpu`] and [`to_cpu`] methods 
//! to move an index to and from the GPU.
//! 
//! [`to_gpu`]: index/struct.IndexImpl.html#method.to_gpu
//! [`to_cpu`]: index/gpu/struct.GpuIndexImpl.html#method.to_cpu
//! 
//! ```
//! # #[cfg(feature = "gpu")]
//! use faiss::{GpuResources, StandardGpuResources, Index, index_factory, MetricType};
//! # use faiss::error::Result;
//!
//! # #[cfg(feature = "gpu")]
//! # fn run() -> Result<()> {
//! let index = index_factory(8, "Flat", MetricType::L2)?;
//! let mut gpu_res = StandardGpuResources::new()?;
//! let index = index.to_gpu(&mut gpu_res, 0)?;
//! # Ok(())
//! # }
//! # #[cfg(feature = "gpu")]
//! # run().unwrap()
//! ```
//! 
//! Details from the official Faiss APIs still apply. Please visit
//! the [Faiss wiki](https://github.com/facebookresearch/faiss/wiki)
//! for additional guidance.
//! 

extern crate faiss_sys;

macro_rules! faiss_try {
    ($e: expr) => {{
        let c = $e;
        if c != 0 {
            return Err(::error::Error::from_last_error(c));
        }
    }}
}

pub mod cluster;
pub mod error;
pub mod index;
pub mod metric;

#[cfg(feature = "gpu")]
pub mod gpu;

pub use index::{Index, index_factory};
pub use metric::MetricType;

#[cfg(feature = "gpu")]
pub use gpu::{GpuResources, StandardGpuResources};
#[cfg(feature = "gpu")]
pub use index::gpu::GpuIndexImpl;
