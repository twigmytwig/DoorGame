# State System

## States

```
Loading → StartGame → Playing ⇄ Paused
```

| State | Purpose |
|-------|---------|
| `Loading` | Initial state. Shows loading screen, waits for assets/timer. |
| `StartGame` | One-time initialization. Spawns player, doors, etc. Immediately transitions to Playing. |
| `Playing` | Active gameplay. Player can move, systems run. |
| `Paused` | Game frozen. Shows pause menu. Press Escape to resume. |

## Why StartGame Exists

Without `StartGame`, spawning on `OnEnter(Playing)` would cause duplicates:

```
Loading → Playing  (spawns entities)
Playing → Paused
Paused → Playing   (spawns again - duplicates!)
```

With `StartGame`:

```
Loading → StartGame → Playing  (spawns once in StartGame)
Playing → Paused
Paused → Playing               (no spawning, just resumes)
```

## Adding New Entities

To spawn entities at game start, use `OnEnter(GameState::StartGame)`:

```rust
use crate::state::GameState;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartGame), spawn_my_entity);
    }
}
```

## Running Systems Only During Gameplay

To run a system only while playing (not during loading/pause):

```rust
app.add_systems(Update, my_system.run_if(in_state(GameState::Playing)));
```

## State Transitions

| From | To | Trigger |
|------|----|---------|
| Loading | StartGame | LoadingTimer finishes (5 seconds) |
| StartGame | Playing | Automatic (immediate) |
| Playing | Paused | Press Escape |
| Paused | Playing | Press Escape |

## File Structure

```
src/state/
  mod.rs        - StatePlugin, timer, pause toggle
  game_state.rs - GameState enum definition
  loading.rs    - Loading screen spawn/despawn/animate
  pause.rs      - Pause menu spawn/despawn
```
