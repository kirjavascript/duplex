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

impl Corner {
    pub fn twist(&mut self, anti: bool) {
        mem::swap(&mut self.0, &mut self.1);
        if anti {
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
    centres: [Face; 6],
}

pub struct Transform {
    edge_swaps: Vec<(usize, bool)>,
}
// move.get_transform()
// cube.do_transform()
// cube.is_2x2x3_solved()
// cube.get_ll_transform(&cube)

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
