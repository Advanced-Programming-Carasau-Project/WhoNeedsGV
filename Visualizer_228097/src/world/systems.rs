use bevy::prelude::*;
use crate::events::TileContentUpdated;
use crate::systems;
use crate::systems::get_path_content;
use crate::world::components::TileHub;

pub fn update_tile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut BackgroundColor, &TileHub), With<TileHub>>,
    mut er_tile_content_updated: EventReader<TileContentUpdated>,
)
{
    for event in er_tile_content_updated.read() {
        for (e, mut b, t) in query.iter_mut() {
            if t.r == event.position.0 && t.c == event.position.1 {   //Aggiorno la tile
                //println!("Trovata tile {:?} in posizione [{}][{}]", b, t.r, t.c);
                b.0 = systems::give_color(event.new_tile.tile_type.clone());
                let child_entity = commands
                    .spawn(ImageBundle {
                        style: Style {
                            width: Val::Percent(90.0),
                            height: Val::Percent(90.0),
                            align_self: AlignSelf::Center,
                            justify_self: JustifySelf::Center,
                            ..default()
                        },
                        image: asset_server.load(get_path_content(event.new_tile.content.clone())).into(),
                        ..default()
                    })
                    .id();
                commands.entity(e).replace_children(&[child_entity]);
                //println!("Rimpiazzata con tile {:?} in posizione [{}][{}]", b, t.r, t.c);
            }
        }
    }
}


