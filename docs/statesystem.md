# State System

## States

```
Loading → LoadingNewLevel → Dialogue → Playing ⇄ Paused
                │              │
                │              └→ BossFight → Defeat
                │                     ↓
                └─────────────────────┘ (restart)
```

| State | Purpose |
|-------|---------|
| `Loading` | Initial state. Shows loading screen, waits for timer. |
| `LoadingNewLevel` | Loads level RON, spawns level entities, transitions based on level type. |
| `Dialogue` | Shows dialogue panel. Press Space/Enter to advance. |
| `Playing` | Active exploration gameplay. Player can move, interact with doors. |
| `Paused` | Game frozen. Shows pause menu. Press Escape to resume. |
| `BossFight` | Boss battle. Player confined to arena, projectiles fire at player. |
| `Defeat` | Game over screen. Press Escape to restart from level 1. |

## State Transitions

| From | To | Trigger |
|------|----|---------|
| Loading | LoadingNewLevel | LoadingTimer finishes (1 second) |
| LoadingNewLevel | Dialogue | Level has dialogue |
| LoadingNewLevel | Playing | Normal level, no dialogue |
| LoadingNewLevel | BossFight | Boss level (`room_type: "boss"`), no dialogue |
| Dialogue | Playing | Dialogue exhausted, normal level |
| Dialogue | BossFight | Dialogue exhausted, boss level |
| Playing | Paused | Press Escape |
| Playing | LoadingNewLevel | Touch door |
| Paused | Playing | Press Escape |
| BossFight | Defeat | Player health reaches 0 |
| Defeat | LoadingNewLevel | Press Escape (resets health + level) |

## Running Systems in Specific States

```rust
// Single state
app.add_systems(Update, my_system.run_if(in_state(GameState::Playing)));

// Multiple states
app.add_systems(Update, my_system.run_if(
    in_state(GameState::Playing).or(in_state(GameState::BossFight))
));
```

## OnEnter / OnExit

```rust
// Runs once when entering a state
app.add_systems(OnEnter(GameState::BossFight), spawn_boss_arena);

// Runs once when exiting a state
app.add_systems(OnExit(GameState::BossFight), cleanup_boss);
```

## File Structure

```
src/state/
  mod.rs              - StatePlugin, timer, pause/restart toggle
  game_state.rs       - GameState enum definition
  loading.rs          - Loading screen spawn/despawn/animate
  loading_new_level.rs - Level loading, entity spawning
  dialogue.rs         - Dialogue panel and advancement
  pause.rs            - Pause menu spawn/despawn
  boss_fight.rs       - Boss arena, attack timer, projectile spawning
  defeat.rs           - Defeat screen spawn/despawn
```

## Key Resources

| Resource | Purpose |
|----------|---------|
| `CurrentLevel` | Tracks level_id, asset handle, loaded status |
| `LoadedLevelData` | Stores parsed LevelData for current level |
| `DialogueState` | Tracks current dialogue line index |
| `PlayerHealth` | Player's current and max health |
| `AttackTimer` | Boss fight projectile spawning timer |
