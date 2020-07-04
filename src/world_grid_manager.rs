/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : environment\world_grid_manager.rs

Copyright (C) 2017 CJ McAllister
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
    This module rovides functions to determine interactions between various objects
    in the world grid.
    
Changelog:

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::f32::consts::PI;
use std::collections::HashMap;

use ggez::{
    graphics as ggez_gfx,
    mint as ggez_mint
};

use ::game_assets::colors::*;


///////////////////////////////////////////////////////////////////////////////
//  Constants
///////////////////////////////////////////////////////////////////////////////

const GRID_CELL_SIZE: f32 = 30.0;

// Y_OFFSET = GRID_CELL_SIZE * sin(pi/3) * 2
// Distance from centerpoint of hex to center of a side 
static Y_OFFSET: f32 = GRID_CELL_SIZE * 0.86602540378;


///////////////////////////////////////////////////////////////////////////////
// Data structures
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Hash)]
pub enum Direction {
    EAST,
    NORTHEAST,
    NORTH,
    NORTHWEST,
    WEST,
    SOUTHWEST,
    SOUTH,
    SOUTHEAST
}
// Equivalence comparison
impl PartialEq for Direction {
    fn eq(&self, other: &Direction) -> bool {
        self == other
    }
}
impl Eq for Direction {}

lazy_static! {
    pub static ref HEX_SIDES: HashMap<Direction, f32> = {
        let mut m = HashMap::new();

        m.insert(Direction::NORTHEAST, PI/6.0);
        m.insert(Direction::NORTH,     PI/2.0);
        m.insert(Direction::NORTHWEST, 5.0*PI/6.0);
        m.insert(Direction::SOUTHWEST, 7.0*PI/6.0);
        m.insert(Direction::SOUTH,     3.0*PI/2.0);
        m.insert(Direction::SOUTHEAST, 11.0*PI/6.0);

        m
    };
}

lazy_static! {
    pub static ref HEX_VERTICES: HashMap<Direction, f32> = {
        let mut m = HashMap::new();

        m.insert(Direction::EAST,       0.0);
        m.insert(Direction::NORTHEAST,  PI/3.0);
        m.insert(Direction::NORTHWEST,  2.0*PI/3.0);
        m.insert(Direction::WEST,       PI);
        m.insert(Direction::SOUTHWEST,  4.0*PI/3.0);
        m.insert(Direction::SOUTHEAST,  5.0*PI/3.0);

        m
    };
}

pub struct WorldGridManager {
    pub max_radial_distance: u32, // Maximum value for an axis of the hex grid
}


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

impl WorldGridManager {
    pub fn new(max_radial_distance: u32) -> WorldGridManager {
        WorldGridManager {
            max_radial_distance: max_radial_distance,
        }
    }    

    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////
     
    pub fn get_grid_size(&self) -> u32 {
        self.max_radial_distance
    }

