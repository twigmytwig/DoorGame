use bevy::prelude::*;

#[derive(Component)]
pub struct DefeatMenu;

pub fn spawn_defeat_menu(mut commands: Commands) {
    commands.spawn((
        DefeatMenu,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("DEFEATED\n\nPress ESC to restart"),
            TextFont {
                font_size: 36.0,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(Justify::Center),
        ));
    });
    
    info!("Defeat menu spawned");
}

pub fn despawn_defeat_menu(
    mut commands: Commands,
    query: Query<Entity, With<DefeatMenu>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    
    info!("Defeat menu despawned");
}