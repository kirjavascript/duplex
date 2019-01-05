// used in trainer + subsets
//
// 10:23 <+Kirjava> I know how to enumerate when the mask is just permutation
// 10:24 <+Kirjava> but orientation is a tricky one
// 10:24 <+Kirjava> maybe I do two phase enumeration
//
// TODO: get orientaions, permutations
//
// get CLL, edge lsit of edges for swap and no swap
//
// send list of json_ll + index to frontend to filter
// send mask back to respond with results
// CO solved and H have edge cases
//
// get_ll
// get_ll_index
// get_ll_json

use crate::cube::*;

// ULB UBR URF UFL
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

// static EP_PARITY

pub fn test() {
    let mut cubes = Vec::new();

    for (co_i, co) in CO.iter().enumerate() {
        for (cp_i, cp) in CP.iter().enumerate() {
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

            // TODO: are all 48 CLL cases needed?
            let valid_solved = co_i != 0 || (cp_i != 1 && cp_i != 3 && cp_i != 4);
            let valid_H = co_i != 7 || (cp_i != 1 && cp_i != 4);

            if valid_solved && valid_H {
                cubes.push(cube);
            }
        }
    }

    println!("CLL {:#?}", cubes.len());
}
