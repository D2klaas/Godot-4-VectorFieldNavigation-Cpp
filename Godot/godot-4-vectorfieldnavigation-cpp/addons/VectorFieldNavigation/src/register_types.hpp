#ifndef GDEXAMPLE_REGISTER_TYPES_H
#define GDEXAMPLE_REGISTER_TYPES_H

#include <godot_cpp/core/class_db.hpp>

using namespace godot;

void initialize_vfn_module(ModuleInitializationLevel p_level);
void uninitialize_vfn_module(ModuleInitializationLevel p_level);

#endif // GDVFN_REGISTER_TYPES_H