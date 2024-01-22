use bevy::prelude::*;
use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
    

}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) -> Entity {
    let main_menu_entity = commands.spawn(
        (
            NodeBundle {
                style: Style{
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                //background_color: Color::RED.into(),
                ..default()
            },
            MainMenu{},
        )
    )
    .with_children(|parent| {
        // Title 
        parent.spawn(
            ImageBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    ..default()
                },
                image: asset_server.load("robot_pixelart.png").into(),
                ..default()
            }
        );
        

        // Play Button 
        parent.spawn(
            (
                ButtonBundle{
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                PlayButton{},
            )
        )
        .with_children(|parent| {
            // Text 
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Play",
                                get_button_text_style(asset_server),
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                },
            );
        });


        // Quit Button
        parent.spawn(
            (
                ButtonBundle{
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                QuitButton{},
            )
        )
        .with_children(|parent| {
            // Text 
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Quit",
                                get_button_text_style(asset_server),
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                },
            );
        });

    })
    .id(); 

    main_menu_entity
}
