#[macro_use]
pub mod interop;

use lazy_static::lazy_static;
use crate::cube::*;
use crate::alg::*;
use self::interop::JSString;

static mut CUBE: Cube = Cube::new();
lazy_static! {
    static ref ALGS: Vec<Alg> = Vec::new();
}

#[no_mangle]
unsafe extern "C" fn update_algs(mut algs: JSString) {
    ALGS.clear();
    ALGS.extend(create_alglist(algs.to_string()));

    console!("solve test ~~");
    let transform = Alg::new("RUR'URU2R'", "sune").unwrap().transform;
    CUBE.reset();
    CUBE.do_transform(&transform);

    for first_auf in 0..4 {
        console!("{}", first_auf);
    }
}
