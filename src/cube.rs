use std::{fmt, mem};
use lazy_static::lazy_static;
use serde_derive::Serialize;

// moves

#[derive(Debug, Clone)]
pub enum Order {
    Normal,
    Prime,
    Double,
}

impl Order {
    pub fn flip(&self) -> Self {
        match self {
            Order::Normal => Order::Prime,
            Order::Prime => Order::Normal,
            Order::Double => Order::Double,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Layer {
    U, F, R, L, B, D, M, E, S,
    Uw, Fw, Rw, Lw, Bw, Dw, X, Y, Z,
}

pub struct Move {
    pub order: Order,
    pub layer: Layer,
}

// cube

#[derive(Debug, PartialEq, Serialize)]
pub enum Face {
    U,R,F,B,L,D,
}


#[derive(PartialEq, Serialize)]
pub struct Edge(Face, Face);

impl Edge {
    pub fn flip(&mut self) {
        mem::swap(&mut self.0, &mut self.1);
    }
}

#[derive(PartialEq, Serialize)]
pub struct Corner(Face, Face, Face);

#[derive(PartialEq, Clone, Copy, Debug)]
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


// transforms

#[derive(Clone, Debug)]
pub struct Transform {
    pub edge_cycles: Vec<Vec<usize>>,
    pub edge_flips: Vec<usize>,
    pub corner_cycles: Vec<Vec<usize>>,
    pub corner_twists: Vec<(usize, Twist)>,
    pub centre_cycles: Vec<Vec<usize>>,
}

impl Transform {
    pub fn is_ll_transform(&self) -> bool {
        let mut cube = Cube::new();
        cube.do_transform(self);
        cube.is_f2l_solved()
    }
}

pub fn moves_to_transform(moves: &Vec<Move>) -> Transform {
    combine_transforms(moves.iter().map(|s|s.get_transform()).collect())
}

pub fn combine_transforms(transforms: Vec<Transform>) -> Transform {
    let mut ep = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let mut eo = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut cp = [0, 1, 2, 3, 4, 5, 6, 7];
    let mut co = [0, 0, 0, 0, 0, 0, 0, 0];
    let mut c = [0, 1, 2, 3, 4, 5];

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
        for centre_cycles in transform.centre_cycles {
            for (i, index_a) in centre_cycles.iter().enumerate() {
                if i != 0 {
                    let index_b = centre_cycles[i-1];
                    c.swap(*index_a, index_b);
                }
            }
        }
    }
    let mut edge_cycles = vec![];
    let mut edge_flips = vec![];
    let mut corner_cycles = vec![];
    let mut corner_twists = vec![];
    let mut centre_cycles = vec![];
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
        visited.clear();
        for (i, pos) in c.iter().enumerate() {
            if !visited.contains(&i) {
                if pos != &i {
                    // get cycles...
                    let mut cycles = vec![];
                    let first = i;
                    let mut current = i;
                    while c[current] != first {
                        cycles.push(current);
                        visited.push(current);
                        current = c[current];
                    }
                    cycles.push(current);
                    visited.push(current);
                    centre_cycles.push(cycles);
                } else {
                    visited.push(i);
                }
            }
        }
    }
    Transform {
        edge_cycles,
        edge_flips,
        corner_cycles,
        corner_twists,
        centre_cycles,
    }
}

macro_rules! prime_double {
    ($name:ident, $pri:ident, $dbl:ident) => {
        lazy_static! {
            static ref $pri: Transform = combine_transforms(vec![
                Move { layer: Layer::$name, order: Order::Normal }.get_transform(),
                Move { layer: Layer::$name, order: Order::Normal }.get_transform(),
                Move { layer: Layer::$name, order: Order::Normal }.get_transform(),
            ]);
            static ref $dbl: Transform = combine_transforms(vec![
                Move { layer: Layer::$name, order: Order::Normal }.get_transform(),
                Move { layer: Layer::$name, order: Order::Normal }.get_transform(),
            ]);
        }
    };
}

prime_double!(U, UPRI, UDBL);
prime_double!(D, DPRI, DDBL);
prime_double!(R, RPRI, RDBL);
prime_double!(F, FPRI, FDBL);
prime_double!(L, LPRI, LDBL);
prime_double!(B, BPRI, BDBL);
prime_double!(M, MPRI, MDBL);
prime_double!(E, EPRI, EDBL);
prime_double!(S, SPRI, SDBL);

lazy_static! {
    static ref XNORM: Transform = combine_transforms(vec![
        Move { layer: Layer::R, order: Order::Normal }.get_transform(),
        Move { layer: Layer::M, order: Order::Prime }.get_transform(),
        Move { layer: Layer::L, order: Order::Prime }.get_transform(),
    ]);
    static ref YNORM: Transform = combine_transforms(vec![
        Move { layer: Layer::U, order: Order::Normal }.get_transform(),
        Move { layer: Layer::E, order: Order::Prime }.get_transform(),
        Move { layer: Layer::D, order: Order::Prime }.get_transform(),
    ]);
    static ref ZNORM: Transform = combine_transforms(vec![
        Move { layer: Layer::S, order: Order::Normal }.get_transform(),
        Move { layer: Layer::F, order: Order::Normal }.get_transform(),
        Move { layer: Layer::B, order: Order::Prime }.get_transform(),
    ]);
    static ref RWNORM: Transform = combine_transforms(vec![
        Move { layer: Layer::R, order: Order::Normal }.get_transform(),
        Move { layer: Layer::M, order: Order::Prime }.get_transform(),
    ]);
    static ref LWNORM: Transform = combine_transforms(vec![
        Move { layer: Layer::L, order: Order::Normal }.get_transform(),
        Move { layer: Layer::M, order: Order::Normal }.get_transform(),
    ]);
    static ref UWNORM: Transform = combine_transforms(vec![
        Move { layer: Layer::U, order: Order::Normal }.get_transform(),
        Move { layer: Layer::E, order: Order::Prime }.get_transform(),
    ]);
    static ref DWNORM: Transform = combine_transforms(vec![
        Move { layer: Layer::D, order: Order::Normal }.get_transform(),
        Move { layer: Layer::E, order: Order::Normal }.get_transform(),
    ]);
    static ref FWNORM: Transform = combine_transforms(vec![
        Move { layer: Layer::F, order: Order::Normal }.get_transform(),
        Move { layer: Layer::S, order: Order::Normal }.get_transform(),
    ]);
    static ref BWNORM: Transform = combine_transforms(vec![
        Move { layer: Layer::B, order: Order::Normal }.get_transform(),
        Move { layer: Layer::S, order: Order::Prime }.get_transform(),
    ]);
}

prime_double!(X, XPRI, XDBL);
prime_double!(Y, YPRI, YDBL);
prime_double!(Z, ZPRI, ZDBL);
prime_double!(Uw, UWPRI, UWDBL);
prime_double!(Dw, DWPRI, DWDBL);
prime_double!(Lw, LWPRI, LWDBL);
prime_double!(Rw, RWPRI, RWDBL);
prime_double!(Fw, FWPRI, FWDBL);
prime_double!(Bw, BWPRI, BWDBL);

impl Move {
    pub fn get_transform(&self) -> Transform {
        match self {
            Move { layer: Layer::U, order: Order::Normal } => {
                Transform {
                    edge_cycles: vec![vec![3, 2, 1, 0]],
                    edge_flips: vec![],
                    corner_cycles: vec![vec![3, 2, 1, 0]],
                    corner_twists: vec![],
                    centre_cycles: vec![],
                }
            },
            Move { layer: Layer::U, order: Order::Double } => UDBL.clone(),
            Move { layer: Layer::U, order: Order::Prime } => UPRI.clone(),
            Move { layer: Layer::D, order: Order::Normal } => {
                Transform {
                    edge_cycles: vec![vec![8, 9, 10, 11]],
                    edge_flips: vec![],
                    corner_cycles: vec![vec![4, 5, 6, 7]],
                    corner_twists: vec![],
                    centre_cycles: vec![],
                }
            },
            Move { layer: Layer::D, order: Order::Double } => DDBL.clone(),
            Move { layer: Layer::D, order: Order::Prime } => DPRI.clone(),
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
                    centre_cycles: vec![],
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
                    centre_cycles: vec![],
                }
            },
            Move { layer: Layer::F, order: Order::Double } => FDBL.clone(),
            Move { layer: Layer::F, order: Order::Prime } => FPRI.clone(),
            Move { layer: Layer::L, order: Order::Normal } => {
                Transform {
                    edge_cycles: vec![vec![4, 11, 7, 3]],
                    edge_flips: vec![],
                    corner_cycles: vec![vec![4, 7, 3, 0]],
                    corner_twists: vec![
                        (7, Twist::Acw),
                        (4, Twist::Cw),
                        (0, Twist::Acw),
                        (3, Twist::Cw),
                    ],
                    centre_cycles: vec![],
                }
            },
            Move { layer: Layer::L, order: Order::Double } => LDBL.clone(),
            Move { layer: Layer::L, order: Order::Prime } => LPRI.clone(),
            Move { layer: Layer::B, order: Order::Normal } => {
                Transform {
                    edge_cycles: vec![vec![5, 8, 4, 0]],
                    edge_flips: vec![5, 8, 4, 0],
                    corner_cycles: vec![vec![5, 4, 0, 1]],
                    corner_twists: vec![
                        (4, Twist::Acw),
                        (5, Twist::Cw),
                        (1, Twist::Acw),
                        (0, Twist::Cw),
                    ],
                    centre_cycles: vec![],
                }
            },
            Move { layer: Layer::B, order: Order::Double } => BDBL.clone(),
            Move { layer: Layer::B, order: Order::Prime } => BPRI.clone(),
            Move { layer: Layer::M, order: Order::Normal } => {
                Transform {
                    edge_cycles: vec![vec![0, 8, 10, 2]],
                    edge_flips: vec![0, 8, 10, 2],
                    corner_cycles: vec![],
                    corner_twists: vec![],
                    centre_cycles: vec![vec![2, 1, 4, 0]],
                }
            },
            Move { layer: Layer::M, order: Order::Double } => MDBL.clone(),
            Move { layer: Layer::M, order: Order::Prime } => MPRI.clone(),
            Move { layer: Layer::E, order: Order::Normal } => {
                Transform {
                    edge_cycles: vec![vec![4, 5, 6, 7]],
                    edge_flips: vec![4, 5, 6, 7],
                    corner_cycles: vec![],
                    corner_twists: vec![],
                    centre_cycles: vec![vec![2, 3, 4, 5]],
                }
            },
            Move { layer: Layer::E, order: Order::Double } => EDBL.clone(),
            Move { layer: Layer::E, order: Order::Prime } => EPRI.clone(),
            Move { layer: Layer::S, order: Order::Normal } => {
                Transform {
                    edge_cycles: vec![vec![3, 11, 9, 1]],
                    edge_flips: vec![3, 11, 9, 1],
                    corner_cycles: vec![],
                    corner_twists: vec![],
                    centre_cycles: vec![vec![5, 1, 3, 0]],
                }
            },
            Move { layer: Layer::S, order: Order::Double } => SDBL.clone(),
            Move { layer: Layer::S, order: Order::Prime } => SPRI.clone(),
            Move { layer: Layer::X, order: Order::Normal } => XNORM.clone(),
            Move { layer: Layer::X, order: Order::Double } => XDBL.clone(),
            Move { layer: Layer::X, order: Order::Prime } => XPRI.clone(),
            Move { layer: Layer::Y, order: Order::Normal } => YNORM.clone(),
            Move { layer: Layer::Y, order: Order::Double } => YDBL.clone(),
            Move { layer: Layer::Y, order: Order::Prime } => YPRI.clone(),
            Move { layer: Layer::Z, order: Order::Normal } => ZNORM.clone(),
            Move { layer: Layer::Z, order: Order::Double } => ZDBL.clone(),
            Move { layer: Layer::Z, order: Order::Prime } => ZPRI.clone(),
            Move { layer: Layer::Uw, order: Order::Normal } => UWNORM.clone(),
            Move { layer: Layer::Uw, order: Order::Double } => UWDBL.clone(),
            Move { layer: Layer::Uw, order: Order::Prime } => UWPRI.clone(),
            Move { layer: Layer::Dw, order: Order::Normal } => DWNORM.clone(),
            Move { layer: Layer::Dw, order: Order::Double } => DWDBL.clone(),
            Move { layer: Layer::Dw, order: Order::Prime } => DWPRI.clone(),
            Move { layer: Layer::Rw, order: Order::Normal } => RWNORM.clone(),
            Move { layer: Layer::Rw, order: Order::Double } => RWDBL.clone(),
            Move { layer: Layer::Rw, order: Order::Prime } => RWPRI.clone(),
            Move { layer: Layer::Lw, order: Order::Normal } => LWNORM.clone(),
            Move { layer: Layer::Lw, order: Order::Double } => LWDBL.clone(),
            Move { layer: Layer::Lw, order: Order::Prime } => LWPRI.clone(),
            Move { layer: Layer::Fw, order: Order::Normal } => FWNORM.clone(),
            Move { layer: Layer::Fw, order: Order::Double } => FWDBL.clone(),
            Move { layer: Layer::Fw, order: Order::Prime } => FWPRI.clone(),
            Move { layer: Layer::Bw, order: Order::Normal } => BWNORM.clone(),
            Move { layer: Layer::Bw, order: Order::Double } => BWDBL.clone(),
            Move { layer: Layer::Bw, order: Order::Prime } => BWPRI.clone(),
        }
    }
}

