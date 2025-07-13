// Datei: src/VFNMap.hpp
#pragma once
#include <godot_cpp/classes/ref_counted.hpp>
#include <godot_cpp/core/class_db.hpp>
#include <godot_cpp/variant/vector2i.hpp>
#include <godot_cpp/variant/vector2.hpp>
#include <godot_cpp/variant/packed_float32_array.hpp>
#include <vector>
#include "VFNField.hpp"

namespace godot {

	struct VFNConnection {
		int target_node_index = -1;
		float effort = 0.0f;
	};

	struct VFNNode {
		Vector3 world_position;
		float height = 0.0f;
		int index = -1;
		bool disabled = false;
		std::vector<VFNConnection> connections;
	};


	class VFNField; // <- Forward declaration

	class VFNMap : public RefCounted {
		GDCLASS(VFNMap, RefCounted)

		protected:
			static void _bind_methods();

		public:
			Ref<VFNField> create_field();
			void build_nodes();
			void set_heightmap_data(const PackedFloat32Array& values );
			void set_height( int index, float height );
			float get_height( int index );

			void set_map_size( Vector2i size );
			Vector2i get_map_size();
			void set_cell_size( Vector2i size );
			Vector2i get_cell_size();

		public:
			VFNMap();
			~VFNMap();

		public:
			std::vector<VFNNode> nodes;
			Vector2i map_size;
			Vector2 cell_size;
	};

} // namespace godot