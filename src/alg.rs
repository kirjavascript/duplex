use crate::cube::*;
use crate::parser::parse_moves;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JSONAlg {
    index: usize,
    name: String,
    moves: String,
    mirror: bool,
    invert: bool,
}

pub fn create_algset(data: String) -> Vec<Alg> {
    let json_algs: Vec<JSONAlg> = serde_json::from_str(&data).unwrap();
    let mut algs = Vec::new();
    for json_alg in json_algs {
        // TODO: error handling
        let alg = Alg::new(&json_alg).unwrap();
        if json_alg.mirror {
            algs.push(alg.mirror());
        }
        if json_alg.invert {
            algs.push(alg.invert());
        }
        if json_alg.mirror && json_alg.invert {
            algs.push(alg.mirror().invert());
        }
        algs.push(alg);
    }
    algs
}

#[derive(Debug)]
pub struct Alg {
    // pub index: usize,
    pub name: String,
    pub moves: Vec<Move>,
    pub transform: Transform,
    pub mirror: bool,
    pub invert: bool,
}

impl Alg {
    pub fn new(json_alg: &JSONAlg) -> Result<Self, String> {
         let moves = parse_moves(&json_alg.moves)?;
         let transform = moves_to_transform(&moves);
         match transform.is_ll_transform() {
             true => Ok(Alg {
                 moves,
                 transform,
                 name: json_alg.name.to_owned(),
                 mirror: false,
                 invert: false
             }),
             false => Err(format!("Not an LL alg: {}", &json_alg.moves)),
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
        Alg {
            moves,
            transform,
            name: format!("mirror {}", self.name),
            mirror: !self.mirror,
            invert: self.invert,
        }
    }

    pub fn invert(&self) -> Self {
        let moves = self.moves.iter().rev().map(|m| Move {
            layer: m.layer.clone(),
            order: m.order.flip(),
        }).collect::<Vec<Move>>();
        let transform = moves_to_transform(&moves);
        Alg {
            moves,
            transform,
            name: format!("inverted {}", self.name),
            mirror: self.mirror,
            invert: !self.invert,
        }
    }
}
