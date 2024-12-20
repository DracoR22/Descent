use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{common::entities::EntityType, cubes::components::CubeComponent};

pub fn spawn_cube_system(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>, mut material_assets: ResMut<Assets<StandardMaterial>>,  asset_server: Res<AssetServer>,) {
    // Define the size of the cube
    let width = 50.0; // Width
    let height = 50.0; // Height
    let depth = 50.0; // Depth

    // Create the cube mesh
    let mesh = mesh_assets.add(Cuboid::new(width, height,depth)); 

    let material = material_assets.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_translation(Vec3::new(400.0, 70.0, 0.0)), 
        ..default()
    })
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(width / 2.0, height / 2.0, depth / 2.0)) 
    .insert(CubeComponent)
    .insert(EntityType::Cube)
    .insert(GravityScale(70.0));

}