#[derive(PartialEq)]
pub struct Cube {
    pub edges: [Edge; 12],
    pub corners: [Corner; 8],
    pub centres: [Face; 6],
}

lazy_static! {
    static ref SOLVED0: Cube = Cube::new();
    static ref SOLVED1: Cube = {
        let mut cube = Cube::new();
        let move_ = Move { layer: Layer::Y, order: Order::Normal };
        cube.do_transform(&move_.get_transform());
        cube
    };
    static ref SOLVED2: Cube = {
        let mut cube = Cube::new();
        let move_ = Move { layer: Layer::Y, order: Order::Double };
        cube.do_transform(&move_.get_transform());
        cube
    };
    static ref SOLVED3: Cube = {
        let mut cube = Cube::new();
        let move_ = Move { layer: Layer::Y, order: Order::Prime };
        cube.do_transform(&move_.get_transform());
        cube
    };
}

impl Cube {
    pub const fn new() -> Self {
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
                Face::U, Face::D, Face::B, Face::R, Face::F, Face::L,
            ],
        }
    }

    pub fn do_transform(&mut self, transform: &Transform) {
        for edge_cycles in transform.edge_cycles.iter() {
            for (i, index_a) in edge_cycles.iter().enumerate() {
                if i != 0 {
                    let index_b = edge_cycles[i-1];
                    self.edges.swap(*index_a, index_b);
                }
            }
        }
        for i in transform.edge_flips.iter() {
            self.edges[*i].flip();
        }
        for corner_cycles in transform.corner_cycles.iter() {
            for (i, index_a) in corner_cycles.iter().enumerate() {
                if i != 0 {
                    let index_b = corner_cycles[i-1];
                    self.corners.swap(*index_a, index_b);
                }
            }
        }
        for (i, type_) in transform.corner_twists.iter() {
            self.corners[*i].twist(*type_);
        }
        for centre_cycles in transform.centre_cycles.iter() {
            for (i, index_a) in centre_cycles.iter().enumerate() {
                if i != 0 {
                    let index_b = centre_cycles[i-1];
                    self.centres.swap(*index_a, index_b);
                }
            }
        }
    }

    pub fn is_ll_solved(&self) -> bool {
        // TODO: benchmark different approaches to doing this
        self.centres[0] == Face::U && (
            (self.edges[0..4] == SOLVED0.edges[0..4] &&
             self.corners[0..4] == SOLVED0.corners[0..4]) ||
            (self.edges[0..4] == SOLVED1.edges[0..4] &&
             self.corners[0..4] == SOLVED1.corners[0..4]) ||
            (self.edges[0..4] == SOLVED2.edges[0..4] &&
             self.corners[0..4] == SOLVED2.corners[0..4]) ||
            (self.edges[0..4] == SOLVED3.edges[0..4] &&
             self.corners[0..4] == SOLVED3.corners[0..4])
        )
    }

    pub fn is_f2l_solved(&self) -> bool {
        self.centres[0] == Face::U && (
            (self.edges[4..] == SOLVED0.edges[4..] &&
             self.corners[4..] == SOLVED0.corners[4..]) ||
            (self.edges[4..] == SOLVED1.edges[4..] &&
             self.corners[4..] == SOLVED1.corners[4..]) ||
            (self.edges[4..] == SOLVED2.edges[4..] &&
             self.corners[4..] == SOLVED2.corners[4..]) ||
            (self.edges[4..] == SOLVED3.edges[4..] &&
             self.corners[4..] == SOLVED3.corners[4..])
        )
    }

}

