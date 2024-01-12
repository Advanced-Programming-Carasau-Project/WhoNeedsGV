mod maze_world_gen;

use robotics_lib::interface::{Direction, Tools};
use robotics_lib::runner::Runnable;
use robotics_lib::world::tile::{Content, TileType};
use robotics_lib::world::World;
use robotics_lib::interface::where_am_i;
use robotics_lib::interface::robot_view;
use robotics_lib::interface::robot_map;
use robotics_lib::interface::go;
use robotics_lib::world::tile::Tile;
use robotics_lib::utils::LibError;
use robotics_lib::runner::{Robot};
use robotics_lib::event::events::Event;
use robotics_lib::energy::Energy;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::runner::Runner;
use tool::StreetExplorer;

use rand::Rng;
use crate::maze_world_gen::Maze;
struct CarasauRobot(Robot);
impl Runnable for CarasauRobot{
    fn process_tick(&mut self, world: &mut World) { //nel tick invoco il tools che ho definito
        println!("{:?}", StreetExplorer::explore_street(self, world, None, None)); //esploro le strade a caso
        let (y, x) = where_am_i(self, world).1;
        println!("coordinate_robot: {}-{}", y, x);

        self.0.energy = Robot::new().energy; //ricaricato l'energia del robot

        println!("{:?}", StreetExplorer::explore_street(self, world, None, Some(Direction::Left)));
        let (y, x) = where_am_i(self, world).1;
        println!("coordinate_robot: {}-{}", y, x);

        self.0.energy = Robot::new().energy; //ricaricato l'energia del robot

        println!("{:?}", StreetExplorer::explore_street(self, world, Some(Content::Coin(1)), Some(Direction::Right)));
        let (y, x) = where_am_i(self, world).1;
        println!("coordinate_robot: {}-{}", y, x);

        self.0.energy = Robot::new().energy; //ricaricato l'energia del robot

        println!("{:?}", StreetExplorer::explore_street(self, world, Some(Content::Coin(1)), Some(Direction::Left)));
        let (y, x) = where_am_i(self, world).1;
        println!("coordinate_robot: {}-{}", y, x);

        //stampo a video tutta la mappa che conosce il robot
        let robot_map = robot_map(world).unwrap();
        for i in 0..robot_map.len(){
            for j in 0..robot_map.len(){
                match &robot_map[i][j] {
                    None => { print!("?"); }
                    Some(t) => {
                        match t.tile_type{
                            TileType::Street => { print!("-"); },
                            TileType::Wall => { print!("#"); },
                            _ => {}
                        }
                    }
                }
            }
            println!("");
        }
    }
    fn handle_event(&mut self, event: Event) {
        // react to this event in your GUI
    }
    fn get_energy(&self) -> &Energy {
        &self.0.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.0.energy
    }
    fn get_coordinate(&self) -> &Coordinate {
        &self.0.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate{
        &mut self.0.coordinate
    }
    fn get_backpack(&self) -> &BackPack {
        &self.0.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.0.backpack
    }
}

struct WorldGenerator {size: usize}
impl Generator for WorldGenerator { //creo un mondo che corrisponde ad un labirinto
fn gen(&mut self) -> robotics_lib::world::world_generator::World {
    let mut map: Vec<Vec<Tile>> = vec![vec![Tile{
        tile_type: TileType::Wall,
        content: Content::None,
        elevation: 0,
    }; mapSize]; mapSize];

    let mut maze = Maze::new_from(mapSize-2);
    maze.generate_maze();
    maze.print();
    println!("");

    map[0][0].tile_type = TileType::Street;
    for i in 0..mapSize{
        for j in 0..mapSize{
            if maze.get_maze_cell(i, j) == false{
                map[i][j].tile_type = TileType::Street;
                if i == 3{
                    map[i][j].content = Content::Coin(1);
                    print!("@");
                }
                else {
                    print!("-");
                    map[i][j].content = Content::None;
                }
            }
            else{
                print!("#");
            }
            map[i][j].elevation = rand::thread_rng().gen_range(0..10);
        }
        println!("");
    }

    let weather_forecast = vec![
        WeatherType::Sunny,
        WeatherType::Rainy,
        WeatherType::Foggy,
        WeatherType::TropicalMonsoon,
    ];

    (
        map,
        (1, 0),
        EnvironmentalConditions::new(&weather_forecast, 30, 12).unwrap(),
        100.0,
        None,
    )
}
}

const mapSize: usize = 51;


fn main() {
    let robot = CarasauRobot(Robot::new()); //definisco il robot
    let mut generator = WorldGenerator{size: mapSize}; //definisco il world generator
    let mut run = Runner::new(Box::new(robot), &mut generator).unwrap(); //creo un runner (l'oggetto che gestisce i tick del mondo). Questa struc creerà il mondo grazie al world generator
    run.game_tick(); //faccio avanzare un tick del mondo (un tick corrisponde all'unità elementare di tempo nel "gioco")
}