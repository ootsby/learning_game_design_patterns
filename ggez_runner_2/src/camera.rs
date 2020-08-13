use super::{CustomError, Grid, Meshes};
use ggez::graphics::DrawParam;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{graphics, Context};
use std::sync::mpsc::Receiver;

pub struct Camera {
    pub location: Vector2<f32>,
    width: f32,
    height: f32,
    player_moved_event_receive: Receiver<f32>,
}

impl Camera {
    pub fn new(
        location_x: f32,
        location_y: f32,
        width: f32,
        height: f32,
        player_moved_event_receive: Receiver<f32>,
    ) -> Camera {
        Camera {
            location: Vector2::new(location_x, location_y),
            width,
            height,
            player_moved_event_receive,
        }
    }

    pub fn draw(
        &self,
        grid: &Grid,
        meshes: &Meshes,
        context: &mut Context,
    ) -> Result<(), CustomError> {
        let game_objects = grid.query(
            self.location.x,
            self.location.y,
            self.location.x + self.width,
            self.location.y + self.height,
        );

        graphics::push_transform(
            context,
            Some(
                DrawParam::new()
                    .dest(Point2::from(self.location * -1.0))
                    .to_matrix(),
            ),
        );
        if let Err(error) = graphics::apply_transformations(context) {
            return Err(CustomError::GgezGameError(error));
        }
        for game_object in game_objects {
            game_object.draw(meshes, context)?;
        }
        graphics::pop_transform(context);

        Ok(())
    }

    pub fn update(&mut self) {
        if let Ok(player_moved_x) = self.player_moved_event_receive.try_recv() {
            self.location.x += player_moved_x
        }
    }
}
