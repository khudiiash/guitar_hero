use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Loader(pub Vec<HandleUntyped>);
