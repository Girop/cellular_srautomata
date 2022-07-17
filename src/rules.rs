use bevy::prelude::*;

enum NeighbourhoodMethod {
    VonNeumann,
    Moore,
}

enum ColorRule {
    Position,             //Implement
    Distance,             //Implement
    White,                //Implement
    StateShading,         //Implement
    NeighbourhoodDensity, //Implement
}

impl ColorRule{
    pub fn color(&self) -> Color{
        match self{
            Self::Position => (),
        }
    }
}

enum ValueRule{
    Single(Vec<bool>),
    Singles(Vec<bool>),
    Many(Vec<bool>)
}

impl ValueRule{
    pub fn generate_rule(&self) -> Vec<bool>{

    }
}

struct Rules{
    survival: ValueRule,
    birth: ValueRule,
    states: u8,
    neighbourhood: NeighbourhoodMethod,
    color: ColorRule
}


fn choose_rules(mut commands: Commands){

}