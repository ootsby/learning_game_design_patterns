use super::GameObject;
use ggez::nalgebra::Vector2;
use ggez::GameResult;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub struct Grid {
    cell_width: f32,
    cell_height: f32,
    cells: Vec<Vec<Vec<u64>>>,
    world_width: usize,
    world_height: u8,
    game_object_off_grid_event: Sender<u64>,
}

impl Grid {
    pub fn new(
        cell_width: f32,
        cell_height: f32,
        world_height: u8,
        world_width: usize,
        game_object_off_grid_event: Sender<u64>,
    ) -> GameResult<Grid> {
        let mut cells = vec![];

        for _y_count in 0..world_height {
            let mut row = vec![];
            for _x_count in 0..world_width {
                row.push(vec![]);
            }
            cells.push(row);
        }

        Ok(Grid {
            cell_width,
            cell_height,
            cells,
            world_width,
            world_height,
            game_object_off_grid_event,
        })
    }

    pub fn add(&mut self, game_object: &GameObject) {
        let x_index = (game_object.location.x / self.cell_width) as usize;
        let y_index = (game_object.location.y / self.cell_height) as usize;

        self.cells[y_index][x_index].push(game_object.id);
    }

    pub fn query<'a>(
        &self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        game_objects: &'a HashMap<u64, GameObject>,
    ) -> Vec<&'a GameObject> {
        let mut result = vec![];

        let index_start_x = (start_x / self.cell_width) as usize;
        let index_start_y = (start_y / self.cell_height) as usize;
        let index_end_x = (end_x / self.cell_width) as usize;
        let index_end_y = (end_y / self.cell_height) as usize;

        for y_index in index_start_y..index_end_y {
            for x_index in index_start_x..index_end_x {
                if y_index >= self.cells.len() || x_index >= self.cells[0].len() {
                    continue;
                }
                for id in self.cells[y_index][x_index].iter() {
                    if let Some(game_object) = game_objects.get(id) {
                        result.push(game_object);
                    }
                }
            }
        }

        result
    }

    pub fn update(
        &mut self,
        game_objects: &mut HashMap<u64, GameObject>,
        gravity_force: Vector2<f32>,
    ) {
        self.move_game_objects(game_objects, gravity_force);
    }

    fn move_game_objects(
        &mut self,
        game_objects: &mut HashMap<u64, GameObject>,
        gravity_force: Vector2<f32>,
    ) {
        // run update on all game objects
        for game_object in game_objects.values_mut() {
            let previous_index_x = (game_object.location.x / self.cell_width) as usize;
            let previous_index_y = (game_object.location.y / self.cell_height) as usize;
            game_object.update(gravity_force);
            let next_index_x = (game_object.location.x / self.cell_width) as usize;
            let next_index_y = (game_object.location.y / self.cell_height) as usize;
            if previous_index_x == next_index_x && previous_index_y == next_index_y {
                continue;
            }
            self.cells[previous_index_y][previous_index_x].retain(|id| id != &game_object.id);
            if next_index_x >= self.world_width || next_index_y >= self.world_height.into() {
                println!("Object with id {} is out of the grid", game_object.id);
                if let Err(error) = self.game_object_off_grid_event.send(game_object.id) {
                    println!("error sending game object off grid event: {}", error);
                }
            } else {
                self.cells[next_index_y][next_index_x].push(game_object.id);
            }
        }
    }

    pub fn remove(&mut self, game_object: &GameObject) {
        let x_index = (game_object.location.x / self.cell_width) as usize;
        let y_index = (game_object.location.y / self.cell_height) as usize;

        if x_index < self.world_width && y_index < self.world_height.into() {
            self.cells[y_index][x_index].retain(|game_object_id| game_object_id != &game_object.id);
        }
    }
}
