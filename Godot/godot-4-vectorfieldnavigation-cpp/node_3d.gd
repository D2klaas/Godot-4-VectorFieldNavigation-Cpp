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
	
	for i in range(0,map.map_size.x * map.map_size.y):
		map.set_height( i, randf_range(1,100) )
	print(map.get_height( 10 ))
	
	var field = map.create_field()
	for i in 20:
		field.add_target( randi_range(0,map.map_size.x * map.map_size.y) )
	tick("prepared field")
	
	var result = field.calculate()
	print("highest: ",result["highest_effort"])
	tick("field calculated")
	
