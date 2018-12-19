#[macro_use]
pub mod interop;

use serde_json::json;
use crate::cube::*;
// use self::interop::*;

pub static mut CUBE: Cube = Cube::new();

export_string!(get_cube, unsafe {
    json!({
        "edges": CUBE.edges,
        "corners": CUBE.corners,
        "centres": CUBE.centres,
    }).to_string()
});

export_string!(get_ll, unsafe {
    json!({
        "edges": &CUBE.edges[..4],
        "corners": &CUBE.corners[..4],
    }).to_string()
});
