/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : game_managers/obstacle_manager.rs

Copyright (C) 2020 CJ McAllister
    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 3 of the License, or
    (at your option) any later version.
    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software Foundation,
    Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301  USA

Purpose:
    This module manages all active obstacles in the game, as well as providing
    Utility Methods for obstacle drawing, moving, etc.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use cast_iron::{
    element::Elemental,
    hex_directions,
    logger,
    mechanics::obstacle::Obstacle,
    Locatable,
};

use ggez::{
    Context as GgEzContext,
    graphics as ggez_gfx,
    mint as ggez_mint,
};

use crate::{
    game_assets::{
        colors,
        hex_grid_cell::HexGridCell,
    },
    game_managers::DrawableMechanic,
    ci_log,
};


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

//TODO: Proper implementation of an error type
#[derive(Debug)]
pub struct ObstacleError;

pub struct ObstacleManager {
    logger:         logger::Instance,
    obstacles:      Vec<Obstacle>,
    obstacle_mesh:  ggez_gfx::Mesh,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl ObstacleManager {
    /// Generic Constructor - creates an empty instance
    pub fn new(logger_original: &logger::Instance, ctx: &mut GgEzContext) -> Self {
        // Clone the logger instance so this module has its own sender to use
        let logger_clone = logger_original.clone();

        ObstacleManager {
            logger:         logger_clone,
            obstacles:      Vec::new(),
            obstacle_mesh:  ggez_gfx::Mesh::new_line(
                                ctx,
                                &[ggez_mint::Point2 {x: 0.0, y: 0.0}, ggez_mint::Point2 {x: 10.0, y: 10.0}],
                                ::DEFAULT_LINE_WIDTH,
                                ::DEFAULT_LINE_COLOR)
                                .unwrap(),
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

impl DrawableMechanic for ObstacleManager {
    type Instance = Obstacle;
    type ErrorType = ObstacleError;

    fn instances(&self) -> &Vec<Self::Instance> {
        &self.obstacles
    }

    fn push_instance(&mut self, instance: Self::Instance) {
        ci_log!(self.logger, logger::FilterLevel::Debug,
            "Adding {} obstacle starting at {} to mesh.",
            String::from(instance.element()),
            instance.origin());

        self.obstacles.push(instance);
    }

    fn mesh(&self) -> &ggez_gfx::Mesh {
        &self.obstacle_mesh
    }

    fn set_mesh(&mut self, mesh: ggez_gfx::Mesh) {
        self.obstacle_mesh = mesh;
    }

    fn add_instance_to_mesh_builder(instance: &Self::Instance,
                                    mesh_builder: &mut ggez_gfx::MeshBuilder,
                                    ggez_ctx: &mut GgEzContext) -> Result<(),Self::ErrorType> {
        // Get all coords for current obstacle instance
        let all_obstacle_coords = instance.all_coords();

        //OPT: *PERFORMANCE* Do this in advance and pass in
        // Get window dimensions
        let (window_x, window_y) = ggez_gfx::size(ggez_ctx);
        let window_center = ggez_mint::Point2 {
            x: window_x / 2.0,
            y: window_y / 2.0
        };

        // Iterate through current obstacle's coords, adding hexes to the mesh for each
        for (i, obstacle_coords) in all_obstacle_coords.iter().enumerate() {
            //OPT: *PERFORMANCE* Not a great spot for this conversion logic...
            // Calculate x, y offsets to determine (x,y) centerpoint from hex grid coords
            let x_offset = obstacle_coords.x() as f32 * (::CENTER_TO_VERTEX_DIST * 3.0);
            let y_offset = (-obstacle_coords.y() as f32 * f32::from(hex_directions::Side::NORTHWEST).sin() * (::CENTER_TO_SIDE_DIST * 2.0)) +
                           (-obstacle_coords.z() as f32 * f32::from(hex_directions::Side::SOUTHWEST).sin() * (::CENTER_TO_SIDE_DIST * 2.0));

            let obstacle_center = ggez_mint::Point2 {
                x: window_center.x + x_offset,
                y: window_center.y + y_offset
            };

            // Create a HexGridCell object and add it to the mesh builder
            let cur_hex = HexGridCell::new(obstacle_center, ::GRID_CELL_SIZE);
            cur_hex.add_to_mesh(colors::from_element(instance.element()), colors::DARKGREY, mesh_builder);

            //OPT: *DESIGN* This is basically an adjacency check, which would be a very useful function for the Coords module
            // Draw a line over the hex side between the new and previous obstacle cell for all but the first cell
            if i > 0 {
                let prev_obstacle_coords = all_obstacle_coords.get(i-1).unwrap();

                let direction = hex_directions::Side::from(*prev_obstacle_coords - *obstacle_coords);
                // Determine direction of hex side that should be overwritten

                //OPT: *STYLE* oh my god...
                // Get the vertices for the direction's side
                let shared_line = [*cur_hex.vertices().get(usize::from(hex_directions::Side::get_adjacent_vertices(direction).0)).unwrap(),
                                   *cur_hex.vertices().get(usize::from(hex_directions::Side::get_adjacent_vertices(direction).1)).unwrap()];

                mesh_builder.line(&shared_line, ::DEFAULT_LINE_WIDTH, colors::from_element(instance.element())).unwrap();
            }
        }

        Ok(())
    }
}



