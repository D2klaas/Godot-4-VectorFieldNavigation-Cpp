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

Dictionary VFNField::calculate(){
	std::deque<int>  open_list;

	// Effort-Map initialisieren: jeder Index beginnt mit sehr hohem Wert
	//std::vector<float> effort_map;
	effort_map.resize(map->nodes.size(), std::numeric_limits<float>::max());

	//std::vector<float> target_map;
	target_map.resize(map->nodes.size(), -1);

	//std::vector<float> target_map;
	vector_map.resize(map->nodes.size(), Vector3(0,0,0));

	std::vector<bool> in_open_list;
	in_open_list.resize(map->nodes.size(), false);

	highest_effort  = 0.0;

	// Alle Targets in open_list kopieren
	for (int target : targets) {
		open_list.push_back(target);
		effort_map[target] = 0.0;
		in_open_list[target] = true;
	}
	UtilityFunctions::print("inserted ", targets.size() , " targets");


	// Sicherheitsbegrenzung: max. Anzahl an Schleifendurchläufen
	const int max_iterations = 20000000;
	int iteration_count = 0;
	int nodes_calculated = 0;

	// Solange offene Knoten vorhanden sind
	while (!open_list.empty()) {
		int current_index = open_list.front();
		open_list.pop_front();
		in_open_list[current_index] = false;

//		UtilityFunctions::print("open node ", current_index );

		VFNNode &current_node = map->nodes[current_index];

		for (VFNConnection &connection : current_node.connections ) {
			int other_index = connection.target_node_index;
			VFNNode &other_node = map->nodes[other_index];

			float effort = effort_map[current_index] + connection.effort;

			if( effort > highest_effort  ){
				highest_effort  = effort;
			}

//			UtilityFunctions::print("test neighbour ", other_index, " with effort ",effort , " contection-effort ",connection.effort );

			if( effort < effort_map[other_index] ){
				effort_map[other_index] = effort;
				target_map[other_index] = current_index;
				if (!in_open_list[other_index]) {
//					UtilityFunctions::print("open neighbour ", other_index, " with effort ",effort );
					open_list.push_back( other_index );
					in_open_list[other_index] = true;
				}
			}
		}

		nodes_calculated++;

		// TODO: Zugriff auf VFNMap und Verarbeitung von Nachbarn etc.
		iteration_count++;
		if (iteration_count >= max_iterations) {
			UtilityFunctions::printerr("Abbruch: Max. Iterationsanzahl erreicht (", max_iterations, ")");
			break;
		}
	}

	for (int index = 0; index < target_map.size(); index++) {
		vector_map[index] = map->nodes[index].world_position.direction_to(map->nodes[target_map[index]].world_position);
	}

	UtilityFunctions::print("nodes calculated ", nodes_calculated);

	Dictionary result;

	// Konvertieren in Godot-kompatible Arrays
	PackedFloat32Array godot_effort_map;
	godot_effort_map.resize(effort_map.size());
	for (size_t i = 0; i < effort_map.size(); i++) {
		godot_effort_map[i] = effort_map[i];
	}

	PackedInt32Array godot_target_map;
	godot_target_map.resize(target_map.size());
	for (size_t i = 0; i < target_map.size(); i++) {
		godot_target_map[i] = target_map[i];
	}

	PackedVector3Array godot_vector_map;
	godot_vector_map.resize(vector_map.size());
	for (size_t i = 0; i < vector_map.size(); i++) {
		godot_vector_map[i] = vector_map[i];
	}

	result["effort_map"] = godot_effort_map;
	result["target_map"] = godot_target_map;
	result["vector_map"] = godot_vector_map;
	result["highest_effort"] = highest_effort;

	return result;
}


void VFNField::_bind_methods() {
	ClassDB::bind_method(D_METHOD("add_target"), &VFNField::add_target);
	ClassDB::bind_method(D_METHOD("clear_targets"), &VFNField::clear_targets);
	ClassDB::bind_method(D_METHOD("remove_target"), &VFNField::remove_target);
	ClassDB::bind_method(D_METHOD("calculate"), &VFNField::calculate);
}