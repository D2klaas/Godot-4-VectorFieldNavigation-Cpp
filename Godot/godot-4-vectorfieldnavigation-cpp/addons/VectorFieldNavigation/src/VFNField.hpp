// Datei: src/VFNField.hpp
#pragma once
#include <godot_cpp/classes/ref_counted.hpp>
#include <godot_cpp/core/class_db.hpp>
#include <algorithm> // für std::find
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

		add_target( int index );
		clear_targets();
		remove_target( int index );
		//void _process(double delta) override;

	public:
		Ref<VFNMap> map;
		std::vector<int> targets;
	};

} // namespace godot
