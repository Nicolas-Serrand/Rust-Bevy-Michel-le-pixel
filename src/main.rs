#![warn(unused_mut)]

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

static GRAVITY:f32 = -980.;

struct Michel{
    speed: f32,
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    jump_height: f32,
    is_jumping: bool,
}

struct Wall;

fn main() {
    App::build()
    .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
    .insert_resource(WindowDescriptor{
        title: "Michel le Pixel".to_string(),
        // width: 500.0,
        // height: 500.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .add_system(michel_gravity_system.system())
    .add_system(michel_collision_system.system())
    .add_system(michel_movement_system.system())
    .add_system(name_position_system.system())
    .run();
}


fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    windows: ResMut<Windows>,
){
    //camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let michel = Michel {speed: 200.,
        position: Vec2::new(0., 0.),
        velocity: Vec2::new(0., 0.), 
        acceleration: Vec2::new(0., 0.),
        jump_height: 400.,
        is_jumping: false,
    };

    let michel_color = Color::rgb(0.0, 0.5, 0.0);
    //spawn Michel le pixel
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(michel_color.into()),
        sprite: Sprite::new(Vec2::new(20.0, 20.0)),
        ..Default::default()
    }).insert(michel);


    //spawn Ground
    let window = windows.get_primary().unwrap();
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
        sprite: Sprite::new(Vec2::new(window.width(), 100.0)),
        transform: Transform::from_xyz(0.0,  - window.height() / 2.0 + 50.0, 0.0),
        ..Default::default()
    }).insert(Wall);

    //spawn Obstacle
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
        sprite: Sprite::new(Vec2::new(50.0, 200.0)),
        transform: Transform::from_xyz(0.0,  - window.height() / 2.0 + 50.0, 0.0),
        ..Default::default()
    }).insert(Wall);

    //spawn Michel Name
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            "Michel",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.,
                color: michel_color,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    });
}

fn michel_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    windows: ResMut<Windows>,
    mut query: Query<(&mut Michel, &mut Transform)>,
) {
    if let Ok((mut michel, mut transform)) = query.single_mut() {
        
        let time_delta = time.delta_seconds();

        let mut deplacement: f32 = 0.;

        if keyboard_input.pressed(KeyCode::Left) {
            deplacement -= michel.speed;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            deplacement += michel.speed;
        }
        if keyboard_input.pressed(KeyCode::Down) {//Add something maybe latter
        }

        if keyboard_input.pressed(KeyCode::Up) && !michel.is_jumping {
            michel.velocity.y = michel.jump_height;
            michel.is_jumping = true;
        }

        michel.velocity.x += time_delta * michel.acceleration.x;
        michel.velocity.y += time_delta * michel.acceleration.y;
        michel.position.x += (michel.velocity.x + deplacement) * time_delta + michel.acceleration.x * time_delta.powi(2) / 2.0;
        michel.position.y += time_delta * michel.velocity.y + michel.acceleration.y * time_delta.powi(2) / 2.0;

        let window = windows.get_primary().unwrap();
        
        // TP Michel if he go out of the window
        michel.position.x = (michel.position.x + window.width() / 2.0).rem_euclid(window.width()) - window.width() / 2.0;
        michel.position.y = (michel.position.y + window.height() / 2.0).rem_euclid(window.height()) - window.height() / 2.0;

        transform.translation.x = michel.position.x;
        transform.translation.y = michel.position.y;


    }
}

fn name_position_system(
    mut query_text: Query<&mut Transform, With<Text>>,
    mut query_michel: Query<&Michel>,
){
    if let Ok(mut text_transform) =  query_text.single_mut(){
        if let Ok(michel) = query_michel.single(){
            text_transform.translation.x = michel.position.x;
            text_transform.translation.y = michel.position.y + 25.;
        }
    }
}

fn michel_gravity_system(
    mut michel_query:  Query<&mut Michel>,
) {
    if let Ok(mut michel) = michel_query.single_mut() {
        michel.acceleration.y = GRAVITY;
    }
}


fn michel_collision_system(
    mut michel_query: Query<(&mut Michel, &Transform, &Sprite)>,
    wall_query: Query<(&Wall, &Transform, &Sprite)>,
) {


    if let Ok((mut michel, michel_transform, michel_sprite)) = michel_query.single_mut() {

        for (_wall, wall_transform, wall_sprite) in wall_query.iter() {
            let collision = collide(
                michel_transform.translation,
                michel_sprite.size,
                wall_transform.translation,
                wall_sprite.size,
            );

            if let Some(collision) = collision {
                // only reflect if michel's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left =>  {   michel.velocity.x = - michel.velocity.x * 0.3;
                                            michel.acceleration.x = 0.;
                                            michel.position.x = wall_transform.translation.x - wall_sprite.size.x / 2. - michel_sprite.size.x / 2.;
                                    },
                    Collision::Right => {   michel.velocity.x = - michel.velocity.x * 0.3;
                                            michel.acceleration.x = 0.;
                                            michel.position.x = wall_transform.translation.x + wall_sprite.size.x / 2. + michel_sprite.size.x / 2.;
                                        },
                    Collision::Top =>   {   michel.velocity.y = - michel.velocity.y * 0.3;
                                            michel.acceleration.y = 0.;
                                            michel.position.y = wall_transform.translation.y + wall_sprite.size.y / 2. + michel_sprite.size.y / 2.;
                                            michel.is_jumping = false;
                                        },
                    Collision::Bottom => {  michel.velocity.y = - michel.velocity.y * 0.3;
                                            michel.acceleration.y = 0.;
                                            michel.position.y = wall_transform.translation.y - wall_sprite.size.y / 2. - michel_sprite.size.y / 2.;
                                        },
                }
                

                //break;
            }
        }
    }
}