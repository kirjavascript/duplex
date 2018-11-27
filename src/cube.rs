pub enum Order {
    Normal,
    Prime,
    Double,
}

#[derive(Debug)]
pub enum Layer {
    U, F, R, L, B, D, M, E, S,
    Uw, Fw, Rw, Lw, Bw, Dw, X, Y, Z,
}

// impl Layer {
//     fn is_rotation(&self) -> bool {
//         match self {
//             Layer::X | Layer::Y | Layer::Z => true,
//             _ => false,
//         }
//     }
// }

pub struct Move {
    order: Order,
    layer: Layer,
}

pub struct Transform {

}

pub struct Alg {
    text: String,
    moves: Vec<Move>,
    transform: Transform,
}

pub struct Cube {

}
