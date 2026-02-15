# Game Flow Documentation

## Overview

This is an ASCII-based door-choosing roguelike built with Bevy 0.18. The game uses a state machine to control flow and loads levels from RON files.

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
    └── RoamingPlugin (entity roaming behavior)
```

## State Machine

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│   Loading ──────► LoadingNewLevel ──────► Dialogue ──────►     │
│      │                   │                    │           Playing
│      │                   │                    │              │
│      │                   ▼                    ▼              │
│      │            (if no dialogue) ─────────────────────────►│
│      │                                                       │
│      └──────────────────────────────────────────────────────►│
│                                                              │
│                         ┌────────────────┐                   │
│                         │                │                   │
│                         ▼                │                   │
│   Playing ◄────────► Paused             │                   │
│      │                (ESC)              │                   │
│      │                                   │                   │
│      │  (touch door)                     │                   │
│      ▼                                   │                   │
│   LoadingNewLevel ───────────────────────┘                   │
│                                                               │
└─────────────────────────────────────────────────────────────────┘
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
  - Transitions to `Dialogue` (if level has dialogue) or `Playing`

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
  - Transitions to `Playing` when exhausted

**OnExit:**
- `despawn_dialogue_panel()`

### 4. Playing
**File:** Various

**Active Systems:**
- `move_player` (player.rs) - Arrow key movement with wall collision
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
    ├── Spawns doors from level_data.doors
    └── Spawns player at level_data.player_start
```

## Key Resources

| Resource | Purpose |
|----------|---------|
| `CurrentLevel` | Tracks level_id, asset handle, loaded status |
| `LoadedLevelData` | Stores parsed LevelData for current level |
| `DialogueState` | Tracks current dialogue line index |

## Key Components

| Component | Purpose |
|-----------|---------|
| `Player` | Marks the player entity |
| `Wall` | Marks wall entities (blocks movement) |
| `LevelDoor` | Door with `leads_to` field for next level |
| `LevelEntity` | Marks entities to despawn on level transition |
| `HitBox` | Collision bounds (width, height) |
| `Roam` | Enables roaming behavior (speed, range) |

## RON Level Format

```ron
(
    id: "level_01_intro",
    name: "The Beginning",
    room_type: "square",  // or "cave"
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
            extra: [Roam(speed: 30.0, range: 100.0)],  // Door moves!
        ),
    ],
    boss: None,
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
