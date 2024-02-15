use bevy::prelude::States;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
//Debug, Clone, Copy, Eq, PartialEq, Hash, Default necessari per far funzionare States
pub enum AppState {
    #[default]
    GeneratingUi,
    RobotIsReady,
    ReadingEvents,
    ProcessingEventMove,
    ProcessingEventUpdateTile,
    ProcessingEventDiscoverTile,
    ProcessingEventUpdateEnergy,
    ProcessingEventTeleport,
    ProcessingEventUpdateEnvironment,
    ProcessingEventOther
}
