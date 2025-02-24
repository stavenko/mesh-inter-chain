use core::fmt;

use itertools::Itertools;

use math::{Scalar, Vector3};

use super::{
    aabb::Aabb,
    octree::{Node, Octree},
    sphere::Sphere,
};

#[derive(Debug)]
pub struct VertexIndex<S> {
    octree: Octree<usize, S>,
    points: Vec<Vector3<S>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct PtId(usize);

impl PartialEq<usize> for PtId {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl fmt::Display for PtId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<S: Scalar> VertexIndex<S> {
    pub fn new(aabb: Aabb<S>) -> Self {
        Self {
            octree: Octree::<usize, S>::new_with_aabb(Vec::new(), aabb),
            points: Vec::new(),
        }
    }

    pub fn get_or_insert_point(&mut self, vertex: Vector3<S>, separation_distance: S) -> PtId {
        if let Some(n) = self.find_closest(vertex, separation_distance) {
            n
        } else {
            self.points.push(vertex);
            let id = self.points.len() - 1;
            let node = Node {
                data: id,
                point: vertex,
            };
            self.octree.insert(node);
            PtId(id)
        }
    }

    pub fn get_point(&self, ix: PtId) -> Vector3<S> {
        self.points[ix.0]
    }

    pub fn print_all(&self) {
        for (i, vertex) in self.points.iter().enumerate() {
            println!("v:{i} {} {} {} ", vertex.x, vertex.y, vertex.z);
        }
    }

    pub fn get_vertex_array(&self) -> Vec<[f64; 3]> {
        self.points
            .clone()
            .into_iter()
            .map(|vec| {
                [
                    vec.x.to_f64().expect("Convertion error"),
                    vec.y.to_f64().expect("Convertion error"),
                    vec.z.to_f64().expect("Convertion error"),
                ]
            })
            .collect_vec()
    }

    pub fn find_closest(&self, center: Vector3<S>, distance: S) -> Option<PtId> {
        let mut points = self.octree.query_within_sphere(Sphere {
            center,
            radius: distance,
        });
        points.sort_by_key(|node| {
            (node.point - center)
                .magnitude_squared()
                .mul(S::from_value(1e8))
                .to_isize()
        });
        points.first().map(|node| PtId(node.data))
    }
}

impl From<usize> for PtId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<PtId> for usize {
    fn from(value: PtId) -> Self {
        value.0
    }
}
