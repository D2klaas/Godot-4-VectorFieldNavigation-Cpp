// vfn_map.rs
use godot::prelude::*;
use crate::vfn_field::VFNField;
use crate::vfn_node::VFNNode;

#[derive(GodotClass, Debug)]
#[class(init, base = RefCounted)]
pub struct VFNMap {
    // interne Felder folgen bei Bedarf
}

#[godot_api]
impl VFNMap {
    #[func]
    pub fn create_field(&self) -> Gd<VFNField> {
		let map_ptr = self as *const VFNMap;
        Gd::from_init_fn(|base| {
            // Accept a base of type Base<Node3D> and directly forward it.
            VFNField {
                map: map_ptr,
            }
        })

    }

	#[func]
	fn add_node( v: Vector3 ) -> u32 {
		return 1;
	}

	#[func]
	fn add_connection( a: u32, b: u32 ) -> u32 {
		return 1;
	}

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