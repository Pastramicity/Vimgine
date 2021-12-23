use bevy::prelude::*;

use bevy_config_cam::*;


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        // .insert_resource(Msaa{samples: 4})
        .insert_resource(WindowDescriptor{
            title: "Vimgine".to_string(),
            width: 3840.,
            height: 2160.,
            vsync: false,
            decorations: true,
            ..Default::default()
        })

        // startup systems
        .add_startup_system(setup.system())

        //runtime systems

        .run();
}

fn setup(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    c.spawn_bundle(PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Plane {size: 5.0})),
        material: materials.add(Color::rgb(0.3,0.5,0.3).into()),
        ..Default::default()
    });
}





