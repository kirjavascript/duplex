#[macro_use]
pub mod interop;

use lazy_static::lazy_static;

use std::sync::Mutex;
use std::collections::{ HashSet, HashMap };
use self::interop::{ JSString, export_string };
use serde_json::json;
use serde_json::value::Value;

use crate::cube::*;
use crate::alg::*;
use crate::enumerate::{ self, Case };


static mut CUBE: Cube = Cube::new();
lazy_static! {
    static ref ALGS: Mutex<Vec<Alg>> = Mutex::new(Vec::new());
    static ref CASES: Mutex<Vec<Case>> = Mutex::new(Vec::new());
    static ref SUBSET: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

#[no_mangle]
extern "C" fn enumerate_ll() {
    let cases = enumerate::get_cases();
    export_string(&json!(&cases).to_string());
    CASES.lock().unwrap().clear();
    CASES.lock().unwrap().extend(cases);
}

#[no_mangle]
extern "C" fn load_algs(algs: JSString) {
    match create_algset(algs.to_string()) {
        Ok(algset) => {
            console!("loaded {} transforms", algset.len());
            ALGS.lock().unwrap().clear();
            ALGS.lock().unwrap().extend(algset);
        },
        Err(err) => {
            export_string(&err);
        },
    }
}

#[no_mangle]
extern "C" fn get_random_from_subset() {
    let subset = SUBSET.lock().unwrap();
    let index = &subset[interop::range_random(0..subset.len())];
    console!("{}", index);
}

#[no_mangle]
extern "C" fn load_subset(subset: JSString) {
    let subset_mask: Case = serde_json::from_str(&subset.to_string()).unwrap();
    let cases = CASES.lock().unwrap();
    let subset: Vec<String> = cases.iter()
        .filter(|case| enumerate::check_mask(&subset_mask, &case))
        .map(|case| case.index.clone())
        .collect();
    export_string(&json!(&subset).to_string());
    SUBSET.lock().unwrap().clear();
    SUBSET.lock().unwrap().extend(subset);
}

#[no_mangle]
unsafe extern "C" fn run_algs() {
    console!("combining algs");

    let do_auf = |index| match index {
        1 => CUBE.do_transform(&UTRANS),
        2 => CUBE.do_transform(&UDBLTRANS),
        3 => CUBE.do_transform(&UPRITRANS),
        _ => {},
    };

    let invert_auf = |index| match index {
        1 => 3,
        3 => 1,
        _ => index,
    };

    // get indexes

    let cases = CASES.lock().unwrap();
    let indices: HashSet<u64> = cases.iter().map(|x| {
        x.index.parse::<u64>().unwrap()
    }).collect();

    // get solutions for just one alg (AUF at end, because we invert later)

    let mut solutions: HashMap<u64, Vec<Value>> = HashMap::new();

    let mut add_solution = |index, solution| {
        match solutions.get_mut(&index) {
            Some(vec) => {
                vec.push(solution);
            },
            None => {
                solutions.insert(index, vec![solution]);
            },
        }
    };

    let mut hits = 0;
    // invert algs before and after
    let algs: Vec<Alg> = ALGS.lock().unwrap()
        .iter()
        .map(|a| a.invert())
        .collect();
    for alg in algs.iter() {
        for auf in 0..4 {
            CUBE.replace(Cube::new());
            CUBE.do_transform(&alg.transform);
            do_auf(auf);

            let index = CUBE.get_ll_index();
            if indices.contains(&index) {
                hits += 1;
                let solution = json!({
                    "index": index.to_string(),
                    "solution": [
                        invert_auf(auf),
                        alg.invert().to_json(),
                    ]
                });
                add_solution(index, solution);
            }
        }
    }

    // get solutions for both algs

    for first_alg in algs.iter() {
        for first_auf in 0..4 {
            for second_alg in algs.iter() {
                for second_auf in 0..4 {
                    CUBE.replace(Cube::new());
                    CUBE.do_transform(&first_alg.transform);
                    do_auf(first_auf);
                    CUBE.do_transform(&second_alg.transform);
                    do_auf(second_auf);
                    let index = CUBE.get_ll_index();
                    if indices.contains(&index) {
                        hits += 1;
                        let solution = json!({
                            "index": index.to_string(),
                            "solution": [
                                invert_auf(second_auf),
                                second_alg.invert().to_json(),
                                invert_auf(first_auf),
                                first_alg.invert().to_json()
                            ]
                        });
                        add_solution(index, solution);
                    }
                }
            }
        }
    }
    console!("tried {} combinations", algs.len() * algs.len() * 16);
    console!("found {} solutions", hits);

    // convert u64 to string in case JSON does anything weird
    let solutions = solutions.iter()
        .map(|s| (s.0.to_string(), s.1))
        .collect::<HashMap<String, &Vec<Value>>>();

    export_string(&json!(solutions).to_string());
}
