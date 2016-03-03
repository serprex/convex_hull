use std;

use cgmath::Point; // Needed for Pt::origin()
use cgmath::EuclideanVector;

use defs::*;

use half_edge_mesh::ptr::{EdgePtr, VertRc};
use half_edge_mesh::iterators::*;

static mut face_id: u32 = 0;

fn get_face_id() -> u32 { unsafe { face_id += 1; face_id } }

pub struct Face {
  pub edge: EdgePtr,
  pub normal: Vec3,
  pub center: Pt,
  pub id: u32,
}

impl Face {
  pub fn empty() -> Face {
    Face {
      id: get_face_id(),
      edge: EdgePtr::empty(),
      // Are these sensible defaults?
      // Are these values even necessary?
      normal: Vec3::unit_z(),
      center: Pt::origin(),
    }
  }

  // Face connected to an existing edge
  pub fn with_edge(edge: EdgePtr) -> Face {
    Face {
      id: get_face_id(),
      edge: edge,
      normal: Vec3::unit_z(),
      center: Pt::origin(),
    }
  }

  pub fn take_edge(&mut self, edge: EdgePtr) { self.edge = edge; }

  pub fn set_edge(&mut self, edge: & EdgePtr) { self.edge = edge.clone(); }

  pub fn is_valid(& self) -> bool { self.edge.is_valid() }

  pub fn num_vertices(& self) -> usize { self.adjacent_verts().count() }

  // Note: this only works when edges and verts are properly connected
  // So wait for the right time during initialization to run this
  pub fn compute_attrs(&mut self) {
    let mut center = Pt::origin();
    let mut count: f32 = 0.0;

    let vert_list: Vec<VertRc> = self.adjacent_verts().to_ptr_vec();

    debug_assert!(vert_list.len() == 3, "should have 3 adjacent vertices");

    for vert in vert_list.iter() {
      let pos = vert.borrow().get_pos();
      center.x += pos.x;
      center.y += pos.y;
      center.z += pos.z;
      count += 1.0;
    }

    // Average position of the corner points
    self.center = center / count;

    let vert_a = vert_list[0].borrow().get_pos();
    let s1 = vert_list[1].borrow().get_pos() - vert_a;
    let s2 = vert_list[2].borrow().get_pos() - vert_a;
    self.normal = s1.cross(s2).normalize();
  }

  pub fn adjacent_verts(& self) -> FaceAdjacentVertIterator {
    FaceAdjacentVertIterator::new(self.edge.clone())
  }

  pub fn adjacent_edges(& self) -> FaceAdjacentEdgeIterator {
    FaceAdjacentEdgeIterator::new(self.edge.clone())
  }

  pub fn adjacent_faces(& self) -> FaceAdjacentFaceIterator {
    FaceAdjacentFaceIterator::new(self.edge.clone())
  }
}

impl PartialEq<Face> for Face {
  fn eq(& self, other: & Face) -> bool { self.id == other.id }
}

impl Eq for Face {}

impl std::hash::Hash for Face {
  fn hash<H>(& self, state: &mut H) where H: std::hash::Hasher {
    state.write_u32(self.id);
    state.finish();
  }
}

impl std::fmt::Debug for Face {
  fn fmt(& self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(fmt, "Face {{ id: {} }}", self.id)
  }
}