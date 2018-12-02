#[macro_use]
extern crate lazy_static;
extern crate nom;

mod parser;
mod cube;

use cube::*;
use parser::parse_moves;

fn main() {
    let sune = parse_moves("RUR'URU2R'").unwrap();

    let sune_transform = combine_transforms(
        sune.iter().map(|s|s.get_transform()).collect()
    );

    let mut cube = Cube::new();

    println!("{}", cube);
    cube.do_transform(sune_transform);
    println!("{}", cube);


    // cube.edges[0].flip();
    // cube.corners[0].twist(false);
    // cube.corners[0].twist(true);
    //


    // let cycles = vec![0, 1, 2, 3];

    // for (i, c) in cycles.iter().enumerate() {
    //     if i != 0 {
    //         println!("swap {:#?}", (cycles[i-1], c));
    //     }
    // }

    // let transform = Move {
    //     layer: Layer::R,
    //     order: Order::Prime,
    // }.get_transform();

    // println!("{:#?}", transform);

    // cube.do_transform(transform);

    // print cube
    // print alg
    // print cube


    // println!("{}", cube);
}
