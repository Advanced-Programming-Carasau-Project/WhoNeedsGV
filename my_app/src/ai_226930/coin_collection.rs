use ohcrab_collection::collection::{CollectTool, LibErrorExtended};
use robotics_lib::world::tile::Content::Coin;
use robotics_lib::world::World;
use crate::LunaticRobot;

impl LunaticRobot{
    pub fn day(&mut self, world: &mut World){
        //println!("COIN ROUTINE");
        let result = CollectTool::collect_content(self, world, &Coin(1), 20, self.robot.energy.get_energy_level());
        match result{
            Ok(_quantity) => {
                //println!("coin trovate");
                if self.get_remaining_backpack_space() < 5{
                    self.must_empty = true;
                }
                self.put_content(Coin(0), world);
            }
            Err(err) => {
                match err{
                    LibErrorExtended::CommonError(_) => {}
                    LibErrorExtended::NoSolution => {
<<<<<<< HEAD
                        //println!("no_solution for coin");
=======
                        println!("no_solution for coin");
                        if let Some(coin) = self.robot.backpack.get_contents().get(&Coin(0)){
                            if coin.clone() > 0{
                                self.put_content(Coin(0), world);
                            }
                        }
>>>>>>> 4b0bba9901c58a77a23aa41facf1f605792df2f4
                        self.explore(world);
                    }
                    LibErrorExtended::RobotMapEmpty => {}
                    LibErrorExtended::NoWalkableTile => {}
                    LibErrorExtended::EnergyOutOfLimit => {}
                }
            }
        }
    }
}