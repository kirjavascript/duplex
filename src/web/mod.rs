#[macro_use]
pub mod interop;

use lazy_static::lazy_static;

use std::sync::Mutex;
use self::interop::{ JSString, export_string };
use serde_json::json;

use crate::cube::*;
use crate::alg::*;
use crate::enumerate::{ self, Case };


static mut CUBE: Cube = Cube::new();
lazy_static! {
    static ref ALGS: Mutex<Vec<Alg>> = Mutex::new(Vec::new());
    static ref CASES: Mutex<Vec<Case>> = Mutex::new(Vec::new());
}

#[no_mangle]
unsafe extern "C" fn load_algs(algs: JSString) {
    let algset = create_algset(algs.to_string());
    console!("loaded {} transforms", algset.len());
    ALGS.lock().unwrap().clear();
    ALGS.lock().unwrap().extend(algset);
}

#[no_mangle]
extern "C" fn enumerate_ll() {
    let cases = enumerate::get_cases();
    export_string(&json!(&cases).to_string());
    CASES.lock().unwrap().clear();
    CASES.lock().unwrap().extend(cases);
}

#[no_mangle]
unsafe extern "C" fn run_algs() {
    // JSFunc
    console!("combining algs...");

    let do_auf = |index| {
        match index {
            1 => CUBE.do_transform(&UTRANS),
            2 => CUBE.do_transform(&UDBLTRANS),
            3 => CUBE.do_transform(&UPRITRANS),
            _ => {},
        }
    };

    // get indexes

    // get solutions for just one alg (AUF at end, because we invert later)

    let algs = ALGS.lock().unwrap();
    for alg in algs.iter() {
        for auf in 0..4 {
            CUBE.replace(Cube::new());
            CUBE.do_transform(&alg.transform);
            do_auf(auf);
            let sltn = (CUBE.get_ll_index(), auf, alg.invert().to_json());
            console!("{}", json!(sltn).to_string());
        }
    }

    // inverse solution
    //22:33 <+Kirjava> so if I invert the cases
// 22:33 <+Kirjava> I can't check for the ones that that are solved in just a single alg
// do them first (all mirrors/inverses)
}

// #[no_mangle]
// extern "C" fn explore_alg(mut input: JSString) {
    // console!("{}", input);
// }

#[no_mangle]
unsafe extern "C" fn explore_solve(input: JSString) {
    console!("solving start");

    let position: Cube = serde_json::from_str(&input.to_string())
        .expect("malformed cube");

    // TODO: setup

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
            CUBE.replace(position.clone());
            do_auf(first_auf);
            CUBE.do_transform(&first_alg.transform);

            if CUBE.is_ll_solved() {
                console!("success: {} {:?}", first_auf, first_alg.moves);
            }

            for second_auf in 0..4 {
                for second_alg in algs.iter() {
                    CUBE.replace(position.clone());
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
