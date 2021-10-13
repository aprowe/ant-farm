pub mod utils;
pub mod pool;
pub mod breeder;
pub mod neat;
pub mod nested_breeder;

pub use crate::neat::NeatNetwork;
pub use crate::neat::NeatBreeder;
pub use crate::neat::NeatGenome;
pub use crate::pool::Pool;
pub use crate::breeder::*;
pub use crate::nested_breeder::*;
pub use evo_macros::derive_breeder;

mod pool2;
pub use pool2::Pool as Pool2;
