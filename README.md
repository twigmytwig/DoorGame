# Bevy Door-Choosing Roguelike

An ASCII-based door-choosing roguelike built with Bevy 0.18. Features exploration, dialogue, NPCs that follow you, and Undertale-style boss fights.

## Prerequisites

### Install Rust

**Windows:**
1. Download and run [rustup-init.exe](https://rustup.rs/)
2. Follow the on-screen instructions
3. Restart your terminal

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify installation:
```bash
rustc --version
cargo --version
```

### System Dependencies

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
```

**Linux (Fedora):**
```bash
sudo dnf install gcc-c++ libX11-devel alsa-lib-devel systemd-devel
```

**macOS:**
No additional dependencies required (Xcode Command Line Tools recommended).

**Windows:**
No additional dependencies required.

## Building and Running

Clone the repository and run:

```bash
cd bevy_game
cargo run --release
```

The `--release` flag enables optimizations for better performance. First build will take a few minutes as it compiles Bevy and dependencies.

For development (faster compile, slower runtime):
```bash
cargo run
```

## Controls

| Key | Action |
|-----|--------|
| WASD | Move |
| Space/Enter | Advance dialogue |
| ESC | Pause / Restart (on defeat) |

## Gameplay

- Navigate through rooms by walking into doors
- Dialogue plays automatically when entering rooms with NPCs
- Some levels have boss fights - dodge the projectiles!
- NPCs like the duck will follow you (and can die in boss fights)

## Project Structure

```
bevy_game/
├── src/
│   ├── main.rs          # App entry point
│   ├── player.rs        # Player movement
│   ├── level.rs         # Level loading/spawning
│   ├── level_schema.rs  # RON level data structures
│   ├── audio.rs         # Music and sound effects
│   ├── state/           # Game states (loading, playing, boss, etc.)
│   └── ...
├── assets/
│   ├── levels/          # RON level files
│   └── sounds/
│       ├── music/       # Background music (mp3)
│       └── sfx/         # Sound effects
└── docs/                # Documentation
```

## Adding Levels

Create a new `.ron` file in `assets/levels/`:

```ron
(
    id: "my_level",
    name: "My Custom Level",
    room_type: "square",
    player_start: (0.0, -200.0),
    dialogue: [],
    doors: [
        (
            position: (0.0, 200.0),
            leads_to: "next_level",
            label: "Exit",
            locked: false,
        ),
    ],
    boss: None,
    items: [],
    npcs: [],
    music: Some("track_name"),  // or None for silence
)
```

## License

MIT
