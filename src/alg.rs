use crate::cube::*;
use crate::parser::parse_moves;

#[derive(Debug)]
pub struct Alg {
    pub moves: Vec<Move>,
    pub transform: Transform,
}

impl Alg {
    pub fn new(input: &str) -> Result<Self, String> {
         let moves = parse_moves(input)?;
         let transform = moves_to_transform(&moves);
         match transform.is_ll_transform() {
             true => Ok(Alg { moves, transform }),
             false => Err("Not an LL alg".to_string()),
         }
    }

    pub fn mirror(&self) -> Self {
        let moves = self.moves.iter().map(|m| {
            match m {
                Move { layer: Layer::R, .. } => Move {
                    layer: Layer::L,
                    order: m.order.flip(),
                },
                Move { layer: Layer::L, .. } => Move {
                    layer: Layer::R,
                    order: m.order.flip(),
                },
                Move { layer: Layer::Rw, .. } => Move {
                    layer: Layer::Lw,
                    order: m.order.flip(),
                },
                Move { layer: Layer::Lw, .. } => Move {
                    layer: Layer::Rw,
                    order: m.order.flip(),
                },
                Move { layer: Layer::X, .. } => Move {
                    layer: Layer::X,
                    order: m.order.clone(),
                },
                _ => Move {
                    layer: m.layer.clone(),
                    order: m.order.flip(),
                }
            }
        }).collect::<Vec<Move>>();
        let transform = moves_to_transform(&moves);
        Alg { moves, transform }
    }

    pub fn invert(&self) -> Self {
        let moves = self.moves.iter().rev().map(|m| Move {
            layer: m.layer.clone(),
            order: m.order.flip(),
        }).collect::<Vec<Move>>();
        let transform = moves_to_transform(&moves);
        Alg { moves, transform }
    }
}
