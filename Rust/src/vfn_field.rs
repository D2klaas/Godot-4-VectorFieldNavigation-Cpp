use godot::prelude::*;
use std::collections::{HashMap, VecDeque};
use crate::vfn_map::VFNMap;

#[derive(GodotClass)]
#[class(init, base = RefCounted)]
pub struct VFNField {
	pub map: *const VFNMap,
}

#[godot_api]
impl VFNField {
    pub fn new(map: &VFNMap) -> Self {
        Self {
            map,
        }
    }
}

