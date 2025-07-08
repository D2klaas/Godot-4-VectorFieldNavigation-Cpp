use godot::prelude::*;
use std::collections::{HashMap, VecDeque};

struct VectorFieldNaviation;

#[gdextension]
unsafe impl ExtensionLibrary for VectorFieldNaviation {
    fn on_level_init(level: InitLevel) {
        println!("[Rust]   Init level {level:?}");
    }

    fn on_level_deinit(level: InitLevel) {
        println!("[Rust]   Deinit level {level:?}");
    }
}

mod vfn_connection;
use vfn_connection::VFNConnection;

mod vfn_node;
use vfn_node::VFNNode;

mod vfn_field;
pub use vfn_field::VFNField;

mod vfn_map;
pub use vfn_map::VFNMap;

/* 
#[derive(GodotClass)]
#[class(init, base = RefCounted)]
pub struct VFNField {
    /// Größe des Feldes in Zellen
    #[export]
    pub size: Vector2i,

    /// Abschneidewert für maximalen Aufwand
    #[export]
    pub effort_cutoff: f32,

    /// Faktor für Aufstieg (Steigung > 0)
    #[export]
    pub climb_factor: f32,
    /// maximale erlaubte Steigung beim Aufstieg
    #[export]
    pub climb_cutoff: f32,
    /// Faktor für Abstieg (Steigung < 0)
    #[export]
    pub drop_factor: f32,
    /// maximale erlaubte Steigung beim Abstieg
    #[export]
    pub drop_cutoff: f32,

    /// Aufwandskarte (Entfernung zu Ziel unter Berücksichtigung aller Faktoren)
    pub effort_field: Vec<f32>,
    /// Zielrichtung jedes Feldes (Index des nächsten Schritts)
    pub aim_field: Vec<usize>,
    /// Bewegungsvektor pro Feld
    pub vector_field: Vec<Vector3>,
    /// Markierung offener Felder (Open List)
    pub open_mask: Vec<bool>,
    /// Liste offener Felder
    pub open_list: VecDeque<usize>,

    /// Zielpositionen (als Index im Feld)
    pub targets: Vec<usize>,
    /// Modifikatoren wie Wasser, Lava, Sand etc.
    pub mod_fields: HashMap<String, Vec<f32>>,
    /// Gewichtung der Modifikatoren
    pub mod_weights: HashMap<String, f32>,
}

#[godot_api]
impl VFNField {
    /// Fügt ein Ziel zur Liste hinzu
    #[func]
    fn add_target(&mut self, pos: Vector2i) {
        let index = (pos.x + pos.y * self.size.x) as usize;
        self.targets.push(index);
    }

    /// Entfernt alle gesetzten Ziele
    #[func]
    fn clear_targets(&mut self) {
        self.targets.clear();
    }

    /// Setzt die Gewichtung für ein Modifikatorfeld
    #[func]
    fn set_mod_weight(&mut self, name: GString, weight: f32) {
        self.mod_weights.insert(name.to_string(), weight);
    }

    /// Initialisiert alle Felder
    #[func]
    fn init_fields(&mut self) {
        let total = (self.size.x * self.size.y) as usize;
        self.effort_field = vec![f32::MAX; total];
        self.aim_field = vec![usize::MAX; total];
        self.vector_field = vec![Vector3::ZERO; total];
        self.open_mask = vec![false; total];
        self.open_list = VecDeque::with_capacity(total);
    }

    /// Hauptberechnung des Feldes (Dijkstra-ähnlich)
    #[func]
    fn calculate(&mut self) {
        let width = self.size.x as usize;
        let height = self.size.y as usize;
        let total = width * height;

        self.effort_field = vec![f32::MAX; total];
        self.aim_field = vec![usize::MAX; total];
        self.vector_field = vec![Vector3::ZERO; total];
        self.open_mask = vec![false; total];
        self.open_list.clear();

        for &target_index in &self.targets {
            self.effort_field[target_index] = 0.0;
            self.aim_field[target_index] = target_index;
            self.open_mask[target_index] = true;
            self.open_list.push_back(target_index);
        }

        let directions = [
            (-1, 0), (1, 0), (0, -1), (0, 1),
            (-1, -1), (1, -1), (1, 1), (-1, 1),
        ];

        while let Some(current_index) = self.open_list.pop_front() {
            self.open_mask[current_index] = false;

            let cx = current_index % width;
            let cy = current_index / width;
            let current_effort = self.effort_field[current_index];

            for (dx, dy) in directions.iter() {
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;

                if nx < 0 || ny < 0 || nx >= width as isize || ny >= height as isize {
                    continue;
                }

                let n_index = (nx as usize) + (ny as usize) * width;
                let mut step_effort = if dx.abs() + dy.abs() == 2 { 1.41 } else { 1.0 };

                for (name, field) in &self.mod_fields {
                    if let Some(weight) = self.mod_weights.get(name) {
                        step_effort += field[n_index] * weight;
                    }
                }

                let new_effort = current_effort + step_effort;

                if new_effort < self.effort_field[n_index] && new_effort < self.effort_cutoff {
                    self.effort_field[n_index] = new_effort;
                    self.aim_field[n_index] = current_index;
                    let cx_world = cx as f32;
                    let cy_world = cy as f32;
                    let nx_world = nx as f32;
                    let ny_world = ny as f32;
                    self.vector_field[n_index] = Vector3::new(cx_world - nx_world, 0.0, cy_world - ny_world).normalized();

                    if !self.open_mask[n_index] {
                        self.open_list.push_back(n_index);
                        self.open_mask[n_index] = true;
                    }
                }
            }
        }
    }

    /// Gibt das komplette Aufwandfeld zurück
    #[func]
    fn get_effort_array(&self) -> PackedFloat32Array {
        PackedFloat32Array::from_iter(self.effort_field.iter().copied())
    }

    /// Gibt das Bewegungsvektorfeld zurück
    #[func]
    fn get_vector_array(&self) -> PackedVector3Array {
        PackedVector3Array::from_iter(self.vector_field.iter().copied())
    }

    /// Gibt das Zielindexfeld zurück
    #[func]
    fn get_aim_array(&self) -> PackedInt32Array {
        PackedInt32Array::from_iter(self.aim_field.iter().map(|&x| x as i32))
    }
}


impl Default for VFNField {
    fn default() -> Self {
        Self {
            size: Vector2i::new(128, 128),
            effort_cutoff: f32::MAX,
            climb_factor: 0.0,
            climb_cutoff: f32::MAX,
            drop_factor: 0.0,
            drop_cutoff: f32::MAX,
            effort_field: Vec::new(),
            vector_field: Vec::new(),
            aim_field: Vec::new(),
            open_mask: Vec::new(),
            open_list: VecDeque::new(),
            targets: Vec::new(),
            mod_fields: HashMap::new(),
            mod_weights: HashMap::new(),
        }
    }
}
*/