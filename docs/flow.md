# Game Flow Documentation

## Overview

This is an ASCII-based door-choosing roguelike built with Bevy 0.18. The game uses a state machine to control flow and loads levels from RON files. Features include exploration, dialogue, and Undertale-style boss fights.

## Startup Flow

```
cargo run
    │
    ▼
main.rs
    │
    ├── DefaultPlugins (window, input, rendering)
    ├── StatePlugin (game state machine)
    ├── CameraPlugin (camera + follow system)
    ├── HitBoxPlugin (collision detection)
    ├── LevelPlugin (RON asset loading, level resources)
    ├── PlayerPlugin (movement system)
    ├── DoorPlugin (door interaction)
    ├── RoamingPlugin (entity roaming behavior)
    └── ProjectilePlugin (projectile movement + collision)
```

## State Machine

```
┌──────────────────────────────────────────────────────────────────────────┐
│                                                                          │
│   Loading ──────► LoadingNewLevel ──────► Dialogue ──────► Playing      │
│                        │                      │               │          │
│                        │                      │               ▼          │
│                        │                      │            Paused        │
│                        │                      │               │          │
│                        ▼                      ▼               │          │
│              (if boss level) ────────► BossFight ◄───────────┘          │
│                                            │                             │
│                                            ▼                             │
│                                         Defeat                           │
│                                            │                             │
│                                            │ (ESC - restart)             │
│                                            ▼                             │
│                                    LoadingNewLevel                       │
│                                                                          │
│   Playing ───(touch door)───► LoadingNewLevel                           │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

## Detailed State Descriptions

### 1. Loading (Initial State)
**File:** `state/loading.rs`

- Spawns loading screen UI ("Loading...")
- Animates loading text dots
- Waits 1 second (LoadingTimer)
- Transitions to `LoadingNewLevel`

### 2. LoadingNewLevel
**File:** `state/loading_new_level.rs`

**OnEnter:**
- `despawn_level_entities()` - Removes all entities with `LevelEntity` component
- `spawn_loading_new_level_screen()` - Shows "Loading Room..." UI
- `start_loading_next_level()` - Begins async RON asset loading

**Update:**
- `animate_loading_room()` - Animates dots
- `check_new_level_ready()` - Polls `Assets<LevelData>` until loaded, then:
  - Stores data in `LoadedLevelData` resource
  - Calls `spawn_level_from_data_internal()` to spawn walls, doors, player
  - Transitions based on level type:
    - Has dialogue → `Dialogue`
    - Boss level (`room_type: "boss"`) → `BossFight`
    - Normal level → `Playing`

**OnExit:**
- Despawns loading screen

### 3. Dialogue
**File:** `state/dialogue.rs`

**OnEnter:**
- `reset_dialogue_state()` - Sets current_line to 0
- `spawn_dialogue_panel()` - Creates UI panel with first dialogue line

**Update:**
- `advance_dialogue()` - On Space/Enter:
  - Increments line index
  - Updates speaker/text UI
  - When exhausted, transitions based on level type:
    - Boss level → `BossFight`
    - Normal level → `Playing`

**OnExit:**
- `despawn_dialogue_panel()`

### 4. Playing
**File:** Various

**Active Systems:**
- `move_player` (player.rs) - WASD movement with wall collision
- `follow_player` (camera.rs) - Camera lerps to player position
- `detect_col_with_player` (hitbox.rs) - Sends collision messages
- `handle_door_touch` (door.rs) - Door collision triggers level transition
- `roam` (roaming.rs) - Entities with `Roam` component move randomly
- `toggle_pause` (state/mod.rs) - ESC toggles pause

**On Door Touch:**
1. `handle_door_touch` reads `LevelDoor.leads_to`
2. Updates `CurrentLevel.level_id` to new level
3. Transitions to `LoadingNewLevel`

### 5. Paused
**File:** `state/pause.rs`

**OnEnter:**
- `spawn_pause_menu()` - Shows pause UI

**Update:**
- `toggle_pause()` - ESC returns to `Playing`

**OnExit:**
- `despawn_pause_menu()`

### 6. BossFight
**File:** `state/boss_fight.rs`

**OnEnter:**
- `spawn_boss_arena()` - Spawns:
  - Combat arena walls (confines player)
  - Boss ASCII art above arena
  - `PlayerArena` component with dimensions
- `reset_attack_timer()` - Initializes projectile spawning

**Update:**
- `move_player` (player.rs) - Player moves within arena
- `fire_projectiles_at_player` (boss_fight.rs) - Every 1 second:
  - Spawns projectile from boss position
  - Aimed at player with random speed (150-300)
  - Stops after 15 projectiles
- `move_projectiles` (projectile.rs) - Projectiles move by velocity
- `handle_projectile_touch_player` (projectile.rs) - On hit:
  - Despawns projectile
  - Decrements player health
  - If health <= 0 → `Defeat`

### 7. Defeat
**File:** `state/defeat.rs`

**OnEnter:**
- `spawn_defeat_menu()` - Shows game over UI

**Update:**
- `toggle_pause()` - ESC triggers restart:
  - Resets `PlayerHealth` to max
  - Resets `CurrentLevel` to "level_01_intro"
  - Transitions to `LoadingNewLevel`

**OnExit:**
- `despawn_defeat_menu()`

## Level Loading Flow

```
CurrentLevel.level_id = "level_01_intro"
            │
            ▼
