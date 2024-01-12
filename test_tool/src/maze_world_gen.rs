use rand::prelude::SliceRandom;
pub struct Maze{
    inner_size: usize,
    maze: Vec<Vec<bool>>,
}

impl Maze{
    pub fn new_from(mut inner_size: usize) -> Self{
        if inner_size % 2 == 0{
            inner_size = inner_size - 1;
        }
        let maze = vec![vec![true; inner_size + 2]; inner_size + 2];
        Maze{
            inner_size,
            maze,
        }
    }

    fn initialize_maze(&mut self){
        let mut i = 1;
        let mut j = 1;
        while i<=self.inner_size {
            j = 1;
            while j<=self.inner_size{
                self.maze[i][j] = false;
                j = j + 2;
            }
            i = i + 2;
        }
    }

    fn return_cell_if_exists(&self, i: i32, j: i32) -> Option<(usize, usize)>{
        if i>=1 && i<=self.inner_size as i32 && j>=1 && j<=self.inner_size as i32 {
            Some((i as usize, j as usize))
        }
        else {
            None
        }
    }

    fn return_near_cells(&self, i: usize, j: usize) -> Vec<Option<(usize, usize)>>{
        let mut near_to_visit: Vec<Option<(usize, usize)>> = vec![];
        let y = i as i32;
        let x = j as i32;
        near_to_visit.push(self.return_cell_if_exists((y-2), x));
        near_to_visit.push(self.return_cell_if_exists((y+2), x));
        near_to_visit.push(self.return_cell_if_exists(y, (x-2)));
        near_to_visit.push(self.return_cell_if_exists(y, (x+2)));
        near_to_visit
    }

    fn break_wall_between(&mut self, current: (usize, usize), to_go: (usize, usize)){
        if current.0 < to_go.0{
            self.maze[current.0 + 1][current.1] = false;
        }
        else if current.0 > to_go.0 {
            self.maze[current.0 - 1][current.1] = false;
        }
        else if current.1 < to_go.1 {
            self.maze[current.0][current.1 + 1] = false;
        }
        else{
            self.maze[current.0][current.1 - 1] = false;
        }
    }

    fn randomized_dfs_search(&mut self, visited: &mut Vec<Vec<bool>>, i: usize, j: usize){
        visited[i][j] = true;
        let mut near_cells = self.return_near_cells(i, j);
        near_cells.shuffle(&mut rand::thread_rng());
        for option_cell in near_cells{
            if let Some(cell) = option_cell{
                if !visited[cell.0][cell.1] {
                    self.break_wall_between((i, j), cell);
                    /*println!("current: {:?} - to_go: {:?}", (i, j), cell);
                    self.print();
                    println!("");*/
                    self.randomized_dfs_search(visited,cell.0, cell.1);
                }
            }
        }
    }

    pub fn generate_maze(&mut self){
        self.initialize_maze();
        let mut visited = self.maze.clone();
        self.randomized_dfs_search(&mut visited, 1, 1);
        self.break_wall_between((0,0), (2, 0)); //entrata
        self.break_wall_between((self.inner_size+1,self.inner_size+1), (self.inner_size-1, self.inner_size+1)); //uscita
    }

    pub fn print(&self){
        for i in 0..self.inner_size +2 {
            for j in 0..self.inner_size + 2{
                if self.maze[i][j]{
                    print!("#");
                }
                else{
                    print!("-");
                }
            }
            println!();
        }
    }

    pub fn get_maze_cell(&self, i: usize, j: usize) -> bool{
        self.maze[i][j]
    }

    pub fn get_outer_size(&self) -> usize{
        self.maze.len()
    }

    pub fn get_inner_size(&self) -> usize{
        self.inner_size
    }
}