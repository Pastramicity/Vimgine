use bevy::prelude::*;

use bevy_config_cam::*;

mod usr;

fn main() {
    let mut app = App::build();
    app
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
        .add_startup_system(usr::setup::setup.system());
        usr::setup::sys_setup(&mut app);
        

        //runtime systems

        app.run();
}

