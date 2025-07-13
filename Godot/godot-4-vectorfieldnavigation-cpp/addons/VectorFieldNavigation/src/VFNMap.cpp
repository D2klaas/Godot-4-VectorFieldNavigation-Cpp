// Datei: src/VFNMap.cpp
#include "VFNMap.hpp"

using namespace godot;

VFNMap::VFNMap() {
	UtilityFunctions::print("VFNMap instance created.");
}

VFNMap::~VFNMap() {
	UtilityFunctions::print("VFNMap instance destroyed.");
}

//creates fiedl and add reference to this map in field
Ref<VFNField> VFNMap::create_field() {
	Ref<VFNField> field;
	field.instantiate();
	Ref<VFNMap> map_ref = Ref(this);
	field->map = map_ref;
	return field;
}

//buildes nodes for map with width and height
void VFNMap::build_nodes(){
	int width = this->map_size.x;
	int height = this->map_size.y;

	int expected_size = width * height;
	nodes.clear();
	nodes.reserve(expected_size);

	for (int y = 0; y < height; y++) {
		for (int x = 0; x < width; x++) {
			int i = y * width + x;

			VFNNode node;
			node.index = i;
			node.world_position = Vector3(x, 0.0f, y);  // y = Höhe
			node.disabled = false;
			nodes.push_back(node);

			//--------- make connections

			for (int dy = -1; dy <= 1; dy++) {
				for (int dx = -1; dx <= 1; dx++) {
					if (dx == 0 && dy == 0) continue; // sich selbst überspringen

					int nx = x + dx;
					int ny = y + dy;

					// Bereichsprüfung
					if (nx < 0 || ny < 0 || nx >= map_size.x || ny >= map_size.y) continue;

					int neighbor_index = ny * map_size.x + nx;

					VFNConnection connection;
					connection.target_node_index = neighbor_index;
					node.connections.push_back(connection);
				}
			}
		}
	}
}

//fills nodes of map with height values
void VFNMap::set_heightmap_data(const PackedFloat32Array& values ) {
	int width = this->map_size.x;
	int height = this->map_size.y;

	int expected_size = width * height;
	if (values.size() != expected_size) {
		UtilityFunctions::printerr("Heightmap size mismatch. Expected ", expected_size, ", got ", values.size());
		return;
	}

	nodes.clear();
	nodes.reserve(expected_size);

	for (VFNNode &node : nodes) {
		node.height = values[node.index];
	}

	for (VFNNode &node : nodes) {
		for (VFNConnection &connection : node.connections ) {
			const Vector3& a = node.world_position;
			const Vector3& b = nodes[connection.target_node_index].world_position;
			connection.effort = a.distance_to(b);
		}
	}

	UtilityFunctions::print("Heightmap data set: ", nodes.size(), " nodes");
}



void VFNMap::_bind_methods() {
	// Hier kannst du später Funktionen für GDScript verfügbar machen
	ClassDB::bind_method(D_METHOD("create_field"), &VFNMap::create_field);
	ClassDB::bind_method(D_METHOD("build_nodes"), &VFNMap::build_nodes);
	ClassDB::bind_method(D_METHOD("set_heightmap_data"), &VFNMap::set_heightmap_data);
}