// display / debug

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let order = match self.order {
            Order::Normal => "",
            Order::Prime => "'",
            Order::Double => "2",
        };
        let layer = format!("{:?}", self.layer);
        let is_lower = layer.ends_with('w')
            || layer == "X" || layer == "Y" || layer == "Z";
        let layer = if is_lower {
            format!("{}", &layer[..1].to_lowercase())
        } else {
            layer
        };
        write!(f, "{}{}", layer, order)
    }
}
impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{:?}", self.0, self.1)
    }
}
impl fmt::Debug for Corner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{:?}{:?}", self.0, self.1, self.2)
    }
}
impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} ", self.corners[0]).ok();
        write!(f, "{:?} ", self.edges[0]).ok();
        write!(f, "{:?}\n", self.corners[1]).ok();
        write!(f, "{:?}  ", self.edges[3]).ok();
        write!(f, "{:?}  ", self.centres[0]).ok();
        write!(f, "{:?}  \n", self.edges[1]).ok();
        write!(f, "{:?} ", self.corners[3]).ok();
        write!(f, "{:?} ", self.edges[2]).ok();
        write!(f, "{:?}\n", self.corners[2]).ok();
        write!(f, "--  -- --\n").ok();
        write!(f, "{:?}  ", self.edges[4]).ok();
        write!(f, "{:?}  ", self.centres[2]).ok();
        write!(f, "{:?}  \n", self.edges[5]).ok();
        write!(f, "{:?}       ", self.centres[5]).ok();
        write!(f, "{:?}\n", self.centres[3]).ok();
        write!(f, "{:?}  ", self.edges[7]).ok();
        write!(f, "{:?}  ", self.centres[4]).ok();
        write!(f, "{:?}\n", self.edges[6]).ok();
        write!(f, "--  -- --\n").ok();
        write!(f, "{:?} ", self.corners[4]).ok();
        write!(f, "{:?} ", self.edges[8]).ok();
        write!(f, "{:?}\n", self.corners[5]).ok();
        write!(f, "{:?}  ", self.edges[11]).ok();
        write!(f, "{:?}  ", self.centres[1]).ok();
        write!(f, "{:?}  \n", self.edges[9]).ok();
        write!(f, "{:?} ", self.corners[7]).ok();
        write!(f, "{:?} ", self.edges[10]).ok();
        write!(f, "{:?}", self.corners[6])
    }
}
