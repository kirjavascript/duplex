use cube::*;
use parser::parse_moves;

mod interop;

static mut CUBE: Cube = Cube::new();


// pub fn init() {

//     let moves = parse_moves("rUR'URU2r'").unwrap();

//     let moves_transform = combine_transforms(
//         moves.iter().map(|s|s.get_transform()).collect()
//     );

//     unsafe {
//         CUBE.do_transform(&moves_transform);
//     }

// }

// Rust.duplex.then((a) => console.log(a.display()))
