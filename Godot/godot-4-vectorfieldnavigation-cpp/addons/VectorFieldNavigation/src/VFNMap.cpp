// Datei: src/VFNMap.cpp
#include "VFNMap.hpp"

using namespace godot;

VFNMap::VFNMap() {
	UtilityFunctions::print("VFNMap instance created.");
}

VFNMap::~VFNMap() {
	UtilityFunctions::print("VFNMap instance destroyed.");
}

Ref<VFNField> VFNMap::create_field() {
	Ref<VFNField> field;
	field.instantiate();
	Ref<VFNMap> map_ref = Ref(this);
	field->map = map_ref;
	return field;
}

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
            node.effort = 0.0f;
            node.disabled = false;

            nodes.push_back(node);
        }
    }
}

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
		node.effort = 1.0f;
	}

    UtilityFunctions::print("Heightmap data set: ", nodes.size(), " nodes");
}

void VFNMap::_bind_methods() {
	// Hier kannst du später Funktionen für GDScript verfügbar machen
	ClassDB::bind_method(D_METHOD("create_field"), &VFNMap::create_field);
}
