pub mod fighters;
pub mod rules;
pub mod weapons;
pub mod effects;

pub mod prelude {
    pub use crate::predefined::fighters::*;
    pub use crate::predefined::rules::*;
    pub use crate::predefined::weapons::*;
    pub use crate::predefined::effects::*;
}