asset_server.load("levels/level_01_intro.ron")
            │
            ▼
    Bevy parses RON into LevelData struct
            │
            ▼
check_new_level_ready() detects asset loaded
            │
            ▼
spawn_level_from_data_internal()
    ├── Spawns walls (square border or cave generation)
    ├── Spawns doors from level_data.doors (with extra components)
    └── Spawns player at level_data.player_start
```

## Key Resources

| Resource | Purpose |
|----------|---------|
| `CurrentLevel` | Tracks level_id, asset handle, loaded status |
| `LoadedLevelData` | Stores parsed LevelData for current level |
| `DialogueState` | Tracks current dialogue line index |
| `PlayerHealth` | Player's current and max health (default: 3/3) |
| `AttackTimer` | Boss fight projectile spawn timer + count |

## Key Components

| Component | Purpose |
|-----------|---------|
| `Player` | Marks the player entity |
| `Wall` | Marks wall entities (blocks movement) |
| `LevelDoor` | Door with `leads_to` field for next level |
| `LevelEntity` | Marks entities to despawn on level transition |
| `HitBox` | Collision bounds (width, height) |
| `Roam` | Enables roaming behavior (speed, range) |
| `Projectile` | Projectile with velocity |
| `Boss` | Marks the boss entity |
| `PlayerArena` | Boss fight arena bounds |

## RON Level Format

```ron
(
    id: "level_01_intro",
    name: "The Beginning",
    room_type: "square",  // "square", "cave", or "boss"
    player_start: (0.0, -200.0),
    dialogue: [
        (speaker: "???", text: "You awaken..."),
    ],
    doors: [
        (
            position: (-200.0, 200.0),
            leads_to: "level_02",
            label: "Left Door",
            locked: false,
        ),
        (
            position: (200.0, 200.0),
            leads_to: "level_03",
            label: "Wandering Door",
            locked: false,
            extra: [Roam(speed: 30.0, range: 100.0)],
        ),
    ],
    boss: None,  // or Some("boss_name") for boss levels
    items: [],
)
```

## Boss Level RON Format

```ron
(
    id: "boss_test",
    name: "Test Boss Arena",
    room_type: "boss",  // This triggers BossFight state
    player_start: (0.0, -150.0),
    dialogue: [
        (speaker: "BOSS", text: "Prepare yourself..."),
    ],
    doors: [],  // No doors in boss fights
    boss: Some("test_boss"),
    items: [],
)
```

## EntityComponent System

Doors (and potentially other entities) can have extra components added via the `extra` field in RON:

```rust
// Defined in level_schema.rs
pub enum EntityComponent {
    Roam { speed: f32, range: f32 },
    // Add more variants as needed
}
```

**How it works:**
1. RON file defines `extra: [Roam(speed: 30.0, range: 100.0)]`
2. `spawn_door_from_data()` spawns the door entity
3. Loops through `extra`, matches on each `EntityComponent`
4. Inserts the corresponding Bevy component via `commands.entity(id).insert(...)`

**Adding new behaviors:**
1. Add variant to `EntityComponent` enum in `level_schema.rs`
2. Add match arm in `spawn_door_from_data()` in `level.rs`
3. Use it in RON files

## Shutdown

- Close window or Ctrl+C in terminal
- Bevy handles cleanup automatically
