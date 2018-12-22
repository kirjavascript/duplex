#[macro_use]
pub mod interop;

use lazy_static::lazy_static;

use std::sync::Mutex;
use crate::cube::*;
use crate::alg::*;
use self::interop::{ JSString, export_string };

static mut CUBE: Cube = Cube::new();
lazy_static! {
    static ref ALGS: Mutex<Vec<Alg>> = Mutex::new(Vec::new());
}

#[no_mangle]
unsafe extern "C" fn load_algs(mut algs: JSString) {
    let algset = create_algset(algs.to_string());
    console!("loaded {} transforms", algset.len());
    ALGS.lock().unwrap().clear();
    ALGS.lock().unwrap().extend(algset);
}

#[no_mangle]
unsafe extern "C" fn explore_solve(mut input: JSString) {
    console!("solving transform");

    let main_transform: Transform = serde_json::from_str(&input.to_string())
        .expect("malformed transform object");

    console!("{:#?}", main_transform);

    let do_auf = |index| {
        match index {
            1 => CUBE.do_transform(&UTRANS),
            2 => CUBE.do_transform(&UDBLTRANS),
            3 => CUBE.do_transform(&UPRITRANS),
            _ => {},
        }
    };
    let algs = ALGS.lock().unwrap();
    for first_auf in 0..4 {
        for first_alg in algs.iter() {
            // first check without a second alg
            CUBE.reset();
            CUBE.do_transform(&main_transform);
            do_auf(first_auf);
            CUBE.do_transform(&first_alg.transform);

            if CUBE.is_ll_solved() {
                console!("success: {} {:?}", first_auf, first_alg.moves);
            }

            for second_auf in 0..4 {
                for second_alg in algs.iter() {
                    CUBE.reset();
                    CUBE.do_transform(&main_transform);
                    do_auf(first_auf);
                    CUBE.do_transform(&first_alg.transform);
                    do_auf(second_auf);
                    CUBE.do_transform(&second_alg.transform);

                    if CUBE.is_ll_solved() {
                        console!("success: {} {:?} {} {:?}",
                            first_auf,
                            first_alg.moves,
                            second_auf,
                            second_alg.moves
                        );
                    }
                }
            }
        }
    }

    console!("solving done");
}

// unsafe fn solve_transform<F>(main_transform: Transform, cb: F) where F: Fn() {
// }
