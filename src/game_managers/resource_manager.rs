/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : game_managers/resource_manager.rs

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
    This module manages all active resources in the game, as well as providing
    Utility Methods for resource drawing, moving, etc.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use cast_iron::{
    element::Elemental,
    logger,
    mechanics::resource::Resource,
    Plottable,
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
// Data structures
///////////////////////////////////////////////////////////////////////////////

//TODO: Proper implementation of an error type
#[derive(Debug)]
pub struct ResourceError;

pub struct ResourceManager {
    logger:         logger::Instance,
    resources:      Vec<Resource>,
    resource_mesh:  ggez_gfx::Mesh,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl ResourceManager {
    /// Generic Constructor - creates an empty instance
    pub fn new(logger_original: &logger::Instance, ctx: &mut GgEzContext) -> Self {
        // Clone the logger instance so this module has its own sender to use
        let logger_clone = logger_original.clone();

        ResourceManager {
            logger:         logger_clone,
            resources:      Vec::new(),
            resource_mesh:  ggez_gfx::Mesh::new_line(
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

impl DrawableMechanic for ResourceManager {
    type Instance = Resource;
    type ErrorType = ResourceError;

    fn instances(&self) -> &Vec<Self::Instance> {
        &self.resources
    }

    fn push_instance(&mut self, instance: Self::Instance) {
        ci_log!(self.logger,
            logger::FilterLevel::Debug,
            "Adding {} resource starting at {} to mesh.",
            String::from(instance.element()),
            instance.origin());

        self.resources.push(instance);
    }

    fn mesh(&self) -> &ggez_gfx::Mesh {
        &self.resource_mesh
    }

    fn set_mesh(&mut self, mesh: ggez_gfx::Mesh) {
        self.resource_mesh = mesh;
    }

    fn add_instance_to_mesh_builder(instance: &Self::Instance,
                                    mesh_builder: &mut ggez_gfx::MeshBuilder,
                                    ggez_ctx: &mut GgEzContext) -> Result<(), Self::ErrorType> {
        // Create a HexGridCell object and add it to the mesh builder
        let cur_hex = HexGridCell::new_from_hex_coords(instance.origin(), ::HEX_RADIUS_VERTEX, ggez_ctx);
        cur_hex.add_to_mesh(colors::from_resource(instance), colors::WHITE, mesh_builder);

        // Create radial HexGridCells as necessary
        cur_hex.add_radials_to_mesh(
            colors::from_resource(instance),
            colors::WHITE,
            instance.radius(),
            true,
            mesh_builder);

        Ok(())
    }
}