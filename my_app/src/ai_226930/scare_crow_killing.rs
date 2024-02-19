use std::collections::VecDeque;
use op_map::op_pathfinding::{get_best_action_to_element, OpActionInput, OpActionOutput, ShoppingList};
use robotics_lib::interface::{destroy, Direction, go, put, robot_map};
use robotics_lib::interface::Direction::{Down, Left, Right, Up};
use robotics_lib::runner::Runnable;
<<<<<<< HEAD
use robotics_lib::world::tile::Content::{Scarecrow};
=======
use robotics_lib::world::tile::Content::{Rock, Scarecrow, Tree};
>>>>>>> 4b0bba9901c58a77a23aa41facf1f605792df2f4
use robotics_lib::world::tile::TileType;
use robotics_lib::world::tile::TileType::Lava;
use robotics_lib::world::World;

use crate::{LunaticRobot};
use crate::ai_226930::valid_coords;

impl LunaticRobot{
    pub fn night(&mut self, world: &mut World){
        //println!("NIGHT ROUTINE");
        if self.lava_coords.is_none(){
            self.explore(world);
            if let Some(lava) = self.search_lava(world){
                self.lava_coords = Some(lava);
            }
        }
        else{
            let mut shopping_list = ShoppingList {
                list: vec![
                    (Rock(0), Some(OpActionInput::Destroy())),
                ],
            };

            while self.get_remaining_backpack_space() > 15 {
                let mut chicken_found = false;
                while !chicken_found {
                    // Get the best move
                    match get_best_action_to_element(self, world, &mut shopping_list) {
                        None => {
                            println!("no move from op_map found");
                            self.explore(world);
                            //if there are no chickens, I explore and then exit the routine
                            return;
                        }
                        Some(next_action) => {
                            //println!("action found: {:?}", next_action);
                            match next_action {
                                OpActionOutput::Move(dir) => {
                                    let res = go(self, world, dir);
                                    if res.is_err(){
                                        return;
                                    }
                                }
                                OpActionOutput::Destroy(dir) => {
                                    // println!("Destroy");
                                    let res = destroy(self, world, dir);
                                    if res.is_err(){
                                        return;
                                    }
                                    chicken_found = true;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            if let Some(lava_unwrap) = &self.lava_coords{
                let lava = (lava_unwrap.0, lava_unwrap.1);
                let direction = lava_unwrap.2.clone();
                self.move_to_coords(lava, world);
                self.replenish();
                let chicken_quantity = self.get_content_quantity(&Rock(0));
                let res = put(self, world,Rock(0),chicken_quantity, direction);
                println!("result put in lava: {:?}", res);
                self.lava_coords = None;
            }




        }
    }
    pub fn search_lava(&mut self, world: &mut World) -> Option<(usize, usize, Direction)>{
<<<<<<< HEAD
        //println!("looking for lava");
        //range where we are currently searching for the undiscovered tile
        let _range = 2usize;

=======
        println!("looking for lava");
>>>>>>> 4b0bba9901c58a77a23aa41facf1f605792df2f4
        let robot_x = self.get_coordinate().get_row();
        let robot_y = self.get_coordinate().get_col();
        //map as seen as the robot
        let known_map = robot_map(world).unwrap();
        let map_size = known_map.len() as i32;
        let mut flag = vec![vec![false; map_size as usize]; map_size as usize];
        let mut queue = VecDeque::new();
        for i in 0..flag.len() {
            for j in 0..flag[i].len() {
                match &known_map[i][j] {
                    None => {flag[i][j] = true}
                    Some(t) => {
                        match t.tile_type {
                            TileType::DeepWater => { flag[i][j] = true }
                            TileType::Wall => { flag[i][j] = true }
                            _ => {}
                        }
                    }
                }
            }
        }
        //coordinates of the tile
        queue.push_back((robot_x, robot_y));
        flag[robot_x][robot_y] = true;

        while !queue.is_empty(){
            let (x_u,y_u) = queue.pop_front().unwrap();
            let x_i = x_u as i32;
            let y_i = y_u as i32;
            if valid_coords(x_i -1, y_i, map_size) && !flag[x_u -1][y_u]{
                if let Some(tile) = &known_map[x_u -1][y_u]{
                    if tile.tile_type == Lava{
                        return Some((x_u, y_u, Up))
                    }
                }
                queue.push_back((x_u-1, y_u));
                flag[x_u -1][y_u] = true;
            }
            if valid_coords(x_i +1, y_i, map_size) && !flag[x_u +1][y_u]{
                if let Some(tile) = &known_map[x_u +1][y_u]{
                    if tile.tile_type == Lava{
                        return Some((x_u, y_u, Down))
                    }
                }
                queue.push_back((x_u+1, y_u));
                flag[x_u +1][y_u] = true
            }
            if valid_coords(x_i, y_i-1, map_size) && !flag[x_u][y_u-1]{
                if let Some(tile) = &known_map[x_u][y_u-1]{
                    if tile.tile_type == Lava{
                        return Some((x_u, y_u, Left))
                    }
                }
                queue.push_back((x_u, y_u-1));
                flag[x_u][y_u-1] = true;
            }
            if valid_coords(x_i, y_i+1, map_size) && !flag[x_u][y_u+1]{
                if let Some(tile) = &known_map[x_u][y_u+1]{
                    if tile.tile_type == Lava{
                        return Some((x_u, y_u, Right))
                    }
                }
                queue.push_back((x_u, y_u+1));
                flag[x_u][y_u+1] = true;
            }

        }
        return None;
    }
}