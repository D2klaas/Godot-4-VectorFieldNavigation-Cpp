use godot::prelude::*;
use crate::vfn_connection::VFNConnection;

/// Struktur eines einzelnen Knotens im Vektor-Feld.
#[derive(Clone, Debug)]
pub struct VFNNode {
	pub index: usize,                // Eindeutiger Index im Feld
	pub position: Vector2i,         // Rasterposition
	pub world_position: Vector3,    // Weltkoordinaten für Vektorrichtungen
	pub steepness: f32,             // Steigung an dieser Stelle
	pub disabled: bool,             // Ob dieser Knoten deaktiviert ist
	pub connections: Vec<VFNConnection>,    // Indizes der Nachbarknoten
}
