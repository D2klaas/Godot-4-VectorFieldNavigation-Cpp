extends Node3D

var map:VFNMap
var t:int

func tick( what:String ):
	if t > 0:
		print(what+": ",Time.get_ticks_msec() - t, " msec")
	t = Time.get_ticks_msec()
	

func _ready() -> void:
	tick("start")
	map = VFNMap.new()
	tick("init")
	map.map_size = Vector2i(256,256)
	map.cell_size = Vector2(1,1)
	map.build_nodes()
	tick("structure created")
	var heightmap = PackedFloat32Array()
	heightmap.resize( map.map_size.x * map.map_size.y )
	heightmap.fill( 1.1 )
	tick("heightmap build")
	map.set_heightmap_data( heightmap )
	tick("heightmap set")
	
	map.set_height( 10, 5.5 )
	print(map.get_height( 10 ))
	
	pass
