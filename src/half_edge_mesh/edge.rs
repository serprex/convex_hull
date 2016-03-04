use std;

use half_edge_mesh::ptr::{Ptr, EdgePtr, VertPtr, FacePtr, EdgeRc, VertRc, FaceRc};
use half_edge_mesh::iterators::*;

// Please, don't make more than 2^32-1 edges, vertices, or faces
// TODO: better ids (mesh-specific?)
// Maybe use this: https://crates.io/crates/snowflake
static mut edge_id: u32 = 0;

fn get_edge_id() -> u32 { unsafe { edge_id += 1; edge_id } }

#[derive(Debug)]
pub struct Edge {
  pub next: EdgePtr,
  pub pair: EdgePtr,
  pub origin: VertPtr,
  pub face: FacePtr,
  pub id: u32,
}

// TODO: change the name of set_*_rc to just set_*, and change the current set_* to set_*_ptr
// because set_*_rc is used way more than set_* at the moment.
impl Edge {
  pub fn empty() -> Edge {
    Edge {
      id: get_edge_id(),
      next: EdgePtr::empty(),
      pair: EdgePtr::empty(),
      origin: VertPtr::empty(),
      face: FacePtr::empty(),
    }
  }

  pub fn with_origin(origin: VertPtr) -> Edge {
    Edge {
      id: get_edge_id(),
      next: EdgePtr::empty(),
      pair: EdgePtr::empty(),
      origin: origin,
      face: FacePtr::empty(),
    }
  }

  pub fn take_next(&mut self, next: EdgePtr) { self.next = next; }

  pub fn set_next(&mut self, next: & EdgePtr) { self.next = next.clone(); }

  pub fn set_next_rc(&mut self, next: & EdgeRc) { self.next = Ptr::new(next); }

  pub fn take_pair(&mut self, pair: EdgePtr) { self.pair = pair; }

  pub fn set_pair(&mut self, pair: & EdgePtr) { self.pair = pair.clone(); }

  pub fn set_pair_rc(&mut self, pair: & EdgeRc) { self.pair = Ptr::new(pair); }

  pub fn take_origin(&mut self, origin: VertPtr) { self.origin = origin; }

  pub fn set_origin(&mut self, origin: & VertPtr) { self.origin = origin.clone(); }

  pub fn set_origin_rc(&mut self, origin: & VertRc) { self.origin = Ptr::new(origin); }

  pub fn set_face(&mut self, face: & FacePtr) { self.face = face.clone(); }

  pub fn take_face(&mut self, face: FacePtr) { self.face = face; }

  pub fn set_face_rc(&mut self, face: & FaceRc) { self.face = Ptr::new(face); }

  // The tests in this function are in order of "subjective likeliness of being invalid"
  pub fn is_valid(& self) -> bool { self.pair.is_valid() && self.face.is_valid() && self.origin.is_valid() && self.next.is_valid() }

  /// Yields edge.origin, then edge.next.origin
  /// Gives you first the source of the half-edge, and then its target
  pub fn adjacent_verts<'a> (&'a self) -> EdgeAdjacentVertIterator<'a> {
    EdgeAdjacentVertIterator::new(self)
  }

  /// Gives you the edges connected to the source of the half-edge first (in *clockwise* order)
  /// and then the edges connected to the target of the half-edge (also *clockwise* order)
  pub fn adjacent_edges(& self) -> EdgeAdjacentEdgeIterator {
    EdgeAdjacentEdgeIterator::new(self)
  }

  /// Yields edge.face, then edge.pair.face
  /// Gives you the "left" face to the half edge, and then the "right" face
  /// Note that the "right" face is not connected to this edge, but to its pair
  pub fn adjacent_faces<'a>(&'a self) -> EdgeAdjacentFaceIterator<'a> {
    EdgeAdjacentFaceIterator::new(self)
  }
}

impl PartialEq<Edge> for Edge {
  fn eq(& self, other: & Edge) -> bool { self.id == other.id }
}

impl Eq for Edge {}

impl std::hash::Hash for Edge {
  fn hash<H>(& self, state: &mut H) where H: std::hash::Hasher {
    state.write_u32(self.id);
    state.finish();
  }
}
