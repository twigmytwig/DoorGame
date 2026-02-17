use bevy::prelude::*;
use crate::hitbox::HitBox;
use crate::audio::play_sfx;
use crate::level_entity::LevelEntity;
use crate::player::PlayerHealth;
use crate::state::GameState;
use crate::hitbox::PlayerTouchedSomething;
use crate::npc::Npc;
use crate::story_flags::{StoryFlags, FlagValue};


#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec2,
}

fn handle_projectile_touch_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut messages: MessageReader<PlayerTouchedSomething>,
    projectiles: Query<(), With<Projectile>>,
    mut health: ResMut<PlayerHealth>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for message in messages.read() {
        // Try to get the LevelDoor component from the touched entity
        if projectiles.get(message.messaging_entity).is_ok() {
            info!("Projectile hit player!");
            //sound of projectile hit
            play_sfx(&mut commands, &asset_server, "player_hit", "mp3");

            commands.entity(message.messaging_entity).despawn();
            health.current -= 1;// TODO: HARD CODED
            if health.current == 0{
                next_state.set(GameState::Defeat)
            }
        }
    }
}

fn move_projectiles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Projectile)>,
) {
    for (mut transform, projectile) in &mut query {
        transform.translation.x += projectile.velocity.x * time.delta_secs();
        transform.translation.y += projectile.velocity.y * time.delta_secs();
    }
}

fn handle_projectile_touch_npc(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    projectiles: Query<(Entity, &Transform, &HitBox), With<Projectile>>,
    npcs: Query<(Entity, &Transform, &HitBox, &Npc)>,
    mut story_flags: ResMut<StoryFlags>,
) {
    for (proj_entity, proj_transform, proj_hitbox) in &projectiles {
        let proj_pos = proj_transform.translation;

        for (npc_entity, npc_transform, npc_hitbox, npc) in &npcs {
            let npc_pos = npc_transform.translation;

            // AABB overlap check
            let overlap_x = (proj_pos.x - npc_pos.x).abs() < (proj_hitbox.width + npc_hitbox.width) / 2.0;
            let overlap_y = (proj_pos.y - npc_pos.y).abs() < (proj_hitbox.height + npc_hitbox.height) / 2.0;

            if overlap_x && overlap_y {
                let name_lower = npc.name.to_lowercase();
                let health_key = format!("{}_health", name_lower);
                let present_key = format!("{}_present", name_lower);
                let status_key = format!("{}_status", name_lower);
                
                //TODO: FIND A BETTER WAY TO DO THIS
                if name_lower == "duck"{
                    play_sfx(&mut commands, &asset_server, "duck_quack", "mp3");
                }
                // Get current health, default to 1 if no flag exists
                let current_health = story_flags.get_number(&health_key).unwrap_or(1);
                let new_health = current_health - 1;

                info!("Projectile hit NPC '{}', health: {} -> {}", npc.name, current_health, new_health);

                // Despawn projectile
                commands.entity(proj_entity).despawn();

                if new_health <= 0 {
                    // NPC dies
                    info!("NPC '{}' has died!", npc.name);
                    story_flags.set(&present_key, FlagValue::Bool(false));
                    story_flags.set(&status_key, FlagValue::Text("died_in_boss".to_string()));
                    story_flags.set(&health_key, FlagValue::Number(0));
                    commands.entity(npc_entity).despawn();
                } else {
                    // Update health
                    story_flags.set(&health_key, FlagValue::Number(new_health));
                }

                break; // Projectile hit something, stop checking this projectile
            }
        }
    }
}

pub fn spawn_projectile_at(
    commands: &mut Commands,
    pos: Vec3,
    velocity: Vec2,
    shape: &str,
) {
    commands.spawn((
        Text2d::new(shape),
        TextFont { font_size: 24.0, ..default() },
        TextColor(Color::srgb(1.0, 0.3, 0.3)),
        Transform::from_translation(pos),
        Projectile { velocity },
        LevelEntity,
        HitBox { width: 24.0, height: 24.0 },
    ));
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            move_projectiles,
            handle_projectile_touch_player,
            handle_projectile_touch_npc,
        ).run_if(in_state(GameState::BossFight)));
    }
}
