use bevy::prelude::{Component};

#[derive(Component)]
pub struct Moving;

#[derive(Component)]
pub struct Line {
    pub index: usize,
    pub x: f32,
    pub active: bool,
}

#[derive(Component)]
pub struct Slide {
    pub length: f32,
    pub active: bool,
}

#[derive(Component)]
pub struct Note {
    pub active: bool,
}

#[derive(Component)]
pub struct Playing;

#[derive(Component)]
pub struct Contact {
    pub index: usize,
    pub x: f32
}

