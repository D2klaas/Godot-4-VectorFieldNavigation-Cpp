
// Datei: src/VFNPenaltyField.cpp
#include "VFNPenaltyField.hpp"

using namespace godot;

VFNField::VFNPenaltyField() {}
VFNField::~VFNPenaltyField() {}

void VFNPenaltyField::init( ){
	penalty_map.resize(map->nodes.size(), 0);
};

void VFNPenaltyField::set_effort( int index, float height ){
	if( index >= map->nodes.size() ){
		UtilityFunctions::print("out of bounds");
		return;
	}
	penalty_map[index] = height;
};

float VFNPenaltyField::get_effort( int index ){
	if( index >= map->nodes.size() ){
		UtilityFunctions::print("out of bounds");
		return 0.0f;
	}
	return penalty_map[index];
};

void VFNPenaltyField::_bind_methods() {
	ClassDB::bind_method(D_METHOD("set_effort"), &VFNPenaltyField::set_effort);
	ClassDB::bind_method(D_METHOD("get_effort"), &VFNPenaltyField::get_effort);
}