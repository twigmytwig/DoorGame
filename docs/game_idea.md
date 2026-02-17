# Game Idea

## Core Rule
No art, only ASCII.

## Concept
A door-choosing game where you progress through rooms by selecting doors. Once you enter a room, there's no going back—only forward.

## Gameplay Loop
1. Player enters a room
2. Room presents multiple doors to choose from
3. Player picks a door
4. New room grants a benefit or inflicts a penalty
5. Repeat

## Room Effects
Rooms can:
- Grant buffs (health, power, speed, etc.)
- Deal damage or apply debuffs
- Provide items or resources
- Present risk/reward tradeoffs

## Strategy Element
Players must weigh their choices carefully since:
- No backtracking allowed
- Resources/health are limited
- Door choices may have visual hints or be completely unknown
- Cumulative effects matter for survival

## Door Bosses
Undertale-inspired boss encounters where:
- Doors attack the player directly
- Player must dodge incoming door projectiles/patterns
- Survival-based combat (dodge, don't attack)
- ASCII patterns flying at the player

## Keys and Locked Doors
- Some doors are locked and require keys to open
- Keys found in certain rooms or as rewards
- Locked doors may hide better rewards (risk/reward)
- Creates resource management decisions (use key now or save it?)

## Audio
- Each door has a unique creaking sound when opened
- Different door types have distinct creaking voices/personalities

## The Duck (Companion System)
- A duck companion follows the player through the game
- The duck provides company, maybe hints, or light dialogue
- In future levels, the player may encounter a trade: **give up the duck for a key**
- The key unlocks a valuable door, but you lose your companion forever
- Reinforces the core theme: **decisions are hard, and some choices can't be undone**
- Do you sacrifice your friend for progress? Or keep them and find another way?
- No right answer—just consequences

## Story Flags System (Branching Narrative)

### Purpose
Track player choices and game events to enable:
- Branching dialogue (NPCs react to what you've done)
- Conditional level destinations (doors lead to different places based on history)
- Persistent consequences (duck died vs duck traded = different story beats)

### Approach: HashMap-based Flags
We use a flexible `HashMap<String, FlagValue>` instead of hard-coded struct fields.

**Why HashMap over struct fields:**
| Approach | Adding new flags | Code changes needed? |
|----------|------------------|---------------------|
| Struct (`duck_alive: bool`) | Add new field | Yes - recompile |
| HashMap (`flags["duck_alive"]`) | Just use it | No - fully data-driven |

**FlagValue types:**
- `Bool(true/false)` - simple on/off states
- `Text("alive"/"dead"/"traded")` - categorical states
- `Number(5)` - counters, scores, quantities

### Usage Examples

**Setting flags:**
- Duck dies in boss → set `duck_status = Text("died_in_boss")`
- Player trades duck → set `duck_status = Text("traded")`
- Player defeats boss → set `bosses_defeated = Number(1)`

**Conditional dialogue in RON:**
```
(speaker: "NPC", text: "Your duck... I'm sorry.", condition: Equals("duck_status", Text("died_in_boss")))
(speaker: "NPC", text: "You traded him?!", condition: Equals("duck_status", Text("traded")))
(speaker: "Duck", text: "Quack!", condition: Equals("duck_status", Text("alive")))
```

**Conditional door destinations:**
```
leads_to: "level_03",
leads_to_if: [
    (condition: Equals("duck_status", Text("alive")), level: "level_03_with_duck"),
    (condition: Equals("duck_status", Text("traded")), level: "level_03_guilt"),
],
```

### Trade-offs
- **Pro:** Maximum flexibility - add new flags without code changes
- **Pro:** Designers can iterate on narrative without recompiling
- **Con:** Less type-safe - string typos won't be caught at compile time
- **Con:** Need discipline with flag naming conventions

This is the same approach used by narrative tools like Ink and Twine.
