use godot::prelude::*;
use crate::vfn_node::VFNNode;

/// Verbindung zwischen zwei Knoten im Vektor-Feld.
#[derive(Clone, Debug)]
pub struct VFNConnection {
	pub target_index: *const VFNNode, // Index des Zielknotens
	pub effort: f32,         // Basisaufwand für die Verbindung
	pub steepness: f32,      // Steigung dieser Verbindung
}