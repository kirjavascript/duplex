use std::{fmt, mem};

// moves

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Move {
    pub order: Order,
    pub layer: Layer,
}

// cube

#[derive(Debug, PartialEq)]
pub enum Face {
    U,R,F,B,L,D,
}

#[derive(PartialEq)]
pub struct Edge(Face, Face);

impl Edge {
    pub fn flip(&mut self) {
        mem::swap(&mut self.0, &mut self.1);
    }
}

#[derive(PartialEq)]
pub struct Corner(Face, Face, Face);

#[derive(PartialEq)]
pub enum Twist {
    Cw, Acw,
}

impl Corner {
    pub fn twist(&mut self, type_: Twist) {
        mem::swap(&mut self.0, &mut self.1);
        if type_ == Twist::Acw {
            mem::swap(&mut self.1, &mut self.2);
        } else {
            mem::swap(&mut self.0, &mut self.2);
        }
    }
}

#[derive(PartialEq)]
pub struct Cube {
    pub edges: [Edge; 12],
    pub corners: [Corner; 8],
    pub centres: [Face; 6],
}

// transforms

pub struct Transform {
    pub edge_cycles: Vec<Vec<usize>>,
    pub edge_flips: Vec<usize>,
    pub corner_cycles: Vec<Vec<usize>>,
    pub corner_twists: Vec<(usize, Twist)>,
}

fn combine_transforms(transforms: Vec<Transform>) -> Transform {
    let mut ep = [0, 1, 2, 3, 4, 5, 6, 7];
    let mut eo = [0, 0, 0, 0, 0, 0, 0, 0];
    let mut cp = [0, 1, 2, 3, 4, 5, 6, 7];
    let mut co = [0, 0, 0, 0, 0, 0, 0, 0];

    for transform in transforms {
        for edge_cycles in transform.edge_cycles {
            for (i, index_a) in edge_cycles.iter().enumerate() {
                if i != 0 {
                    let index_b = edge_cycles[i-1];
                    ep.swap(*index_a, index_b);
                }
            }
        }
        for i in transform.edge_flips {
            eo[i] = if eo[i] == 1 { 0 } else { 1 };
        }
        for corner_cycles in transform.corner_cycles {
            for (i, index_a) in corner_cycles.iter().enumerate() {
                if i != 0 {
                    let index_b = corner_cycles[i-1];
                    ep.swap(*index_a, index_b);
                }
            }
        }
        for (i, type_) in transform.corner_twists {
            match type_ {
                Twist::Cw => {

                },
                Twist::Acw => {

                },
            }
        }
    }
    println!("{:#?}", ep);
    Transform {
        edge_cycles: vec![],
        edge_flips: vec![],
        corner_cycles: vec![],
        corner_twists: vec![],
    }
}

impl Move {
    pub fn get_transform(&self) -> Transform {
        match self {
            Move { layer: Layer::U, order: Order::Normal } => {
                Transform {
                    edge_cycles: vec![vec![3, 2, 1, 0]],
                    edge_flips: vec![],
                    corner_cycles: vec![vec![3, 2, 1, 0]],
                    corner_twists: vec![],
                }
            },
            Move { layer: Layer::U, order: Order::Double } => {
                combine_transforms(vec![
                    Move { layer: Layer::U, order: Order::Normal }.get_transform(),
                    Move { layer: Layer::U, order: Order::Normal }.get_transform(),
                ])
            },
            _ => panic!("unimplemented move"),
        }
    }
}

// pub struct Alg {
//     text: String,
//     transform: Transform,
// }

//

// impl Alg {
//     is_ll_alg
// }

impl Cube {
    pub fn new() -> Self {
        Cube {
            edges: [
                        Edge(Face::U, Face::B),
                Edge(Face::U, Face::R), Edge(Face::U, Face::F),
                        Edge(Face::U, Face::L),
                Edge(Face::B, Face::L), Edge(Face::B, Face::R),
                Edge(Face::F, Face::R), Edge(Face::F, Face::L),
                        Edge(Face::D, Face::B),
                Edge(Face::D, Face::R), Edge(Face::D, Face::F),
                        Edge(Face::D, Face::L),
            ],
            corners: [
                Corner(Face::U, Face::L, Face::B), Corner(Face::U, Face::B, Face::R),
                Corner(Face::U, Face::R, Face::F), Corner(Face::U, Face::F, Face::L),
                Corner(Face::D, Face::B, Face::L), Corner(Face::D, Face::R, Face::B),
                Corner(Face::D, Face::F, Face::R), Corner(Face::D, Face::L, Face::F),
            ],
            centres: [
                Face::U, Face::B, Face::R, Face::F, Face::L, Face::D,
            ],
        }
    }

    fn is_solved(&self) -> bool {
        // TODO: add isomorphisms
        self == &Cube::new()
    }

    pub fn do_transform(&mut self, transform: Transform) {
        for edge_cycles in transform.edge_cycles {
            for (i, index_a) in edge_cycles.iter().enumerate() {
                if i != 0 {
                    let index_b = edge_cycles[i-1];
                    self.edges.swap(*index_a, index_b);
                }
            }
        }
        for i in transform.edge_flips {
            self.edges[i].flip();
        }
        for corner_cycles in transform.corner_cycles {
            for (i, index_a) in corner_cycles.iter().enumerate() {
                if i != 0 {
                    let index_b = corner_cycles[i-1];
                    self.corners.swap(*index_a, index_b);
                }
            }
        }
        for (i, type_) in transform.corner_twists {
            self.corners[i].twist(type_);
        }
    }

    // is_ll_solved
    // cube.is_2x2x3_solved()

}


impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{:?}", self.0, self.1)
    }
}
impl fmt::Display for Corner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{:?}{:?}", self.0, self.1, self.2)
    }
}
impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: add visualcube (temporary)
        write!(f, "{} ", self.corners[0]);
        write!(f, "{} ", self.edges[0]);
        write!(f, "{}\n", self.corners[1]);
        write!(f, "{}  ", self.edges[3]);
        write!(f, "{:?}  ", self.centres[0]);
        write!(f, "{}  \n", self.edges[1]);
        write!(f, "{} ", self.corners[3]);
        write!(f, "{} ", self.edges[2]);
        write!(f, "{}\n", self.corners[2]);

        write!(f, "{}  ", self.edges[4]);
        write!(f, "{:?}  ", self.centres[1]);
        write!(f, "{}  \n", self.edges[5]);
        write!(f, "{:?}       ", self.centres[4]);
        write!(f, "{:?}\n", self.centres[2]);
        write!(f, "{}  ", self.edges[7]);
        write!(f, "{:?}  ", self.centres[3]);
        write!(f, "{}\n", self.edges[6]);

        write!(f, "{} ", self.corners[4]);
        write!(f, "{} ", self.edges[8]);
        write!(f, "{}\n", self.corners[5]);
        write!(f, "{}  ", self.edges[10]);
        write!(f, "{:?}  ", self.centres[5]);
        write!(f, "{}  \n", self.edges[9]);
        write!(f, "{} ", self.corners[7]);
        write!(f, "{} ", self.edges[11]);
        write!(f, "{}", self.corners[6])
    }
}
