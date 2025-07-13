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

void VFNField::calculate(){
	std::vector<int> open_list;

    // Effort-Map initialisieren: jeder Index beginnt mit sehr hohem Wert
    //std::vector<float> effort_map;
    effort_map.resize(map->nodes.size(), std::numeric_limits<float>::max());

    //std::vector<float> target_map;
    target_map.resize(map->nodes.size(), -1);

	// Alle Targets in open_list kopieren
	for (int target : targets) {
		open_list.push_back(target);
		effort_map[target] = 0.0;
	}


    // Sicherheitsbegrenzung: max. Anzahl an Schleifendurchläufen
    const int max_iterations = 100000;
    int iteration_count = 0;

	// Solange offene Knoten vorhanden sind
	while (!open_list.empty()) {
		int current_index = open_list.back();
		open_list.pop_back();

		VFNNode &current_node = map->nodes[current_index];

		for (VFNConnection &connection : current_node.connections ) {
			int other_index = connection.target_node_index;
			VFNNode &other_node = map->nodes[other_index];

			float effort = effort_map[current_index] + connection.effort;

			if( effort < effort_map[other_index] ){
				effort_map[other_index] = effort;
				target_map[other_index] = current_index;
			}
		}

		// TODO: Zugriff auf VFNMap und Verarbeitung von Nachbarn etc.
        iteration_count++;
        if (iteration_count >= max_iterations) {
            UtilityFunctions::printerr("Abbruch: Max. Iterationsanzahl erreicht (", max_iterations, ")");
            break;
        }
	}
}


void VFNField::_bind_methods() {
	ClassDB::bind_method(D_METHOD("add_target"), &VFNField::add_target);
	ClassDB::bind_method(D_METHOD("clear_targets"), &VFNField::clear_targets);
	ClassDB::bind_method(D_METHOD("remove_target"), &VFNField::remove_target);
	ClassDB::bind_method(D_METHOD("calculate"), &VFNField::calculate);
}