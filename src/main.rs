use bevy::prelude::*;








fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_config_cam::PlayerPlugin)
        // .insert_resource(Msaa{samples: 4})
        .insert_resource(WindowDescriptor{
            title: "Vimgine".to_string(),
            width: 3840.,
            height: 2160.,
            // vsync: false,
            decorations: true,
            ..Default::default()
        })
        .add_startup_system(setup.system())

        // .add_system(editor_cam_look.system())
        // .add_system(editor_cam_move.system())
        // .add_system(toggle_cursor.system())
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


