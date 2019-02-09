use crate::cube::*;
use crate::parser::parse_moves;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MirrorType {
    FB, LR, Yes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSONAlg {
    name: String,
    moves: String,
    mirror: Option<MirrorType>,
    invert: bool,
    #[serde(default)]
    length: usize,
}

pub fn create_algset(data: String) -> Result<Vec<Alg>, String> {
    let json_algs: Vec<JSONAlg> = serde_json::from_str(&data).unwrap();
    let mut algs = Vec::new();
    for json_alg in json_algs {
        let alg = Alg::new(&json_alg)?;
        if json_alg.invert {
            algs.push(alg.invert());
        }
        if let Some(type_) = json_alg.mirror {
            algs.push(alg.mirror(&type_));
            if json_alg.invert {
                algs.push(alg.mirror(&type_).invert());
            }
        }

        algs.push(alg);
    }
    Ok(algs)
}

macro_rules! conjugate {
    ($conjugation:expr, $conjugates:expr, $json_alg:expr) => {
        let mut conj = $json_alg.clone();
        conj.moves = format!($conjugation, &conj.moves);
        conj.name = format!($conjugation, &conj.name);
        if let Ok(alg) = Alg::new(&conj) {
            $conjugates.push(alg);
        }
    };
}

fn _get_conjugates(json_alg: &JSONAlg) -> Vec<Alg> {
    let mut conjugates = Vec::new();
    conjugate!("R {} R'", conjugates, json_alg);
    conjugate!("R' {} R", conjugates, json_alg);
    conjugate!("L {} L'", conjugates, json_alg);
    conjugate!("L' {} L", conjugates, json_alg);
    conjugate!("F {} F'", conjugates, json_alg);
    conjugate!("F' {} F", conjugates, json_alg);
    conjugate!("B {} B'", conjugates, json_alg);
    conjugate!("B' {} B", conjugates, json_alg);
    conjugates
}

#[derive(Debug)]
pub struct Alg {
    pub name: String,
    pub moves: Vec<Move>,
    pub transform: Transform,
    pub mirror: bool,
    pub invert: bool,
}

impl Alg {
    pub fn new(json_alg: &JSONAlg) -> Result<Self, String> {
         let moves = parse_moves(&json_alg.moves).map_err(|err| {
             format!("{} ({})", err, json_alg.name)
         })?;
         let transform = moves_to_transform(&moves);
         match transform.is_ll_transform() {
             true => Ok(Alg {
                 moves,
                 transform,
                 name: json_alg.name.to_owned(),
                 mirror: false,
                 invert: false
             }),
             false => Err(format!("Not an LL alg {} ({})", &json_alg.moves, &json_alg.name)),
         }
    }

    pub fn get_full_name(&self) -> String {
        format!(
            "{}{}{}",
            (if self.invert { "invert " } else { "" }),
            (if self.mirror { "mirror " } else { "" }),
            self.name,
        )
    }

    fn mirror(&self, type_: &MirrorType) -> Self {
        let moves = self.moves.iter().map(|m| match type_ {
                MirrorType::FB => Self::mirror_fb(m),
                MirrorType::LR => Self::mirror_lr(m),
                _ => unreachable!(),
            }).collect::<Vec<Move>>();
        let transform = moves_to_transform(&moves);
        Alg {
            moves,
            transform,
            name: self.name.clone(),
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
            name: self.name.clone(),
            mirror: self.mirror,
            invert: !self.invert,
        }
    }

    pub fn to_json(&self) -> JSONAlg {
        let moves = self.moves.iter()
            .map(|x|format!("{:?}", x))
            .collect::<Vec<String>>()
            .join(" ");
        JSONAlg {
            name: self.name.clone(),
            moves,
            mirror: if self.mirror {
                Some(MirrorType::Yes)
            } else {
                None
            },
            invert: self.invert,
            length: self.moves.len(),
        }
    }

    fn mirror_lr(m: &Move) -> Move {
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
    }

    fn mirror_fb(m: &Move) -> Move {
        match m {
            Move { layer: Layer::B, .. } => Move {
                layer: Layer::F,
                order: m.order.flip(),
            },
            Move { layer: Layer::F, .. } => Move {
                layer: Layer::B,
                order: m.order.flip(),
            },
            Move { layer: Layer::Bw, .. } => Move {
                layer: Layer::Fw,
                order: m.order.flip(),
            },
            Move { layer: Layer::Fw, .. } => Move {
                layer: Layer::Bw,
                order: m.order.flip(),
            },
            Move { layer: Layer::Z, .. } => Move {
                layer: Layer::Z,
                order: m.order.clone(),
            },
            _ => Move {
                layer: m.layer.clone(),
                order: m.order.flip(),
            }
        }
    }
}
