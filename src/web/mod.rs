#[macro_use]
pub mod interop;

use serde::{Serialize, Deserialize};

use lazy_static::lazy_static;

use std::sync::Mutex;
use std::collections::{ HashSet, HashMap };
use std::cmp::Ordering;
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
    static ref SOLUTIONS: Mutex<HashMap<u64, Vec<Solution>>> = Mutex::new(HashMap::new());
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
extern "C" fn get_canonical(subset: JSString) {
    let cases = CASES.lock().unwrap();
    let solutions = SOLUTIONS.lock().unwrap();

    let solutions_str = solutions.iter()
        .map(|s| {
            (s.0, s.1.iter().map(|s| s.index).collect())
        })
        .collect::<HashMap<&u64, Vec<usize>>>();

    let subset_mask: Case = serde_json::from_str(&subset.to_string()).unwrap();

    let cases = cases.iter()
        .filter(|case| enumerate::check_mask(&subset_mask, &case))
        .map(|case| json!({
            "case": case,
            "solutionIndices": solutions_str.get(&case.ll_index),
        }))
        .collect::<Vec<Value>>();

    export_string(&json!(cases).to_string());
}

#[no_mangle]
extern "C" fn get_group_algs(subset: JSString) {
    let cases = CASES.lock().unwrap();
    let solutions = SOLUTIONS.lock().unwrap();

    let subset_mask: Case = serde_json::from_str(&subset.to_string()).unwrap();

    let mut cases = cases.iter()
        .filter(|case| enumerate::check_mask(&subset_mask, &case))
        .map(|case| {
            (case, solutions.get(&case.ll_index))
        })
        .collect::<Vec<(&Case, Option<&Vec<Solution>>)>>();

    cases.sort_by(|a, b| {
        if a.1.is_none() || b.1.is_none() {
            Ordering::Less
        } else {
            a.1.unwrap()[0].name.cmp(&b.1.unwrap()[0].name)
        }
    });

    let cases = cases.iter()
        .map(|(case, indices)| json!({
            "case": case,
            "solutionIndices": (
                indices.map(|s| s.iter().map(|s| s.index).collect::<Vec<usize>>())
            ),
        }))
        .collect::<Vec<Value>>();

    export_string(&json!(cases).to_string());
}

#[no_mangle]
extern "C" fn get_group_reduce(subset: JSString) {
    let cases = CASES.lock().unwrap();
    let solutions = SOLUTIONS.lock().unwrap();

    let subset_mask: Case = serde_json::from_str(&subset.to_string()).unwrap();

    let mut names = HashSet::new();
    for s in solutions.values().flatten() {
        names.insert(&s.name);
    }

    let cases = cases.iter()
        .filter(|case| enumerate::check_mask(&subset_mask, &case))
        .map(|case| {
            (case, solutions.get(&case.ll_index))
        })
        .collect::<Vec<(&Case, Option<&Vec<Solution>>)>>();

    let mut coverage = names.iter()
        .map(|name| {
            let qty = cases.iter().filter(|(_, solutions)| {
                if let Some(solutions) = solutions {
                    for s in solutions.iter() {
                        if s.name == **name {
                            return true;
                        }
                    }
                }
                false
            }).collect::<Vec<_>>().len();
            (name, qty)
        })
        .collect::<Vec<(&&String, usize)>>();

    coverage.sort_by(|a, b| {
        let size = b.1.cmp(&a.1);
        if size == Ordering::Equal {
            a.0.len().cmp(&b.0.len())
        } else {
            size
        }
    });

    let coverage: Vec<&&String> = coverage.iter().map(|d| d.0).collect();

    let mut cases = cases.iter()
        .map(|(case, solutions)| {
            if let Some(solutions) = solutions {

                let mut found = None;

                'top: for name in coverage.iter() {
                    for (i, solution) in solutions.iter().enumerate() {
                        if ***name == solution.name {
                            found = Some((i, &solution.name));
                            break 'top;
                        }
                    }
                }

                let mut indices = solutions.iter().map(|s| s.index).collect::<Vec<usize>>();

                if let Some((index, _)) = found {
                    indices.swap(0, index);
                }

                (case, Some(indices), found.map(|s| s.1))
            } else {
                (case, None, None)
            }
        })
        .collect::<Vec<(&&Case, Option<Vec<_>>, Option<&String>)>>();

    cases.sort_by(|a, b| {
        if a.2.is_none() || b.2.is_none() {
            Ordering::Less
        } else {
            a.2.unwrap().cmp(&b.2.unwrap())
        }
    });

    let cases = cases.iter()
        .map(|(case, indices, _)| json!({
            "case": case,
            "solutionIndices": indices,
        }))
        .collect::<Vec<Value>>();

    export_string(&json!(cases).to_string());
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Solution {
    #[serde(skip_serializing)]
    ll_index: u64,
    #[serde(skip_serializing)]
    index: usize,
    #[serde(skip_serializing)]
    name: String,
    #[serde(skip_serializing)]
    transforms: usize,
    alg: Value,
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
    let indices: HashSet<u64> = cases.iter().map(|x| x.ll_index).collect();

    // get solutions for just one alg (AUF at end, because we invert later)

    let mut solutions: HashMap<u64, Vec<Solution>> = HashMap::new();

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
            let mut check_index = |index| {
                if indices.contains(&index) {
                    let alg = alg.invert();
                    let solution = Solution {
                        ll_index: index,
                        index: hits,
                        name: alg.get_full_name().to_lowercase(),
                        transforms: (alg.mirror as usize
                            + alg.invert as usize),
                        alg: json!([
                            invert_auf(auf),
                            alg.to_json(),
                        ]),
                    };
                    add_solution(index, solution);
                    hits += 1;
                }
            };
            let ll_indices = CUBE.get_ll_indices();
            check_index(ll_indices[0]);
            check_index(ll_indices[1]);
            check_index(ll_indices[2]);
            check_index(ll_indices[3]);
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
                    let mut check_index = |index| {
                        if indices.contains(&index) {
                            let first_alg = first_alg.invert();
                            let second_alg = second_alg.invert();
                            let solution = Solution {
                                ll_index: index,
                                index: hits,
                                name: second_alg.get_full_name().to_lowercase(),
                                transforms: (4 + first_alg.mirror as usize
                                    + first_alg.invert as usize
                                    + second_alg.mirror as usize
                                    + second_alg.invert as usize),
                                alg: json!([
                                    invert_auf(second_auf),
                                    second_alg.to_json(),
                                    invert_auf(first_auf),
                                    first_alg.to_json()
                                ]),
                            };
                            add_solution(index, solution);
                            hits += 1;
                        }
                    };
                    let ll_indices = CUBE.get_ll_indices();
                    check_index(ll_indices[0]);
                    check_index(ll_indices[1]);
                    check_index(ll_indices[2]);
                    check_index(ll_indices[3]);
                }
            }
        }
    }
    console!("tried {} combinations", algs.len() * algs.len() * 16);
    console!("found {} solutions", hits);

    let mut solutions_json: Vec<&Solution> = solutions.values().flatten().collect();

    solutions_json.sort_by(|a, b| {
        a.index.cmp(&b.index)
    });

    export_string(&json!(solutions_json).to_string());

    // sort by canonical
    for solution in solutions.values_mut() {
        solution.sort_by(|a, b| {
            (&a.transforms).cmp(&b.transforms)
        });
    }

    SOLUTIONS.lock().unwrap().clear();
    SOLUTIONS.lock().unwrap().extend(solutions);
}
