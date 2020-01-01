// mod mod_math;
use crate::mod_math;

pub fn use_add() -> u32{
    let x = 1;
    let y = 2;
    let z = mod_math::add(x, y);
    z
}