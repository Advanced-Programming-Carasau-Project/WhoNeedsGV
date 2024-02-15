use bevy::prelude::*;
use crate::energy::components::{IsEnergy};
use crate::events::EnergyUpdated;

pub fn update_energy(
    mut query: Query<&mut Text, With<IsEnergy>>,
    mut er_energy_updated: EventReader<EnergyUpdated>
)
{
    //println!("Dentro update_energy");
    for event in er_energy_updated.read(){
        if let Ok(mut t) = query.get_single_mut() {

            let percentage = event.total_energy as f32 / 10.0;

            let mut tmp = String::from("\nEnergy: ");
            tmp.push_str(percentage.to_string().as_str());
            tmp.push_str("% (");
            tmp.push_str(event.total_energy.to_string().as_str());
            tmp.push_str(")\n");

            t.sections[1].value = tmp;
        }
    }
}
