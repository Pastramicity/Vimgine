use bevy::prelude::*;

#[macro_use]
extern crate bevy_discovery;

struct EditorCam;
const EDITOR_MOVE_SPEED: f32 = 5.;
const EDITOR_FAST_MOVE_MUL: f32 = 3.;
const EDITOR_LOOK_Z_SPEED: f32 = 2.;
const EDITOR_LOOK_V_SPEED: f32 = 1.5;



fn main() {
    App::build()
        .insert_resource(Msaa{samples: 4})
        .insert_resource(WindowDescriptor{
            title: "Vimgine".to_string(),
            width: 3840.,
            height: 2160.,
            vsync: false,
            decorations: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DiscoveryPlugin)
        .add_startup_system(setup.system())
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
    c.spawn()
        .insert_bundle(PerspectiveCameraBundle{
            transform: Transform::from_xyz(-2.0, 2.5, 5.0),
            ..Default::default()
        })
        .insert(EditorCam);
}

/// This system toggles the cursor's visibility when the space bar is pressed
#[system]
fn toggle_cursor(keyboard: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if keyboard.just_pressed(KeyCode::RAlt) {
        window.set_cursor_lock_mode(!window.cursor_locked());
        window.set_cursor_visibility(!window.cursor_visible());
    }
}

#[system]
fn editor_cam_move(
    q: Query<&mut Transform, With<EditorCam>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
){
    q.for_each_mut(|mut t|{
        // get keyboard from user and get movement amount in that direction
        let dir_x:f32= 
            if keyboard.pressed(KeyCode::D) {1.} else {0.} +
            if keyboard.pressed(KeyCode::A) {-1.} else {0.};
        let inc_x = dir_x * (EDITOR_MOVE_SPEED * time.delta_seconds());

        let dir_z:f32= 
            if keyboard.pressed(KeyCode::S) {1.} else {0.} +
            if keyboard.pressed(KeyCode::W) {-1.} else {0.}; 
        let inc_z = dir_z * (EDITOR_MOVE_SPEED * time.delta_seconds());

        let speed_mul = if keyboard.pressed(KeyCode::LShift) {EDITOR_FAST_MOVE_MUL} else {1.};

        // move by amount specified
        let forward = t.rotation.mul_vec3(Vec3::Z);
        t.translation += forward * inc_z * speed_mul;
        let right = t.rotation.mul_vec3(Vec3::X);
        t.translation += right * inc_x * speed_mul;
    });
}

// needs some work, mouse support
#[system]
fn editor_cam_look(
    q: Query<&mut Transform, With<EditorCam>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
){
    q.for_each_mut(|mut t|{
        //change left/right looking, the up/down
        let dy = (if keyboard.pressed(KeyCode::L) {-EDITOR_LOOK_Z_SPEED} else {0.}+ 
              if keyboard.pressed(KeyCode::H) {EDITOR_LOOK_Z_SPEED} else {0.})
              * time.delta_seconds();
        
        let rot_y = Quat::from_rotation_y(dy);
        t.rotation *= rot_y;
        //rotate vertically (with clamping)
        let pi_half = std::f32::consts::FRAC_PI_2;
        let dv = (if keyboard.pressed(KeyCode::K) {EDITOR_LOOK_V_SPEED} else {0.}+
                   if keyboard.pressed(KeyCode::J){-EDITOR_LOOK_V_SPEED} else {0.})
                   * time.delta_seconds();
        let rot_v = Quat::from_axis_angle(t.rotation.mul_vec3(Vec3::X), dv);
        
        //dont rotate if rotation is past pi_half either direction
        let mut test_rot = t.clone();
        test_rot.rotate(rot_v);
        let test_v = test_rot.rotation.mul_vec3(Vec3::X).length();
        if test_v > pi_half || test_v < -pi_half{
            return;
        }

        //rotate
        t.rotate(rot_v);
        

    });
}




#[derive(DiscoveryPlugin)]
struct DiscoveryPlugin;

