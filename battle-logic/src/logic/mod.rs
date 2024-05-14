pub mod equipment;
pub mod fight;
pub mod fighter;
pub mod rule;

pub mod prelude {
    pub use crate::logic::equipment::*;
    pub use crate::logic::fight::*;
    pub use crate::logic::fighter::*;
    pub use crate::logic::rule::*;
}