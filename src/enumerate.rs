// used in trainer + subsets
//
// 10:23 <+Kirjava> I know how to enumerate when the mask is just permutation
// 10:24 <+Kirjava> but orientation is a tricky one
// 10:24 <+Kirjava> maybe I do two phase enumeration
//
// TODO: remove rotational symmetry
//
// get CLL, edge lsit of edges for swap and no swap
// mask -> get list of indexes
//
// send mask back to respond with results

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::cube::*;

static CO: [[u8; 4]; 1] = [
    [0, 0, 0, 0], // solved
    // [2, 2, 2, 0], // sune
    // [0, 1, 1, 1], // antisune
    // [0, 1, 0, 2], // L
    // [2, 1, 0, 0], // U
    // [1, 2, 0, 0], // T
    // [1, 1, 2, 2], // Bruno
    // [1, 2, 1, 2], // H
];

static CP: [(usize, usize); 1] = [
    (0, 0),
    // (0, 1), // back
    // (1, 2), // right
    // (2, 3), // front
    // (3, 0), // left
    // (0, 2), // diag
];


static EO: [[u8; 4]; 1] = [
    [0, 0, 0, 0],
    // [0, 1, 0, 1],
    // [1, 0, 1, 0],
    // [1, 1, 0, 0],
    // [0, 1, 1, 0],
    // [0, 0, 1, 1],
    // [1, 0, 0, 1],
    // [1, 1, 1, 1],
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Case {
    index: String,
    edges: Vec<Edge>,
    corners: Vec<Corner>,
}

#[allow(non_snake_case)]
pub fn get_cases() -> Vec<Case> {

    let mut positions = Vec::new();
    let (EP, EP_PARITY) = get_EP();

    let mut index = 0;
    for (co_i, co) in CO.iter().enumerate() {
        for (cp_i, cp) in CP.iter().enumerate() {
            for ep_i in 0..12 {
                let ep = if cp_i == 0 { &EP } else { &EP_PARITY };
                let ep_transform = &ep[ep_i];
                for eo in &EO {

                    let mut cube = Cube::new();

                    // do cp
                    let (a, b) = cp;
                    if a != b {
                        cube.corners.swap(*a, *b);
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
                        positions.push((index, cube));
                        index += 1;
                    }
                }
            }
        }
    }

    // get unique indexes
    let mut map = HashMap::new();

    console!("LL cases {:#?}", positions.len());
    for (index, cube) in &mut positions {
        map.insert(cube.get_ll_index(), (*index, cube.clone()));
    }
    console!("(unique) {:#?}", map.len());

    // convert to vec
    let mut vec: Vec<(&u64, &(usize, Cube))> = map.iter().collect();
    vec.sort_by(|a, b| (a.1).0.cmp(&(b.1).0));
    let mut vec: Vec<Case> = vec.iter()
        .map(|(k, (_, v))| Case {
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
        .collect();


    // remove rotations
    let mut index = 0;
    while index < vec.len() {
        let mut found = false;
        'next: for i in 0..vec.len() {
            if index == 4 && i == 6 {
                // console!("{:#?} {:#?}", vec[i], vec[index]);
            }
            if i != index && cmp_auf(&vec[i], &vec[index]) {
                found = true;
                break 'next
            }
        }
        if found {
            vec.swap_remove(index);
        } else {
            index += 1;
        }
    }
    console!("(rotate) {:#?}", vec.len());

    vec
}

fn cmp_auf(this: &Case, case: &Case) -> bool {
    let is_same = |edges: &[Edge], corners: &[Corner]| {
        edges == &case.edges[..] && corners == &case.corners[..]
    };
    let edges = &mut this.edges.clone()[..];
    let corners = &mut this.corners.clone()[..];
    for _ in 0..4 {
        rotate(edges, corners);
        if is_same(edges, corners) {
            return true
        }
    }
    false
}

fn rotate(edges: &mut [Edge], corners: &mut [Corner]) {
    for edge in edges.iter_mut() {
        rotate_sticker(&mut edge.0);
        rotate_sticker(&mut edge.1);
    }
    for corner in corners.iter_mut() {
        rotate_sticker(&mut corner.0);
        rotate_sticker(&mut corner.1);
        rotate_sticker(&mut corner.2);
    }
    edges.rotate_left(1);
    corners.rotate_left(1);
}

fn rotate_sticker(sticker: &mut Face) {
    use crate::cube::Face::*;
    std::mem::replace(sticker, match sticker {
        U => U,
        B => R,
        R => F,
        F => L,
        L => B,
        _ => unreachable!()
    });
}

// ULB UBR URF UFL
// UB, UR, UF, UL

// Case {
//     index: "813144912430163",
//     edges: [
//         UB,
//         UF,
//         UL,
//         UR
//     ],
//     corners: [
//         ULB,
//         UBR,
//         URF,
//         UFL
//     ]
// } Case {
//     index: "212150138700883",
//     edges: [
//         UR,
//         UF,
//         UB,
//         UL
//     ],
//     corners: [
//         ULB,
//         UBR,
//         URF,
//         UFL
//     ]
// }
