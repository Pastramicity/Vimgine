use bevy::prelude::*;

use crate::usr::sys;
use crate::usr::cmp;

pub fn sys_setup(app: &mut AppBuilder){
    app
        .add_startup_system(sys::hello_world::hello_world.system());
}

pub fn setup(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    c.spawn_bundle(PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Plane {size: 5.0})),
        material: materials.add(Color::rgb(0.3,0.5,0.3).into()),
        ..Default::default()
    }).insert(cmp::example_cmp::example_component);
}

