// vfn_field.rs
use godot::prelude::*;
use std::collections::{HashMap, VecDeque};

#[derive(GodotClass, Debug)]
#[class(init, base = RefCounted)]
pub struct VFNPenaltyField {
    pub map: *const VFNMap,
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
        }
    }
}
