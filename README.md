# SWADE Character Creator

A comprehensive character creation and management system for **Savage Worlds Adventure Edition (SWADE)**, built with Rust. 

## Project Overview

This application enables players and game masters to create, manage, and advance SWADE characters with full support for:

- **Character Creation Wizard** - UI-agnostic character creation system
- **Character Management** - Save, load, and organize multiple characters
- **Character Advancement** - Track experience and apply advances
- **Rules Validation** - Automatic enforcement of SWADE game rules
- **Game Data Queries** - Comprehensive CLI for viewing all SWADE rules data

The system supports both **Wild Cards** (player characters and major NPCs) and **Extras** (minor NPCs), with complete tracking of attributes, skills, edges, hindrances, powers, and arcane backgrounds.

## Prerequisites

- **Rust 1.85+** - Install via [rustup](https://rustup.rs/)
- **Node.js 18+** - Install from [nodejs.org](https://nodejs.org/) or via a version manager
- **Tauri system dependencies** - Platform-specific requirements:
  - **macOS**: Xcode Command Line Tools (`xcode-select --install`)
  - **Linux**: See [Tauri Linux prerequisites](https://v2.tauri.app/start/prerequisites/#linux)
  - **Windows**: See [Tauri Windows prerequisites](https://v2.tauri.app/start/prerequisites/#windows)

## Quick Start

```bash
# Install frontend dependencies
cd swade-gui
npm install

# Run the desktop application (development mode with hot reload)
npm run tauri dev

# Build for production
npm run tauri build
```

The database is embedded in the application binary - no additional setup required.

## Tech Stack

### Core Library (Rust)
- **Rust 1.85+** - Core application logic and business rules (uses Rust 2024 edition)
- **rusqlite** - Embedded SQLite database with bundled library
- **serde** - JSON serialization and deserialization
- **thiserror** - Comprehensive error handling
- **specta** - TypeScript type generation from Rust types

### Desktop GUI (Tauri + Svelte)
- **Tauri 2** - Native desktop application framework
- **Svelte 5** - Reactive frontend framework
- **TailwindCSS 4** - Utility-first CSS framework
- **TypeScript** - Type-safe frontend development
- **tauri-specta** - Automatic TypeScript bindings generation

### Database
- **SQLite** - Embedded database with all SWADE game rules data
- **Embedded Distribution** - Database compiled into binary, extracted to user data directory
- **Version Tracking** - Automatic updates when rules data changes

## Architecture

The project uses a **layered architecture** with clear separation of concerns:

```
┌─────────────────────────────────────────────────────────────┐
│                    Presentation Layer                        │
│                  (CLI / Desktop GUI)                  │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                     Service Layer                            │
│              (Data Loading & Query Orchestration)            │
│  CharacterService, EdgeService, SkillService, etc.          │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                     View Models                              │
│      (Rich Domain Objects with Business Logic)               │
│  CharacterView, EdgeView, SkillValue, RequirementTree, etc. │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                     Base Models                              │
│              (Pure Data Structures)                          │
│  Character, Edge, Skill, Modifier, Requirement, etc.        │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                  Repository Layer                            │
│              (Database Access - CRUD Only)                   │
│  42 Repositories for all database tables                     │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                  Database Layer                              │
│           (SQLite with Embedded Rules Data)                  │
│  Connection Management, Embedded Database, Versioning        │
└─────────────────────────────────────────────────────────────┘
```

### Key Architectural Decisions

1. **Layered Architecture** - Strict separation between presentation, data loading, domain logic, and storage
2. **Repository Pattern** - All database access through type-safe repository objects
3. **Dual Model System** - Base models (pure data) separate from view models (data + domain logic)
4. **Rich Domain Models** - Business rules (SWADE mechanics, requirement evaluation, cost calculation) live in view models, not services
5. **Service Layer** - Thin orchestration layer for loading data and assembling view models; no business logic
6. **Embedded Database** - SQLite database compiled into binary with automatic updates
7. **Type-Safe Domain Modeling** - Rust's type system enforces SWADE rules at compile time
8. **Automatic Type Generation** - View models are automatically exported to TypeScript for the frontend

### Frontend/Backend Type Sharing

The project uses **tauri-specta** to automatically generate TypeScript types from Rust view models:

```
Rust View Models (swade-core)     TypeScript Types (swade-gui)
─────────────────────────────     ────────────────────────────
CharacterView          ───────►   CharacterView
EdgeView               ───────►   EdgeView
Die, Rank, Modifier    ───────►   Die, Rank, Modifier
... (50+ types total)             ... (auto-generated)
```

**How it works:**
1. View models in `swade-core` derive `specta::Type` alongside `Serialize`/`Deserialize`
2. Tauri commands in `swade-gui` are annotated with `#[specta::specta]`
3. On dev build, TypeScript bindings are auto-generated to `swade-gui/src/lib/bindings.ts`
4. Frontend imports typed `commands` object instead of using raw `invoke()`

**Benefits:**
- Type changes in Rust immediately surface as TypeScript compiler errors
- No manual type synchronization between frontend and backend
- Generated command wrappers provide full autocomplete and type checking
- Result types properly model success/error states

## Data Design

The database schema is designed to be:
- **Programmatic** - Integer die sizes (4, 6, 8, 10, 12) for easy calculations
- **Flexible** - String-based targeting allows for derived stats and future expansion
- **Extensible** - Source tracking supports multiple rulebooks
- **Comprehensive** - Handles full complexity of SWADE character mechanics

### Core Tables

**Game Rules Data:**
- `attributes` - The five core attributes (Agility, Smarts, Spirit, Strength, Vigor)
- `skills` - All available skills with linked attributes
- `ranks` - Character progression ranks (Novice → Legendary)
- `ancestries` - Character races/species with racial abilities
- `edges` - Character advantages organized by category
- `hindrances` - Character disadvantages (minor/major variants)
- `powers` - Arcane powers with costs and effects
- `arcane_backgrounds` - Types of magic/psionics available
- `gear` - Equipment items with categories and stats
- `gear_categories` - Hierarchical equipment categorization
- `weapon_stats` - Weapon-specific statistics (damage, range, etc.)
- `armor_stats` - Armor-specific statistics (protection values)
- `shield_stats` - Shield-specific statistics
- `ammunition_stats` - Ammunition-specific statistics

**Flexible Systems:**
- `modifiers` - Changes to attributes, skills, and derived stats
  - Die increments (d6 → d8)
  - Roll bonuses (+2 to rolls)
  - Flat bonuses (Pace +2)
  - Descriptive abilities
- `requirements` - Prerequisites for edges, powers, etc.
- `requirement_expressions` - Tree structure for complex logical requirements (AND/OR/NOT)
- `ancestry_choices` - Choice-based bonuses (free edges, heritage traits)
- `gear_modifiers` - Modifiers granted by equipment
- `gear_requirements` - Prerequisites for equipment use

**Character Data:**
- `characters` - Main character records with point tracking
- `character_attributes` - Character's attribute values
- `character_skills` - Character's skill values
- `character_edges` - Edges taken by character
- `character_hindrances` - Hindrances selected
- `character_powers` - Powers known
- `character_arcane_backgrounds` - Arcane background selections
- `character_gear` - Equipment owned by character
- `character_advances` - Character advancement history
- `character_modifiers` - Custom modifiers applied to character
- `character_notes` - Character notes and backstory

### Database Features

- **Point Tracking** - Automatic tracking of attribute, skill, and hindrance points
- **Requirement Validation** - Expression trees support complex prerequisites
- **Modifier Application** - Type-safe modifier system with multiple value types
- **Ancestry Choices** - Flexible system for free edges, mandatory hindrances, heritage traits
- **Foreign Key Constraints** - Data integrity enforced at database level
- **Version Tracking** - SHA256 checksums enable automatic updates

For complete database documentation, see [`./db/README.md`](./db/README.md).

## Contributing

Contributions are welcome! Please follow these guidelines:

### Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Run Clippy and fix warnings (`cargo clippy`)
- Add documentation for public APIs
- Include examples in documentation

### Testing Requirements
- NO DOCTESTS
- Tests should be within a module in the file that they're testing
- Add unit tests for new functionality
- Ensure all tests pass (`cargo test`)
- Test error cases, not just happy paths

### Architecture Guidelines
- Respect layer boundaries (no cross-layer calls)
- Keep business logic in view models
- Keep data access in repositories
