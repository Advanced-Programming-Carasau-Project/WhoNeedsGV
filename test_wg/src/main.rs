use rocket::launch;
use rocket::get;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use robotics_lib::world::tile::Tile;
use who_needs_gv_world_generator::WorldGenerator;
use robotics_lib::world::world_generator::Generator;
use who_needs_gv_world_generator::Biome;

#[derive(Serialize)]
struct World{
    size :usize,
    tiles: Vec<Vec<Tile>>,
    biomes: Vec<Vec<Biome>>,
}

#[get("/get_data")]
fn get_data() -> Json<World> {
    let size: usize = 500; //max: 1024 potenze di 2 per non avere problemi con la canvas

    let mut world_generator = WorldGenerator::new(size);


    //world_generator.e_seed = 1688614679299147791;
    //world_generator.m_seed = 15581163141988959099;
    //world_generator.t_seed = 15151208778076085932;

    //world_generator.trees = false; //per generare mondi più velocemente se non ti interessano gli alberi
    //world_generator.teleports = false;
    world_generator.rivers = false;

    println!("world_generator.e_seed: {}",world_generator.e_seed);
    println!("world_generator.m_seed: {}",world_generator.m_seed);

    let generato = world_generator.gen();

    let mondo = World{
        size: world_generator.size,
        tiles: generato.0,
        biomes: world_generator.biomes,
    };

    println!("[.............................. ] sending to JS");

    Json(mondo)
}
#[launch]
fn rocket()->_{
    rocket::build().mount("/", routes![get_data]).mount("/", rocket::fs::FileServer::from("static"))
}