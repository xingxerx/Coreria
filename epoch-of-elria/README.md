# Epoch of Elria - A Universe of Games and Tools

This repository contains the "Epoch of Elria" project, a collection of games and tools developed in C++ and Rust. The main project is "The Dream Weaver's Heart," a narrative-driven 3D RPG.

---

## 🌟 The Dream Weaver's Heart - A Narrative 3D RPG

**The Dream Weaver's Heart** is an immersive RPG set in the infinite Metaverse, where consciousness shapes reality through stories, memories, and pure melody. Four unique heroes must unite to transform The One - an ancient entity of absolute order - through collaboration rather than destruction.

### 🎭 Main Characters

-   **Xing (The Weaver)** - Master of narrative reality, dwells in the Infinite Library
-   **Xerx (The Liberator)** - Memory warrior fighting mental oppression in sterile reality
-   **The Heart** - Emotional catalyst born from a dying book, eager to participate in stories
-   **Lyra (Pure Melody)** - Awakener of consciousness through harmonic resonance

### 👹 The Antagonist

-   **The One** - Ancient entity seeking absolute order through narrative suppression

### 🎮 Core Gameplay Features

-   **Character Switching System**: Switch between any of the four heroes, each with unique abilities.
-   **3D Metaverse Exploration**: Navigate infinite narrative dimensions with WASD controls in a real-time 3D world.
-   **RPG Progression**: Level up characters, learn new abilities, and build relationships for synergy bonuses.
-   **Collaborative Storytelling**: Create stories that become part of the game world and transform the antagonist through love and harmony, not violence.
-   **Advanced Systems**:
    -   **Memory System**: Reconstruct complete memories from scattered fragments.
    -   **Harmony System**: Build resonance between characters to achieve ultimate power.
    -   **Narrative Magic**: Use story-based spells to alter reality.

### 🔧 Building and Running

To build and run the game, use the following `make` commands:

```bash
# Build the complete game
make dream_weaver_complete

# Run the game
make run-complete

# Build the RPG character demo
make rpg_characters

# Run the RPG character demo
make run-rpg

# Build the 3D open world demo
make game_3d_openworld

# Run the 3D open world demo
make run-3d
```

---

## 🤖 Arch Reactor AI - Moderation Tool

A chat moderation tool written in Rust. This tool is a separate, experimental project.

### Features

-   Filters messages based on a list of banned words.
-   Detects excessive use of capitalization.
-   Configurable via `moderation_config.toml`.
-   Includes a placeholder for integration with a "Gemini" AI for advanced analysis.

### Running

```bash
# Navigate to the epoch-of-elria directory and run
cargo run --bin ArchReactorAIlogic
```

---

## 🧠 Meta-Cognition Layer

A small, self-contained Rust program demonstrating a concept of self-reflection for AI systems. It is likely intended as a conceptual layer for the Arch Reactor AI.

### Running

```bash
# Navigate to the epoch-of-elria directory and run
cargo run --bin Meta-CognitionLayer
```

---

## Legacy Engine

The repository also contains the original 2D C++ game engine with parallel processing and SVG graphics. This was the precursor to "The Dream Weaver's Heart."

For more information on the legacy engine, please see the [LEGACY_README.md](LEGACY_README.md).

---

## 📝 License

This project is part of the "Epoch of Elria" game development series. Open source and free to use.

## 🤝 Contributing

Feel free to submit issues, feature requests, or pull requests to improve the engine and games!
