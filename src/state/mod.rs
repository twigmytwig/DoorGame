use bevy::prelude::*;

#[derive(Resource)]
struct LoadingTimer(Timer);

mod game_state;
mod loading;
mod pause;
mod loading_new_level;
mod dialogue;
mod boss_fight;
mod defeat;

pub use game_state::GameState;

use crate::player::PlayerHealth;
use crate::ui;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerHealth {current: 3, max: 3})
            .insert_resource(LoadingTimer(Timer::from_seconds(1.0, TimerMode::Once)))
            .init_resource::<dialogue::DialogueState>()
            .init_state::<GameState>()
            
            // Loading state systems
            .add_systems(OnEnter(GameState::Loading), loading::spawn_loading_screen)
            .add_systems(Update, (
                check_assets_loaded,
                loading::animate_loading,
            ).run_if(in_state(GameState::Loading)))
            .add_systems(OnExit(GameState::Loading),
                loading::despawn_loading_screen)

            // Dialogue state systems
            .add_systems(OnEnter(GameState::Dialogue), (
                dialogue::reset_dialogue_state,
                dialogue::spawn_dialogue_panel,
                ui::spawn_health_ui
            ))
            .add_systems(Update, dialogue::advance_dialogue.run_if(in_state(GameState::Dialogue)))
            .add_systems(OnExit(GameState::Dialogue), dialogue::despawn_dialogue_panel)

            // LoadingNewLevel state systems
            .add_systems(OnEnter(GameState::LoadingNewLevel), (
                loading_new_level::spawn_loading_new_level_screen,
                loading_new_level::despawn_level_entities,
                loading_new_level::start_loading_next_level,
            ))
            .add_systems(Update, (
                loading_new_level::animate_loading_room,
                loading_new_level::check_new_level_ready,
            ).run_if(in_state(GameState::LoadingNewLevel)))
            .add_systems(OnExit(GameState::LoadingNewLevel), loading::despawn_loading_screen)

            // Pause state systems
            .add_systems(OnEnter(GameState::Paused), pause::spawn_pause_menu)
            .add_systems(OnExit(GameState::Paused), pause::despawn_pause_menu)

            // defeat state systems
            .add_systems(OnEnter(GameState::Defeat), defeat::spawn_defeat_menu)
            .add_systems(OnExit(GameState::Defeat), defeat::despawn_defeat_menu)
            
            // Pause toggle (works in Playing, Paused, or Defeat states)
            .add_systems(Update,
                toggle_pause.run_if(
                    in_state(GameState::Playing)
                        .or(in_state(GameState::Paused))
                        .or(in_state(GameState::Defeat))
                )
            )

            // BossFight state systems
            .add_systems(OnEnter(GameState::BossFight), (
                boss_fight::spawn_boss_arena,
                boss_fight::reset_attack_timer,
                ui::spawn_health_ui,
            ))
            .add_systems(Update, boss_fight::fire_projectiles_at_player.run_if(in_state(GameState::BossFight)))

            // Health UI systems
            .add_systems(Update, ui::update_health_ui.run_if(resource_changed::<PlayerHealth>))
            .add_systems(OnEnter(GameState::LoadingNewLevel), ui::despawn_health_ui)
            .add_systems(OnEnter(GameState::Defeat), ui::despawn_health_ui);
    }
}

fn check_assets_loaded(
    time: Res<Time>,
    mut timer: ResMut<LoadingTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    timer.0.tick(time.delta());

    if timer.0.is_finished() {
        next_state.set(GameState::LoadingNewLevel);
    }
}

fn toggle_pause( // ALSO HANDLES RESTARTING GAME TODO: DONT PUT RESTART LOGIC IN HERE
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut player_health: ResMut<PlayerHealth>,
    mut current_level: ResMut<crate::level::CurrentLevel>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => {
                info!("Game paused");
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                info!("Game resumed");
                next_state.set(GameState::Playing);
            }
            GameState::Defeat => {
                info!("Game restarted");
                // Reset player health
                player_health.current = player_health.max;
                // Reset to first level
                current_level.level_id = "level_01_intro".to_string();
                current_level.loaded = false;
                // Go to LoadingNewLevel to reload
                next_state.set(GameState::LoadingNewLevel);
            }
            _ => {}
        }
    }
}