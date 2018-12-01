extern crate nom;

mod parser;
mod cube;

use cube::*;
use parser::parse_moves;

fn main() {
    // println!("{:?}", parse_moves("RUR'URU2R'"));
    // mirror / inverse / minverse
    let mut cube = Cube::new();
    // cube.edges[0].flip();
    // cube.corners[0].twist(false);
    // cube.corners[0].twist(true);

    // let cycles = vec![0, 1, 2, 3];

    // for (i, c) in cycles.iter().enumerate() {
    //     if i != 0 {
    //         println!("swap {:#?}", (cycles[i-1], c));
    //     }
    // }

    cube.do_transform(Move {
        layer: Layer::U,
        order: Order::Double,
    }.get_transform());

    println!("{}", cube);
}
