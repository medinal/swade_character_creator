# SWADE GUI

Desktop application for the SWADE Character Creator, built with Tauri 2 + SvelteKit.

## Tech Stack

- **Tauri 2** - Rust-based desktop application framework
- **SvelteKit** - Frontend framework with Svelte 5 runes
- **TypeScript** - Type-safe frontend code
- **Tailwind CSS** - Utility-first styling
- **tauri-specta** - Automatic TypeScript type generation from Rust
- **Tiptap** - Rich text editor for notes
- **marked** - Markdown parsing

## Development

```bash
# Install dependencies
npm install

# Run in development mode (hot reload)
npm run tauri dev

# Build for production
npm run tauri build
```

## Architecture

### Type-Safe Frontend/Backend Communication

The GUI uses **tauri-specta** to automatically generate TypeScript types from Rust view models. This ensures the frontend and backend stay in sync without manual type definitions.

```
┌─────────────────────────────────────────────────────────────┐
│                      Frontend (SvelteKit)                    │
│                                                              │
│   import { commands, type CharacterView } from "$lib/bindings"
│   const result = await commands.getCharacters()              │
│                         │                                    │
└─────────────────────────│────────────────────────────────────┘
                          │ Typed IPC
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                      Backend (Tauri/Rust)                    │
│                                                              │
│   #[tauri::command]                                          │
│   #[specta::specta]                                          │
│   fn get_characters(...) -> Result<Vec<CharacterView>, ...>  │
│                         │                                    │
└─────────────────────────│────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                    swade-core Library                        │
│                                                              │
│   #[derive(Serialize, Deserialize, specta::Type)]           │
│   pub struct CharacterView { ... }                           │
└─────────────────────────────────────────────────────────────┘
```

### Generated Bindings

TypeScript bindings are automatically generated to `src/lib/bindings.ts` during development. This file includes:

- **Typed Commands** - `commands.getCharacters()`, `commands.createCharacter(name)`, etc.
- **Type Definitions** - All view models (CharacterView, EdgeView, Die, etc.)
- **Result Type** - Proper error handling with `Result<T, E>` pattern

### Data Flow

1. Frontend calls typed command: `commands.getCharacters()`
2. Tauri invokes the Rust handler with type-safe parameters
3. Rust handler uses swade-core services to load data
4. Response is serialized and returned with proper types
5. Frontend receives typed `Result<CharacterView[], string>`

### Adding New Commands

1. Add the command in `src-tauri/src/lib.rs`:
   ```rust
   #[tauri::command]
   #[specta::specta]
   fn my_command(arg: String) -> Result<MyType, String> {
       // implementation
   }
   ```

2. Register in the builder's `collect_commands!` macro

3. Ensure `MyType` derives `specta::Type` in swade-core

4. Run `npm run tauri dev` - bindings regenerate automatically

5. Use in frontend:
   ```typescript
   import { commands } from "$lib/bindings";
   const result = await commands.myCommand("arg");
   ```

## Project Structure

```
swade-gui/
├── src/
│   ├── lib/
│   │   ├── bindings.ts       # Auto-generated TypeScript types
│   │   ├── character-utils.ts
│   │   ├── components/       # Svelte components
│   │   │   ├── WizardLayout.svelte, WizardHeader.svelte, WizardFooter.svelte
│   │   │   ├── StepNav.svelte, ErrorAlert.svelte
│   │   │   ├── AdvancementModal.svelte, AdvancementHistory.svelte
│   │   │   ├── ConfirmDeleteModal.svelte, TextInputModal.svelte
│   │   │   ├── EdgeImpactWarningModal.svelte, HindrancePointsCard.svelte
│   │   │   ├── GearBrowserModal.svelte, GearItem.svelte
│   │   │   ├── NotesPanel.svelte, PortraitUpload.svelte
│   │   │   └── ResourcePool.svelte, StatusTracker.svelte
│   │   ├── contexts/         # Svelte context providers (wizardContext.ts)
│   │   └── utils/            # Utility functions (formatting, modifiers, etc.)
│   ├── routes/
│   │   ├── +page.svelte      # Main application page (character list)
│   │   ├── character/[id]/   # Character view/edit page
│   │   └── create/           # Character creation wizard
│   │       ├── +page.svelte  # Wizard entry point
│   │       ├── ancestry/
│   │       ├── attributes/
│   │       ├── edges/
│   │       ├── hindrances/
│   │       ├── powers/
│   │       └── skills/
│   └── app.css               # Global styles (Tailwind)
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs            # Tauri app setup and command registration
│   │   ├── main.rs           # Entry point
│   │   ├── state.rs          # Application state management
│   │   ├── error.rs          # Error types
│   │   └── commands/         # Tauri command handlers
│   │       ├── mod.rs
│   │       ├── character.rs
│   │       ├── ancestry.rs
│   │       ├── attributes.rs
│   │       ├── edges.rs
│   │       ├── gear.rs
│   │       ├── hindrances.rs
│   │       ├── skills.rs
│   │       ├── powers.rs
│   │       ├── notes.rs
│   │       └── advancement.rs
│   └── Cargo.toml            # Rust dependencies
└── package.json
```

## IDE Setup

Recommended extensions for VS Code:
- [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