    /// Draws a baseline hex grid to the graphics window.
    pub fn draw_grid(
        &self,
        center: ggez_mint::Point2<f32>,
        mesh_builder: &mut ggez_gfx::MeshBuilder
    ) {
        // Draw GRID_CELL_SIZE-width hexagon sides recursively
        self.recursive_hex_draw(WHITE, center, 0, mesh_builder);

        let spoke_color = GREEN;
        // Draw spokes recursively in all directions
        for (_dir, theta) in HEX_VERTICES.iter() {
            // Determine origin point for current direction
            let origin = ggez_mint::Point2 {
                x: center.x + (GRID_CELL_SIZE * theta.cos()),
                y: center.y - (GRID_CELL_SIZE * theta.sin())
            };
            self.recursive_spoke_draw(spoke_color, origin, *theta, 0, mesh_builder);
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Helper Functions
    ///////////////////////////////////////////////////////////////////////////

    /// Draws a hex grid at the given level using recursive calls radiating out
    /// from the given center.
    fn recursive_hex_draw(
        &self,
        color: ggez_gfx::Color,
        center: ggez_mint::Point2<f32>,
        level: u32,
        mut input_mesh: &mut ggez_gfx::MeshBuilder
    ) {
        // Final level exit case
        if level == self.max_radial_distance {
            return;
        }

        // HEX_SIZE to be used to correctly translate levels > 0
        static HEX_SIZE: f32 = Y_OFFSET * 2.0;
        
        // Draw a parallel line and dispatch a spoke draw call at the current level
        // for each intercardinal direction.
        for (_dir, theta) in HEX_SIDES.iter() {
            // Calculate parallel line endpoints
            let mut endpt_x = center.x + GRID_CELL_SIZE * (theta - PI/6.0).cos();
            let mut endpt_y = center.y - GRID_CELL_SIZE * (theta - PI/6.0).sin();
            let mut endpt_a = ggez_mint::Point2 {
                x: endpt_x,
                y: endpt_y
            };

            endpt_x = center.x + GRID_CELL_SIZE * (theta + PI/6.0).cos();
            endpt_y = center.y - GRID_CELL_SIZE * (theta + PI/6.0).sin();
            let mut endpt_b = ggez_mint::Point2 {
                x: endpt_x,
                y: endpt_y
            };

            // Translate lines based on level
            endpt_a.x = endpt_a.x + level as f32 * (HEX_SIZE * theta.cos());
            endpt_a.y = endpt_a.y - level as f32 * (HEX_SIZE * theta.sin());
            endpt_b.x = endpt_b.x + level as f32 * (HEX_SIZE * theta.cos());
            endpt_b.y = endpt_b.y - level as f32 * (HEX_SIZE * theta.sin());

            // Add the line to the GGEZ mesh builder
            input_mesh = input_mesh.line(&[endpt_a, endpt_b], 1.0, WHITE).unwrap();
        }
        
        // Make the recursive call
        self.recursive_hex_draw(color, center, level+1, input_mesh);
    }

    /// Draws a spoke (i.e. -<) from a point in the given direction.
    /// Recursively spawns two more spoke draws at the endpoint
    fn recursive_spoke_draw(
        &self,
        mut color: ggez_gfx::Color,
        origin: ggez_mint::Point2<f32>,
        theta: f32,
        level: u32,
        mut input_mesh: &mut ggez_gfx::MeshBuilder
    ) {
        // Final level exit case
        if level == self.max_radial_distance {
            return;
        }

        let mut lines: [[ggez_mint::Point2<f32>; 2]; 3] = [[ggez_mint::Point2 {x: 0.0, y: 0.0}; 2]; 3];
        let mut endpoints: [ggez_mint::Point2<f32>; 3] = [ggez_mint::Point2 {x: 0.0, y: 0.0}; 3];

        // Calculate endpoint of stem
        endpoints[0] = ggez_mint::Point2 {
            x: origin.x + (GRID_CELL_SIZE * theta.cos()),
            y: origin.y - (GRID_CELL_SIZE * theta.sin())
        };
        lines[0] = [origin, endpoints[0]];

        // Calculate branch endpoints
        endpoints[1] = ggez_mint::Point2 {
            x: endpoints[0].x + (GRID_CELL_SIZE * (theta + PI/3.0).cos()),
            y: endpoints[0].y - (GRID_CELL_SIZE * (theta + PI/3.0).sin())
        };
        endpoints[2] = ggez_mint::Point2 {
            x: endpoints[0].x + (GRID_CELL_SIZE * (theta - PI/3.0).cos()),
            y: endpoints[0].y - (GRID_CELL_SIZE * (theta - PI/3.0).sin())
        };
        lines[1] = [endpoints[0], endpoints[1]];
        lines[2] = [endpoints[0], endpoints[2]];

        // Draw lines
        for i in 0..=2 {
            input_mesh = input_mesh.line(&lines[i], 1.0, WHITE).unwrap();
        }

        // Make the recursive calls
        color.g = color.g - 0.1;

        color.r = color.r + 0.1;
        self.recursive_spoke_draw(color, endpoints[1], theta, level+1, input_mesh);
        color.r = color.b + 0.1;
        self.recursive_spoke_draw(color, endpoints[2], theta, level+1, input_mesh);
    }
}