extends Node3D

func _init() -> void:
	var f = VFNMap.new()
	var n = "peng"
	print(f.label)
	var field = f.create_field()
	print(field.label)
	
