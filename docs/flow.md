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
    ├── CurrentMusic resource (audio state)
    ├── StatePlugin (game state machine)
    ├── CameraPlugin (camera + follow system)
    ├── HitBoxPlugin (collision detection)
    ├── LevelPlugin (RON asset loading, level resources)
    ├── PlayerPlugin (movement system)
    ├── DoorPlugin (door interaction)
    ├── RoamingPlugin (entity roaming behavior)
    ├── FollowPlugin (NPC follow player behavior)
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
- Initializes `StoryFlags` with default values (duck_status, duck_present, duck_health)
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
  - Handles level music (play/stop based on `level_data.music`)
  - Transitions based on level type:
    - Has dialogue → `Dialogue`
    - Boss level (`room_type: "boss"`) → `BossFight`
    - Normal level → `Playing`

**OnExit:**
- Despawns loading screen

### 3. Dialogue
**File:** `state/dialogue.rs`

**OnEnter:**
- `reset_dialogue_state()` - Sets current_line to 0 (runs first via `.chain()`)
- `spawn_dialogue_panel()` - Creates UI panel, skips lines from absent NPCs
- `spawn_health_ui()` - Shows player health
- `spawn_follower_health_ui()` - Shows follower health (e.g., duck)

**Update:**
- `advance_dialogue()` - On Space/Enter:
  - Increments line index
  - Skips dialogue from NPCs where `{name}_present` is false
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
- `spawn_health_ui()` - Shows player health
- `spawn_follower_health_ui()` - Shows follower health

**Update:**
- `move_player` (player.rs) - Player moves within arena
- `follow` (follow.rs) - NPCs with `Follow` component lerp toward player
- `fire_projectiles_at_player` (boss_fight.rs) - Every 1 second:
  - Spawns projectile from boss position
  - Aimed at player with random speed (150-300)
  - Stops after 15 projectiles
- `move_projectiles` (projectile.rs) - Projectiles move by velocity
- `handle_projectile_touch_player` (projectile.rs) - On hit:
  - Despawns projectile
  - Decrements player health
  - If health <= 0 → `Defeat`
- `handle_projectile_touch_npc` (projectile.rs) - On NPC hit:
  - Despawns projectile
  - Decrements `{npc_name}_health` in StoryFlags
  - If health <= 0: sets `{name}_present = false`, `{name}_status = "died_in_boss"`, despawns NPC
- `update_follower_health_ui` (ui.rs) - Updates when StoryFlags changes

### 7. Defeat
**File:** `state/defeat.rs`

**OnEnter:**
- `spawn_defeat_menu()` - Shows game over UI
- `despawn_health_ui()` - Removes player health UI
- `despawn_follower_health_ui()` - Removes follower health UI

**Update:**
- `toggle_pause()` - ESC triggers restart:
  - Resets `PlayerHealth` to max
  - Resets `StoryFlags` (duck_status = "alive", duck_present = true, duck_health = 3)
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
    ├── Spawns NPCs from level_data.npcs (checks StoryFlags for presence)
    └── Spawns player at level_data.player_start
            │
            ▼
Handle level music
    ├── level_data.music = Some(track) → play_music(track)
    └── level_data.music = None → stop_music()
```

## Story Flags System

**File:** `story_flags.rs`

A HashMap-based system for tracking persistent game state across levels.

```rust
StoryFlags {
    flags: HashMap<String, FlagValue>
}

FlagValue = Bool(bool) | Text(String) | Number(i32)
```

**Naming Convention:**
- `{npc_name}_status` → "alive", "dead", "traded" (what happened)
- `{npc_name}_present` → true/false (can they spawn/speak)
- `{npc_name}_health` → current health
- `{npc_name}_max_health` → max health

**How it affects gameplay:**
1. **NPC Spawning:** `spawn_npc_from_data()` checks `{name}_present` - if false, NPC doesn't spawn
2. **Dialogue:** `can_speaker_speak()` checks `{name}_present` - skips lines from absent NPCs
3. **Combat:** `handle_projectile_touch_npc()` decrements `{name}_health`, sets flags on death
4. **Health UI:** `update_follower_health_ui()` reads health from flags

**Example flow when duck dies:**
```
Projectile hits duck
    │
    ▼
duck_health: 3 → 2 → 1 → 0
    │
    ▼
duck_present = false
duck_status = "died_in_boss"
    │
    ▼
Duck entity despawned
    │
    ▼
