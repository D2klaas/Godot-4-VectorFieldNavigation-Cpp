use godot::prelude::*;
use std::collections::{HashMap, VecDeque};
use crate::vfn_field::VFNField;

#[derive(GodotClass)]
#[class(init, base = RefCounted)]
pub struct VFNMap {
	// ... alle Felder ...
}

#[godot_api]
impl VFNMap {
	 fn create_field(&self) -> VFNField {
		VFNField::new(&self)
	}
}

impl Default for VFNMap {
	fn default() -> Self {
		Self {
		}
	}
}