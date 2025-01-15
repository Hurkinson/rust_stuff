use bevy::prelude::*; 

#[derive(Component)]
pub struct Player {
    pub speed:f32,
}

#[derive(Component)]
pub struct SilverCoinsStack {
    pub lifetime: Timer
}

#[derive(Resource)]
pub struct Mana(pub f32);

// pub enum ScalingMode {
//     Fixed {
//         width: f32,
//         height: f32,
//     },
//     WindowSize(f32),
//     AutoMin {
//         min_width: f32,
//         min_height: f32,
//     },
//     AutoMax {
//         max_width: f32,
//         max_height: f32,
//     },
//     FixedVertical(f32),
//     FixedHorizontal(f32),
// }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin{
                primary_window: Some(Window {
                    title: "AAA FancyRPG".into(),
                    resolution: (800.0,600.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .build(),
        )  
        .insert_resource(Mana(100.0))  
        .add_systems(Startup, setup)
        .add_systems(Update, (character_movement, spawn_coins, coins_lifetime))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = bevy::render::camera::ScalingMode::AutoMin {
        min_width: 256.0, 
        min_height: 144.0,
    };

    commands.spawn(camera);

    let texture = asset_server.load("character.png");

    commands
        .spawn(SpriteBundle {
            texture,
            ..default()
        })
        .insert(Player { speed: 200.0});
    }

fn spawn_coins(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut mana: ResMut<Mana>,
    player: Query<&Transform, With<Player>>
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();

    if mana.0 >= 10.0 {
        mana.0 -= 10.0;
        info!("tu jette un sort 'd'or des fous' ! , celà te coute : {:?} points de mana", mana.0);

        let texture = asset_server.load("silver_coins.png");

        commands.spawn((
            SpriteBundle {
                texture,
                transform: *player_transform,
                ..default()
            },
            SilverCoinsStack {
                lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            }
        ));
    }
}

fn coins_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut silvercoins: Query<(Entity, &mut SilverCoinsStack)>,
    mut mana: ResMut<Mana>,
) {
    for (silvercoins_ent, mut silv_coins) in &mut silvercoins {
        silv_coins.lifetime.tick(time.delta());

        if silv_coins.lifetime.finished() {
            mana.0 += 15.0;

            commands.entity(silvercoins_ent).despawn();

            info!("l'illusion se dissipe, tu recupère 15 points de mana. tu disposes de {:?} points de mana !" , mana.0)
        }
    }
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for(mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();

        // qwerty keys input

        //forward
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += movement_amount;
        }
        // backward
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= movement_amount;
        }
        //right
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x += movement_amount;
        }
        //left
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= movement_amount;
        }
    }
}