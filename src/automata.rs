use bevy::prelude::*;

pub struct automataPlugin;

impl Plugin for automataPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}

#[derive(Component)]
struct Automata {
    size: Vec3,
}

#[derive(Component)]
struct Cell {
    neighbour_count: u8,
    color: Color,
    state: bool, // change -> u8
}

// Create box where fun stuff happens
// => startup system creating box
// => cell creation function
// => normal system checking box
// => events when cell in box exists
// => event reader function to run rules on cells
// Attach egui to struct Automata
// ? Starting condition/ how ? -> User created / predefined

fn setup_automata(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let starting_size = 64;
    let cell_container: Vec<Vec<Vec<_>>> = Vec::new();

    // Some math to always center this around (0,0,0)
    for i in 0..starting_size {
        for j in 0..starting_size {
            for k in 0..starting_size {}
        }
    }
}
