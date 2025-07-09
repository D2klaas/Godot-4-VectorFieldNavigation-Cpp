// vfn_field.rs
use godot::prelude::*;
use std::collections::{HashMap, VecDeque};

#[derive(GodotClass, Debug)]
#[class(init, base = RefCounted)]
pub struct VFNPenaltyField {
    pub map: *const VFNMap,
/*
    pub size: Vector2i,
    pub effort_cutoff: f32,
    pub climb_factor: f32,
    pub climb_cutoff: f32,
    pub drop_factor: f32,
    pub drop_cutoff: f32,
    pub effort_field: Vec<f32>,
    pub aim_field: Vec<usize>,
    pub vector_field: Vec<Vector3>,
    pub open_mask: Vec<bool>,
    pub open_list: VecDeque<usize>,
    pub targets: Vec<usize>,
    pub mod_fields: HashMap<String, Vec<f32>>,
    pub mod_weights: HashMap<String, f32>,
     */
}

#[godot_api]
impl VFNPenaltyField {
    pub fn set_map(&mut self, map: &VFNMap) {
        self.map = map as *const VFNMap;
    }

    pub fn set_map_ptr(&mut self, ptr: *const VFNMap) {
        self.map = ptr;
    }

    fn init(_base: Base<RefCounted>) -> Self {
        Self {
            map: std::ptr::null(),
/*
            size: Vector2i::ZERO,
            effort_cutoff: 1e9,
            climb_factor: 0.0,
            climb_cutoff: 1e9,
            drop_factor: 0.0,
            drop_cutoff: 1e9,
            effort_field: Vec::new(),
            aim_field: Vec::new(),
            vector_field: Vec::new(),
            open_mask: Vec::new(),
            open_list: VecDeque::new(),
            targets: Vec::new(),
            mod_fields: HashMap::new(),
            mod_weights: HashMap::new(),
  */
        }
    }
}
