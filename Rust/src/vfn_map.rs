// vfn_map.rs
use godot::prelude::*;
use crate::vfn_field::VFNField;
use crate::vfn_node::VFNNode;
use crate::vfn_connection::VFNConnection;

#[derive(GodotClass, Debug)]
#[class(init, base = RefCounted)]
pub struct VFNMap {
	// interne Felder folgen bei Bedarf
	#[export]
	#[init(val = GString::from("This is also a VFNMap"))]
	pub label: GString,
	pub size: Vector2i,
	pub height_data: Vec<f32>,
	pub nodes: Vec<VFNNode>,
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
				label: GString::from("this is a VFNField"),
			}
		})

	}

	#[func]
	pub fn set_heightmap_data(&mut self, size: Vector2i, values: PackedFloat32Array) {
		self.size = size;
		self.height_data = values.to_vec(); // oder `.as_slice().to_vec()` bei Bedarf

		godot_print!(
			"Heightmap übernommen: {} Werte für {}x{}",
			self.height_data.len(),
			size.x,
			size.y
		);

	}

	#[func]
	pub fn build_nodes_from_heightmap(&mut self) {
		let width = self.size.x as usize;
		let height = self.size.y as usize;
		self.nodes.clear();

		for y in 0..height {
			for x in 0..width {
				let index = y * width + x;
				let h = self.height_data[index];

				let node = VFNNode {
					index,
					position: Vector2i::new(x as i32, y as i32),
					world_position: Vector3::new(x as f32, h, y as f32),
					height: h,
					steepness: 0.0,
					disabled: false,
					connections: Vec::new(), // später setzen
				};

				self.nodes.push(node);
			}
		}

		// Jetzt Pointers generieren
		let node_ptrs: Vec<*const VFNNode> = self.nodes.iter().map(|n| n as *const _).collect();

		// Jetzt z. B. 4er-Nachbarn setzen
		for y in 0..height {
			for x in 0..width {
				let index = y * width + x;
				let mut conns = Vec::new();

				for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
					let nx = x as isize + dx;
					let ny = y as isize + dy;

					if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
						let n_index = ny as usize * width + nx as usize;
						let other = node_ptrs[n_index];
						let height_diff = unsafe { (*other).height - self.nodes[index].height };
						let steepness = height_diff; // oder ggf. normieren
						let cost = 1.0 + steepness.abs(); // einfaches Beispiel

						let n_index = ny as usize * width + nx as usize;
						conns.push(VFNConnection {
							target_index: other,
							steepness: steepness,
							effort: cost,
						});

					}
				}

				self.nodes[index].connections = conns;
			}
		}

		godot_print!("{} Nodes erzeugt mit Nachbarn", self.nodes.len());
	}


	#[func]
	fn add_node( v: Vector3 ) -> u32 {
		return 1;
	}

	#[func]
	fn add_connection( a: u32, b: u32 ) -> u32 {
		return 1;
	}

	fn get_node_at( v: Vector2i ) -> Option<VFNNode> {
		return None;
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

	fn get_node_from_index( index:u32 ) ->  Option<VFNNode> {
		return None;
	}


}
/*
impl Default for VFNMap {
	fn default() -> Self {
		Self {
			label: GString::from("This is a VFNMap"),
		}
	}
} 
*/