use bevy::prelude::*;
#[derive(Component, PartialEq, Eq, Clone)]
pub enum Player {
    Host,
    AI(u32),
}
