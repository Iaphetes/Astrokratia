use bevy::prelude::*;
#[derive(Component, PartialEq, Eq)]
pub enum Player {
    Host,
    AI(u32),
}
