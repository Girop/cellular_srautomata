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
    state: bool,
}

enum NeighbourhoodRule {
    VonNeumann,
    Moore,
}

enum SpawnRule {}

enum ColorRule {
    XYZ,                  //Implement
    Distance,             //Implement
    White,                //Implement
    StateShading,         //Implement
    NeighbourhoodDensity, //Implement
}

fn generate_color(choosen_rule: ColorRule, cell_position: Vec3, neighbour_count: u8) -> Color {
    match choosen_rule {
        ColorRule::XYZ => (),
        ColorRule::Distance => (),
        ColorRule::White => Color::rgb(1., 1., 1.),
        ColorRule::StateShading => (),
        ColorRule::NeighbourhoodDensity => (),
    }
}
