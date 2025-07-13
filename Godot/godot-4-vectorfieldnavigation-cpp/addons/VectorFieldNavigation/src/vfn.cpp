#pragma once

#include <godot_cpp/classes/node3d.hpp>
#include <godot_cpp/core/class_db.hpp>

using namespace godot;

class vfn : public Node3D {
    GDCLASS(vfn, Node3D)

protected:
    static void _bind_methods();

public:
    vfn();
    ~vfn();

    void _process(double delta);
};