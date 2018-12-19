#[macro_use]
pub mod interop;

use serde_json::json;
use crate::cube::*;
// use self::interop::*;

static mut CUBE: Cube = Cube::new();

export_string!(get_cube, unsafe { format!("{}", CUBE) });

export_string!(get_ll, unsafe {
    json!({
        "edges": &CUBE.edges[..4],
        "corners": &CUBE.corners[..4],
    }).to_string()
});
