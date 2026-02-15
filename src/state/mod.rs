use bevy::prelude::*;

#[derive(Resource)]
struct LoadingTimer(Timer);

mod game_state;
mod loading;
mod pause;
mod loading_new_level;
mod dialogue;

pub use game_state::GameState;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
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

            // StartGame: level.rs handles asset loading and transitions to Playing/Dialogue

            // Dialogue state systems
            .add_systems(OnEnter(GameState::Dialogue), (
                dialogue::reset_dialogue_state,
                dialogue::spawn_dialogue_panel,
            ))
            .add_systems(Update, dialogue::advance_dialogue.run_if(in_state(GameState::Dialogue)))
            .add_systems(OnExit(GameState::Dialogue), dialogue::despawn_dialogue_panel)

            // LoadingNewLevel state systems
            .add_systems(OnEnter(GameState::LoadingNewLevel), (loading_new_level::spawn_loading_new_level_screen, loading_new_level::despawn_level_entities))
            .add_systems(Update, loading_new_level::animate_loading_room.run_if(in_state(GameState::LoadingNewLevel)))
            .add_systems(OnExit(GameState::LoadingNewLevel), loading::despawn_loading_screen)

            // Pause state systems
            .add_systems(OnEnter(GameState::Paused), pause::spawn_pause_menu)
            .add_systems(OnExit(GameState::Paused), pause::despawn_pause_menu)
            
            // Pause toggle (works in Playing or Paused states)
            .add_systems(Update, 
                toggle_pause.run_if(in_state(GameState::Playing).or(in_state(GameState::Paused)))
            );
    }
}

fn check_assets_loaded(
    time: Res<Time>,
    mut timer: ResMut<LoadingTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    timer.0.tick(time.delta());

    if timer.0.is_finished() {
        next_state.set(GameState::StartGame);
    }
}

fn toggle_pause(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
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
            _ => {}
        }
    }
}