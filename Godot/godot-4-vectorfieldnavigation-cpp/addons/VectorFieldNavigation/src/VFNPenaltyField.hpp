
// Datei: src/VFNPenaltyField.hpp
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

	class VFNPenaltyField : public RefCounted {
		GDCLASS(VFNPenaltyField, RefCounted)

	protected:
		static void _bind_methods();
	
	public:
		void set_effort( int index, float effort );
		float get_effort( int index );
		void init( );

	public:
		VFNPenaltyField();
		~VFNPenaltyField();

	public:
		Ref<VFNMap> map;
		std::vector<float> penalty_map;
		bool boolean;
		bool dynamic;
		bool upmost;
	};

} // namespace godot
