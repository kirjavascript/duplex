// used in trainer + subsets
//
// 10:23 <+Kirjava> I know how to enumerate when the mask is just permutation
// 10:24 <+Kirjava> but orientation is a tricky one
// 10:24 <+Kirjava> maybe I do two phase enumeration
//
// remove rotational symmetry
// TODO: just check ZBLL cases
// TODO: test solved cube is fine
//
// get CLL, edge lsit of edges for swap and no swap
// mask -> get list of indexes
//
// send mask back to respond with results

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::cube::*;

// ULB UBR URF UFL
// UB, UR, UF, UL

static CO: [[u8; 4]; 8] = [
    [0, 0, 0, 0], // solved
    [2, 2, 2, 0], // sune
    [0, 1, 1, 1], // antisune
    [0, 1, 0, 2], // L
    [2, 1, 0, 0], // U
    [1, 2, 0, 0], // T
    [1, 1, 2, 2], // Bruno
    [1, 2, 1, 2], // H
];

static CP: [(usize, usize); 6] = [
    (0, 0),
    (0, 1), // back
    (1, 2), // right
    (2, 3), // front
    (3, 0), // left
    (0, 2), // diag
];


static EO: [[u8; 4]; 8] = [
    [0, 0, 0, 0],
    [0, 1, 0, 1],
    [1, 0, 1, 0],
    [1, 1, 0, 0],
    [0, 1, 1, 0],
    [0, 0, 1, 1],
    [1, 0, 0, 1],
    [1, 1, 1, 1],
];

macro_rules! edge_cycles {
    ( $( $x:expr ),* ) => {
        Transform {
            edge_cycles: vec![$($x),*],
            edge_flips: vec![],
            corner_cycles: vec![vec![]],
            corner_twists: vec![],
            centre_cycles: vec![],
        }
    };
}

#[allow(non_snake_case)]
fn get_EP() -> ([Transform; 12], [Transform; 12]) {
    let EP = [
        // solved
        edge_cycles![vec![]],
        // 1x H perm
        edge_cycles![vec![0, 2], vec![1, 3]],
        // 2x Z perm
        edge_cycles![vec![0, 3], vec![1, 2]],
        edge_cycles![vec![1, 0], vec![2, 3]],
        // 8* U perm
        edge_cycles![vec![0, 1, 2]],
        edge_cycles![vec![2, 1, 0]],
        edge_cycles![vec![1, 2, 3]],
        edge_cycles![vec![3, 2, 1]],
        edge_cycles![vec![2, 3, 0]],
        edge_cycles![vec![0, 3, 2]],
        edge_cycles![vec![3, 0, 1]],
        edge_cycles![vec![1, 0, 3]],
    ];
    let EP_PARITY = [
        // 4* adj swap
        edge_cycles![vec![0, 1]],
        edge_cycles![vec![1, 2]],
        edge_cycles![vec![2, 3]],
        edge_cycles![vec![3, 0]],
        // 2* opp swap
        edge_cycles![vec![0, 2]],
        edge_cycles![vec![1, 3]],
        // 2* rotation
        edge_cycles![vec![0, 1, 2, 3]],
        edge_cycles![vec![3, 2, 1, 0]],
        // 4* weird swap
        edge_cycles![vec![0, 2, 3, 1]],
        edge_cycles![vec![0, 2, 1, 3]],
        edge_cycles![vec![2, 0, 1, 3]],
        edge_cycles![vec![2, 0, 3, 1]],
    ];
    (EP, EP_PARITY)
}

#[derive(Serialize, Deserialize)]
pub struct Case {
    index: String,
    edges: Vec<Edge>,
    corners: Vec<Corner>,
}

#[allow(non_snake_case)]
pub fn get_cases() -> Vec<Case> {

    let mut cubes = Vec::new();
    let (EP, EP_PARITY) = get_EP();

    for (co_i, co) in CO.iter().enumerate() {
        let ep = if co_i == 0 { &EP } else { &EP_PARITY };
        for (cp_i, cp) in CP.iter().enumerate() {
            for ep_i in 0..12 {
                let ep_transform = &ep[ep_i];
                for eo in &EO {

                    let mut cube = Cube::new();

                    // do cp
                    let (a, b) = cp;
                    if a != b {
                        cube.edges.swap(*a, *b);
                    }
                    // do co
                    for (i, rot) in co.iter().enumerate() {
                        match rot {
                            2 => cube.corners[i].twist(Twist::Acw),
                            1 => cube.corners[i].twist(Twist::Cw),
                            _ => {},
                        }
                    }
                    // do ep
                    cube.do_transform(ep_transform);
                    // do eo
                    for (i, flip) in eo.iter().enumerate() {
                        if flip == &1 {
                            cube.edges[i].flip();
                        }
                    }


                    // are all 48 CLL cases needed?
                    let valid_solved = co_i != 0 || (cp_i != 1 && cp_i != 3 && cp_i != 4);
                    let valid_h = co_i != 7 || (cp_i != 1 && cp_i != 4);

                    if valid_solved && valid_h {
                        cubes.push(cube);
                    }
                }
            }
        }
    }

    // get unique cases
    let mut map = HashMap::new();

    let mut f = 0;
    console!("LL cases {:#?}", cubes.len());
    for cube in &mut cubes {
        // vec drain?

        if let Some(exists) = map.get(&cube.get_ll_index()) {
            if f == 0 {
                println!("{}", exists);
                println!("{}", cube);
            }
            f += 1;
        }
        map.insert(cube.get_ll_index(), cube.clone());
    }
    console!("LL cases unique {:#?}", map.len());

    // convert to vec
    map.iter()
        .map(|(k, v)| Case {
            index: format!("{}", *k),
            edges: v.edges[..4].iter().fold(vec![], |mut acc, cur| {
                acc.push(cur.clone());
                acc
            }),
            corners: v.corners[..4].iter().fold(vec![], |mut acc, cur| {
                acc.push(cur.clone());
                acc
            }),
        })
        .collect()
}
