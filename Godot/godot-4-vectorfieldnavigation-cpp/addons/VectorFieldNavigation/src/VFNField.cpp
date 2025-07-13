// Datei: src/VFNField.cpp
#include "VFNField.hpp"

using namespace godot;

VFNField::VFNField() {}
VFNField::~VFNField() {}

void VFNField::add_target(int index) {
    // Füge nur hinzu, wenn der Index noch nicht existiert
    if (std::find(targets.begin(), targets.end(), index) == targets.end()) {
        targets.push_back(index);
    }
}

void VFNField::clear_targets(){
	targets.clear();
}

void VFNField::remove_target(int index) {
    // Entfernt ALLE Einträge mit dem Wert `index` aus dem targets-Vektor
    targets.erase(
        std::remove(targets.begin(), targets.end(), index),
        targets.end()
    );
}

void VFNField::_bind_methods() {
	ClassDB::bind_method(D_METHOD("add_target"), &VFNField::add_target);
	ClassDB::bind_method(D_METHOD("clear_targets"), &VFNField::clear_targets);
	ClassDB::bind_method(D_METHOD("remove_target"), &VFNField::remove_target);
}