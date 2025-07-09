extends Node3D
class_name VFN

@export var cell_count:Vector2i = Vector2i(128,128)
@export var cell_size:Vector2 = Vector2.ONE
@export var heightmap:PackedFloat32Array 
var map:VFNMap


func _init() -> void:
	reinit()

func reinit():
	heightmap.resize( cell_count.x * cell_count.y )
	heightmap.fill(1.1)
	map = VFNMap.new()
	var t = Time.get_ticks_msec()
	map.set_heightmap_data( cell_count, heightmap )
	print("T: ",Time.get_ticks_msec()-t," msec")

	t = Time.get_ticks_msec()
	map.build_nodes_from_heightmap();
	print("T: ",Time.get_ticks_msec()-t," msec")
