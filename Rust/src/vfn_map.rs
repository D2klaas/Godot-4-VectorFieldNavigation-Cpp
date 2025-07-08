use godot::prelude::*;
use std::collections::{HashMap, VecDeque};
use crate::vfn_field::VFNField;
use crate::vfn_node::VFNNode;

#[derive(GodotClass)]
#[class(init, base = RefCounted)]
pub struct VFNMap {
	// ... alle Felder ...
}

#[godot_api]
impl VFNMap {
	#[func]
	fn create_field(&self) -> VFNField {
		VFNField::new(&self)
	}

	#[func]
	fn add_node( v: Vector3 ) -> u32 {
		return 1;
	}

	#[func]
	fn add_connection( a: u32, b: u32 ) -> u32 {
		return 1;
	}

	#[func]
	fn get_node_at( v: Vector2i ) -> VFNNode {
		VFNNode {
			index: 0,
			position: Vector2i::new(5, 7),
			world_position: Vector3::new(5.0, 0.0, 7.0),
			steepness: 0.0,
			disabled: false,
			connections: vec![],
		}
	}

	#[func]
	fn set_height( pos:Vector2i, height:f32 ) -> f32 {
		return 1.0;
	}

	#[func]
	fn get_height( pos:Vector2i ) -> f32{
		return 1.0;
	}

	#[func]
	fn disable_node( pos:Vector2i ){

	}

	#[func]
	fn enable_node( pos:Vector2i ){

	}

	#[func]
	fn get_node_from_index( index:u32 ) -> VFNNode{
		VFNNode {
			index: 0,
			position: Vector2i::new(5, 7),
			world_position: Vector3::new(5.0, 0.0, 7.0),
			steepness: 0.0,
			disabled: false,
			connections: vec![],
		}
	}


}

impl Default for VFNMap {
	fn default() -> Self {
		Self {
		}
	}
}