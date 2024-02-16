use bevy::prelude::Component;

#[derive(Component)]
pub struct Log{}

#[derive(Component)]
pub struct Hour{}

#[derive(Component)]
pub struct ActualDayTime{}

#[derive(Component)]
pub struct Score{}

#[derive(Component)]
pub struct Elevation{}


pub const N_EVENT_IN_LOG:usize = 10;
