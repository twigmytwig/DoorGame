//on enter system -> spawn arena that confines player -> spawn boss -> start attacks some how
use bevy::prelude::*;
use rand::Rng;
use crate::art::SCARY_DOOR_ART;
use crate::level_entity::LevelEntity;
use crate::level::spawn_wall_at;
use crate::player::Player;
use crate::projectile::spawn_projectile_at;
use crate::level_event::LevelEvent;
use crate::story_flags::{FlagValue, StoryFlags};

const WALL_SIZE: f32 = 32.0;

#[derive(Component)]
pub struct PlayerArena {
    pub height: f32,
    pub width: f32,
}

#[derive(Component)]
pub struct Boss;

#[derive(Resource)]
pub struct AttackTimer {
    pub timer: Timer,
    pub projectiles_fired: u32,
    pub max_projectiles: u32,
    pub event_sent: bool,
}

impl Default for AttackTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            projectiles_fired: 0,
            max_projectiles: 15,
            event_sent: false,
        }
    }
}

/// Tracks whether the boss fight has been initialized (to avoid re-running setup on re-entry from dialogue)
#[derive(Resource, Default)]
pub struct BossFightInitialized(pub bool);

pub fn reset_attack_timer(mut commands: Commands, initialized: Res<BossFightInitialized>) {
    if initialized.0 {
        info!("Boss fight already initialized, skipping attack timer reset");
        return;
    }
    commands.insert_resource(AttackTimer::default());
}

pub fn fire_projectiles_at_player(
    mut commands: Commands,
    time: Res<Time>,
    mut attack_timer: ResMut<AttackTimer>,
    player_query: Query<&Transform, With<Player>>,
    boss_query: Query<&Transform, With<Boss>>,
    mut event_writer: MessageWriter<LevelEvent>,
) {
    // Don't fire if we've reached max - send event if not already sent
    if attack_timer.projectiles_fired >= attack_timer.max_projectiles {
        if !attack_timer.event_sent {
            info!("All projectiles fired, sending ProjectilesDone event");
            event_writer.write(LevelEvent::ProjectilesDone);
            attack_timer.event_sent = true;
        }
        return;
    }

    attack_timer.timer.tick(time.delta());

    if attack_timer.timer.just_finished() {
        let Ok(player_transform) = player_query.single() else { return };
        let Ok(boss_transform) = boss_query.single() else { return };

        let mut rng = rand::rng();

        // Random spawn position near the boss
        let spawn_offset_x = rng.random_range(-100.0..100.0);
        let spawn_pos = Vec3::new(
            boss_transform.translation.x + spawn_offset_x,
            boss_transform.translation.y,
            5.0,
        );

        // Direction toward player
        let direction = Vec2::new(
            player_transform.translation.x - spawn_pos.x,
            player_transform.translation.y - spawn_pos.y,
        ).normalize();

        // Random speed
        let speed = rng.random_range(150.0..300.0);
        let velocity = direction * speed;

        spawn_projectile_at(&mut commands, spawn_pos, velocity, "{=}");

        attack_timer.projectiles_fired += 1;
        info!("Fired projectile {}/{}", attack_timer.projectiles_fired, attack_timer.max_projectiles);
    }
}

pub fn spawn_boss_arena(
    mut commands: Commands,
    mut story_flags: ResMut<StoryFlags>,
    mut initialized: ResMut<BossFightInitialized>,
) {
    if initialized.0 {
        info!("Boss fight already initialized, skipping arena spawn");
        return;
    }

    // Mark as initialized
    initialized.0 = true;

    // Initialize boss_phase to 1 at start of fight
    story_flags.set("boss_phase", FlagValue::Number(1));
    let arena_width = 300.0;
    let arena_height = 200.0;
    let arena_x = 0.0;
    let arena_y = -100.0; // Lower part of screen

    // Spawn arena data (for collision bounds)
    commands.spawn((
        PlayerArena { width: arena_width, height: arena_height },
        Transform::from_translation(Vec3::new(arena_x, arena_y, 0.0)),
        LevelEntity,
    ));

    // Calculate arena bounds
    let left = arena_x - arena_width / 2.0;
    let right = arena_x + arena_width / 2.0;
    let top = arena_y + arena_height / 2.0;
    let bottom = arena_y - arena_height / 2.0;

    // Spawn arena walls
    let cols = (arena_width / WALL_SIZE).ceil() as i32;
    let rows = (arena_height / WALL_SIZE).ceil() as i32;

    // Top and bottom walls
    for i in 0..=(cols-1) {
        let x = left + i as f32 * WALL_SIZE;
        spawn_wall_at(&mut commands, Vec3::new(x, top, 0.0), WALL_SIZE);
        spawn_wall_at(&mut commands, Vec3::new(x, bottom, 0.0), WALL_SIZE);
    }

    // Left and right walls
    for i in 1..rows {
        let y = bottom + i as f32 * WALL_SIZE;
        spawn_wall_at(&mut commands, Vec3::new(left, y, 0.0), WALL_SIZE);
        spawn_wall_at(&mut commands, Vec3::new(right, y, 0.0), WALL_SIZE);
    }

    // Spawn boss art above arena
    commands.spawn((
        Text2d::new(SCARY_DOOR_ART),
        TextFont { font_size: 16.0, ..default() },
        TextColor(Color::srgb(0.8, 0.2, 0.2)),
        Transform::from_translation(Vec3::new(arena_x, top + 200.0, 1.0)),
        Boss,
        LevelEntity,
    ));
}

pub fn reset_boss_fight_initialized(mut initialized: ResMut<BossFightInitialized>) {
    initialized.0 = false;
}