// Datei: src/VFNField.hpp
#pragma once
#include <godot_cpp/classes/ref_counted.hpp>
#include <godot_cpp/core/class_db.hpp>
#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/packed_float32_array.hpp>
#include <godot_cpp/variant/packed_int32_array.hpp>
#include <algorithm> // für std::find
#include <queue>
#include <limits>
#include "VFNMap.hpp"

namespace godot {

	class VFNMap; // <- Forward declaration

	class VFNField : public RefCounted {
		GDCLASS(VFNField, RefCounted)

	protected:
		static void _bind_methods();

	public:
		VFNField();
		~VFNField();

		void add_target( int index );
		void clear_targets();
		void remove_target( int index );
		Dictionary calculate( );

	public:
		Ref<VFNMap> map;
		std::vector<int> targets;
		std::vector<int> open_list;
		std::vector<float> effort_map;
		std::vector<int> target_map;
		std::vector<Vector3> vector_map;
		float highest_effort ;
	};

} // namespace godot