Next level: duck doesn't spawn, Duck dialogue skipped
```

## Audio System

**File:** `audio.rs`

The audio system handles background music that persists across level transitions.

### CurrentMusic Resource

```rust
pub struct CurrentMusic {
    pub entity: Option<Entity>,  // The audio entity
    pub track: Option<String>,   // Track name currently playing
}
```

### How Music Works

1. Each level can specify a `music` field in its RON file:
   - `music: Some("track_name")` - plays that track
   - `music: None` - stops music (silence)

2. When loading a level (`check_new_level_ready()`):
   - If level has music and it's **different** from current → switch tracks
   - If level has music and it's **same** as current → music continues uninterrupted
   - If level has `None` → stop music

3. Music files are loaded from `assets/sounds/music/{track_name}.mp3`

### Functions

| Function | Purpose |
|----------|---------|
| `play_music()` | Start/switch music (skips if same track playing) |
| `stop_music()` | Stop current music and clear track state |
| `play_sfx()` | Play one-shot sound effect |

### Level Music Flow

```
Level 1 (music: "exploration")
    │
    ▼
play_music("exploration") → starts playing
    │
    ▼
Level 2 (music: "exploration")  ← same track
    │
    ▼
play_music("exploration") → NO-OP (already playing)
    │
    ▼
Boss Level (music: "boss_theme")  ← different track
    │
    ▼
play_music("boss_theme") → stops old, starts new
    │
    ▼
Silent Level (music: None)
    │
    ▼
stop_music() → music stops
```

### RON Level Music Examples

```ron
// Exploration levels share the same track
(
    id: "level_01",
    music: Some("Claude debussy_Clair de lune (8-Bit)"),
    ...
)

// Boss level with different music
(
    id: "boss_test",
    music: Some("Playboi Carti - Magnolia (Official Video)"),
    ...
)

// Silent level
(
    id: "quiet_room",
    music: None,
    ...
)
```

## Key Resources

| Resource | Purpose |
|----------|---------|
| `CurrentLevel` | Tracks level_id, asset handle, loaded status |
| `LoadedLevelData` | Stores parsed LevelData for current level |
| `DialogueState` | Tracks current dialogue line index |
| `PlayerHealth` | Player's current and max health (default: 3/3) |
| `AttackTimer` | Boss fight projectile spawn timer + count |
| `StoryFlags` | HashMap-based persistent game state (NPC status, health, presence) |
| `CurrentMusic` | Tracks current music entity and track name |

## Key Components

| Component | Purpose |
|-----------|---------|
| `Player` | Marks the player entity |
| `Wall` | Marks wall entities (blocks movement) |
| `LevelDoor` | Door with `leads_to` field for next level |
| `LevelEntity` | Marks entities to despawn on level transition |
| `HitBox` | Collision bounds (width, height) |
| `Roam` | Enables roaming behavior (speed, range) |
| `Follow` | Enables follow-player behavior (speed, distance) |
| `Npc` | Marks NPC entity with name (maps to StoryFlags) |
| `Projectile` | Projectile with velocity |
| `Boss` | Marks the boss entity |
| `PlayerArena` | Boss fight arena bounds |
| `FollowerHealthContainer` | UI container for follower health (links to npc_name) |
| `FollowerHeartDisplay` | Individual heart in follower health UI |

## RON Level Format

```ron
(
    id: "level_01_intro",
    name: "The Beginning",
    room_type: "square",  // "square", "cave", or "boss"
    player_start: (0.0, -200.0),
    dialogue: [
        (speaker: "???", text: "You awaken..."),
        (speaker: "Duck", text: "Follow me!"),  // Skipped if duck not present
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
    npcs: [
        (
            name: "duck",
            position: (100.0, 200.0),
            extra: [Follow(speed: 5.0, distance: 50.0)],
        ),
    ],
    music: Some("exploration"),  // or None for silence
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
    music: Some("boss_theme"),  // Boss-specific music
)
```

## EntityComponent System

Doors and NPCs can have extra components added via the `extra` field in RON:

```rust
// Defined in level_schema.rs
pub enum EntityComponent {
    Roam { speed: f32, range: f32 },
    Follow { speed: f32, distance: f32 },
}
```

**How it works:**
1. RON file defines `extra: [Follow(speed: 5.0, distance: 50.0)]`
2. `spawn_door_from_data()` or `spawn_npc_from_data()` spawns the entity
3. Loops through `extra`, matches on each `EntityComponent`
4. Inserts the corresponding Bevy component via `commands.entity(id).insert(...)`

**Adding new behaviors:**
1. Add variant to `EntityComponent` enum in `level_schema.rs`
2. Add match arm in `spawn_door_from_data()` and/or `spawn_npc_from_data()` in `level.rs`
3. Use it in RON files

## Shutdown

- Close window or Ctrl+C in terminal
- Bevy handles cleanup automatically
