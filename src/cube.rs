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

#[derive(PartialEq, Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Transform {
    pub edge_cycles: Vec<Vec<usize>>,
    pub edge_flips: Vec<usize>,
    pub corner_cycles: Vec<Vec<usize>>,
    pub corner_twists: Vec<(usize, Twist)>,
}

pub fn combine_transforms(transforms: Vec<Transform>) -> Transform {
    let mut ep = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let mut eo = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut cp = [0, 1, 2, 3, 4, 5, 6, 7];
    let mut co = [0, 0, 0, 0, 0, 0, 0, 0];

    for transform in transforms {
        for edge_cycles in transform.edge_cycles {
            for (i, index_a) in edge_cycles.iter().enumerate() {
                if i != 0 {
                    let index_b = edge_cycles[i-1];
                    ep.swap(*index_a, index_b);
                    eo.swap(*index_a, index_b);
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
                    cp.swap(*index_a, index_b);
                    co.swap(*index_a, index_b);
                }
            }
        }
        for (i, type_) in transform.corner_twists {
            match type_ {
                Twist::Cw => {
                    co[i] += 1;
                    if co[i] == 3 {
                        co[i] = 0;
                    }
                },
                Twist::Acw => {
                    co[i] -= 1;
                    if co[i] == -1 {
                        co[i] = 2;
                    }
                },
            }
        }
    }
    let mut edge_cycles = vec![];
    let mut edge_flips = vec![];
    let mut corner_cycles = vec![];
    let mut corner_twists = vec![];
    {
        let mut visited = vec![];
        for (i, pos) in ep.iter().enumerate() {
            if !visited.contains(&i) {
                if pos != &i {
                    // get cycles...
                    let mut cycles = vec![];
                    let first = i;
                    let mut current = i;
                    while ep[current] != first {
                        cycles.push(current);
                        visited.push(current);
                        current = ep[current];
                    }
                    cycles.push(current);
                    visited.push(current);
                    edge_cycles.push(cycles);
                } else {
                    visited.push(i);
                }
            }
        }
        visited.clear();
        for (i, pos) in cp.iter().enumerate() {
            if !visited.contains(&i) {
                if pos != &i {
                    // get cycles...
                    let mut cycles = vec![];
                    let first = i;
                    let mut current = i;
                    while cp[current] != first {
                        cycles.push(current);
                        visited.push(current);
                        current = cp[current];
                    }
                    cycles.push(current);
                    visited.push(current);
                    corner_cycles.push(cycles);
                } else {
                    visited.push(i);
                }
            }
        }
        for (i, flip) in eo.iter().enumerate() {
            if flip == &1 {
                edge_flips.push(i);
            }
        }
        for (i, twist) in co.iter().enumerate() {
            if twist != &0 {
                corner_twists.push((
                    i,
                    { if twist == &1 { Twist::Cw } else { Twist::Acw } }
                ));
            }
        }
    }
    Transform {
        edge_cycles,
        edge_flips,
        corner_cycles,
        corner_twists,
    }
}

lazy_static! {
    static ref UPRI: Transform = combine_transforms(vec![
        Move { layer: Layer::U, order: Order::Normal }.get_transform(),
        Move { layer: Layer::U, order: Order::Normal }.get_transform(),
        Move { layer: Layer::U, order: Order::Normal }.get_transform(),
    ]);
    static ref UDBL: Transform = combine_transforms(vec![
        Move { layer: Layer::U, order: Order::Normal }.get_transform(),
        Move { layer: Layer::U, order: Order::Normal }.get_transform(),
    ]);
    static ref RPRI: Transform = combine_transforms(vec![
        Move { layer: Layer::R, order: Order::Normal }.get_transform(),
        Move { layer: Layer::R, order: Order::Normal }.get_transform(),
        Move { layer: Layer::R, order: Order::Normal }.get_transform(),
    ]);
    static ref RDBL: Transform = combine_transforms(vec![
        Move { layer: Layer::R, order: Order::Normal }.get_transform(),
        Move { layer: Layer::R, order: Order::Normal }.get_transform(),
    ]);
    static ref FPRI: Transform = combine_transforms(vec![
        Move { layer: Layer::F, order: Order::Normal }.get_transform(),
        Move { layer: Layer::F, order: Order::Normal }.get_transform(),
        Move { layer: Layer::F, order: Order::Normal }.get_transform(),
    ]);
    static ref FDBL: Transform = combine_transforms(vec![
        Move { layer: Layer::F, order: Order::Normal }.get_transform(),
        Move { layer: Layer::F, order: Order::Normal }.get_transform(),
    ]);
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
            Move { layer: Layer::U, order: Order::Double } => UDBL.clone(),
            Move { layer: Layer::U, order: Order::Prime } => UPRI.clone(),
            Move { layer: Layer::R, order: Order::Normal } => {
                Transform {
                    edge_cycles: vec![vec![6, 9, 5, 1]],
                    edge_flips: vec![],
                    corner_cycles: vec![vec![6, 5, 1, 2]],
                    corner_twists: vec![
                        (2, Twist::Acw),
                        (1, Twist::Cw),
                        (5, Twist::Acw),
                        (6, Twist::Cw),
                    ],
                }
            },
            Move { layer: Layer::R, order: Order::Double } => RDBL.clone(),
            Move { layer: Layer::R, order: Order::Prime } => RPRI.clone(),
            Move { layer: Layer::F, order: Order::Normal } => {
                Transform {
                    edge_cycles: vec![vec![7, 10, 6, 2]],
                    edge_flips: vec![10, 2, 6, 7],
                    corner_cycles: vec![vec![3, 7, 6, 2]],
                    corner_twists: vec![
                        (3, Twist::Acw),
                        (7, Twist::Cw),
                        (6, Twist::Acw),
                        (2, Twist::Cw),
                    ],
                }
            },
            Move { layer: Layer::F, order: Order::Double } => FDBL.clone(),
            Move { layer: Layer::F, order: Order::Prime } => FPRI.clone(),
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
//     is_ll_alg -> just check transforms are < 4
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
        write!(f, "--  -- --\n");
        write!(f, "{}  ", self.edges[4]);
        write!(f, "{:?}  ", self.centres[1]);
        write!(f, "{}  \n", self.edges[5]);
        write!(f, "{:?}       ", self.centres[4]);
        write!(f, "{:?}\n", self.centres[2]);
        write!(f, "{}  ", self.edges[7]);
        write!(f, "{:?}  ", self.centres[3]);
        write!(f, "{}\n", self.edges[6]);
        write!(f, "--  -- --\n");
        write!(f, "{} ", self.corners[4]);
        write!(f, "{} ", self.edges[8]);
        write!(f, "{}\n", self.corners[5]);
        write!(f, "{}  ", self.edges[11]);
        write!(f, "{:?}  ", self.centres[5]);
        write!(f, "{}  \n", self.edges[9]);
        write!(f, "{} ", self.corners[7]);
        write!(f, "{} ", self.edges[10]);
        write!(f, "{}", self.corners[6])
    }
}
