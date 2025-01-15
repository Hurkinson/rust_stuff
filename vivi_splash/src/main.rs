// use std::time::Duration;

// use bevy::prelude::*;
// use bevy_splash_screen::{SplashAssetType, SplashItem, SplashPlugin, SplashScreen};
// use bevy_tweening::EaseFunction;

// #[derive(Clone, Copy, Debug, Default, States, Hash, PartialEq, Eq)]
// enum ScreenStates {
//     #[default]
//     Splash,
//     Menu,
// }

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .init_state::<ScreenStates>()
//         .add_plugins(
//             SplashPlugin::new(ScreenStates::Splash, ScreenStates::Menu)
//                 .add_screen(SplashScreen {
//                     brands: vec![SplashItem {
//                         asset: SplashAssetType::SingleText(
//                             Text::from_sections([
//                                 TextSection::new(
//                                     "The Meupong Project\n",
//                                     TextStyle {
//                                         font_size: 40.,
//                                         color: Color::WHITE,
//                                         ..default()
//                                     },
//                                 ),
//                                 TextSection::new(
//                                     "by\n",
//                                     TextStyle {
//                                         font_size: 24.,
//                                         color: Color::WHITE.with_a(0.75),
//                                         ..default()
//                                     },
//                                 ),
//                                 TextSection::new(
//                                     "Wild Pixo production",
//                                     TextStyle {
//                                         font_size: 32.,
//                                         color: Color::WHITE,
//                                         ..default()
//                                     },
//                                 ),
//                             ])
//                             .with_justify(JustifyText::Center),
//                             "FiraSans-Bold.ttf".to_string(),
//                         ),
//                         tint: Color::SEA_GREEN,
//                         width: Val::Percent(30.),
//                         height: Val::Px(150.),
//                         ease_function: EaseFunction::QuarticInOut.into(),
//                         duration: Duration::from_secs_f32(5.),
//                         is_static: false,
//                     }],
//                     background_color: BackgroundColor(Color::BLACK),
//                     ..default()
//                 })
//                 .add_screen(SplashScreen {
//                     brands: vec![SplashItem {
//                         asset: SplashAssetType::SingleText(
//                             Text::from_sections([TextSection::new(
//                                 "With Bevy Engine",
//                                 TextStyle {
//                                     font_size: 32.,
//                                     color: Color::WHITE,
//                                     ..default()
//                                 },
//                             )])
//                             .with_justify(JustifyText::Center),
//                             "FiraSans-Bold.ttf".to_string(),
//                         ),
//                         tint: Color::WHITE,
//                         width: Val::Percent(30.),
//                         height: Val::Px(150.),
//                         ease_function: EaseFunction::QuarticInOut.into(),
//                         duration: Duration::from_secs_f32(5.),
//                         is_static: false,
//                     }],
//                     wait_to_start: bevy_splash_screen::WaitScreenType::AfterEnd,
//                     background_color: BackgroundColor(Color::BLACK),
//                     ..default()
//                 })
//                 .add_screen(SplashScreen {
//                     brands: vec![SplashItem {
//                         asset: SplashAssetType::SingleText(
//                             Text::from_sections([TextSection::new(
//                                 "With Love <3",
//                                 TextStyle {
//                                     font_size: 32.,
//                                     color: Color::WHITE,
//                                     ..default()
//                                 },
//                             )])
//                             .with_justify(JustifyText::Center),
//                             "FiraSans-Bold.ttf".to_string(),
//                         ),
//                         tint: Color::RED,
//                         width: Val::Percent(30.),
//                         height: Val::Px(150.),
//                         ease_function: EaseFunction::QuarticInOut.into(),
//                         duration: Duration::from_secs_f32(5.),
//                         is_static: false,
//                     }],
//                     wait_to_start: bevy_splash_screen::WaitScreenType::AfterEnd,
//                     background_color: BackgroundColor(Color::WHITE),
//                     ..default()
//                 }),
//         )
//         .add_systems(Startup, create_scene)
//         .run()
// }

// fn create_scene(mut cmd: Commands) {
//     cmd.spawn(Camera2dBundle::default());
// }


use std::time::Duration;

use bevy::prelude::*;
use bevy_splash_screen::{SplashAssetType, SplashItem, SplashPlugin, SplashScreen};
use bevy_tweening::EaseFunction;

#[derive(Clone, Copy, Debug, Default, States, Hash, PartialEq, Eq)]
enum ScreenStates {
    #[default]
    Splash,
    Menu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<ScreenStates>()
        .add_plugins(
            SplashPlugin::new(ScreenStates::Splash, ScreenStates::Menu)
                .add_screen(SplashScreen {
                    brands: vec![SplashItem {
                        asset: SplashAssetType::SingleText(
                            Text::from_sections([
                                TextSection::new(
                                    "The Meupong Project\n",
                                    TextStyle {
                                        font_size: 40.,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ),
                                TextSection::new(
                                    "by\n",
                                    TextStyle {
                                        font_size: 24.,
                                        color: Color::WHITE.with_a(0.75),
                                        ..default()
                                    },
                                ),
                                TextSection::new(
                                    "Wild Pixo production",
                                    TextStyle {
                                        font_size: 32.,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ),
                            ])
                            .with_justify(JustifyText::Center),
                            "FiraSans-Bold.ttf".to_string(),
                        ),
                        tint: Color::SEA_GREEN,
                        width: Val::Percent(30.),
                        height: Val::Px(150.),
                        ease_function: EaseFunction::QuarticInOut.into(),
                        duration: Duration::from_secs_f32(5.0),
                        is_static: false,
                    }],
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                })
                .add_screen(SplashScreen {
                    brands: vec![SplashItem {
                        asset: SplashAssetType::SingleText(
                            Text::from_sections([TextSection::new(
                                "With Bevy Engine",
                                TextStyle {
                                    font_size: 32.,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            )])
                            .with_justify(JustifyText::Center),
                            "FiraSans-Bold.ttf".to_string(),
                        ),
                        tint: Color::WHITE,
                        width: Val::Percent(30.),
                        height: Val::Px(150.),
                        ease_function: EaseFunction::QuarticInOut.into(),
                        duration: Duration::from_secs_f32(5.0),
                        is_static: false,
                    }],
                    wait_to_start: bevy_splash_screen::WaitScreenType::AfterEnd,
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                })
                .add_screen(SplashScreen {
                    brands: vec![SplashItem {
                        asset: SplashAssetType::SingleText(
                            Text::from_sections([TextSection::new(
                                "With Love <3",
                                TextStyle {
                                    font_size: 32.,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            )])
                            .with_justify(JustifyText::Center),
                            "FiraSans-Bold.ttf".to_string(),
                        ),
                        tint: Color::RED,
                        width: Val::Percent(30.),
                        height: Val::Px(150.),
                        ease_function: EaseFunction::QuarticInOut.into(),
                        duration: Duration::from_secs_f32(5.0),
                        is_static: false,
                    }],
                    wait_to_start: bevy_splash_screen::WaitScreenType::AfterEnd,
                    background_color: BackgroundColor(Color::WHITE),
                    ..default()
                }),
        )
        .add_systems(Startup, create_scene)
        .run()
}

fn create_scene(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}
