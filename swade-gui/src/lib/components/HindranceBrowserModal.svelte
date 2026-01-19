<script lang="ts">
  import {
    commands,
    type CharacterView,
    type HindranceView,
    type GameConfig,
  } from "$lib/bindings";
  import { onMount } from "svelte";
  import HindranceSelector from "./selectors/HindranceSelector.svelte";

  interface Props {
    character: CharacterView;
    onClose: () => void;
    onCharacterChanged: (character: CharacterView, warnings: string[]) => void;
  }

  let { character, onClose, onCharacterChanged }: Props = $props();

  let hindrances = $state<HindranceView[]>([]);
  let gameConfig = $state<GameConfig | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let error = $state<string | null>(null);
  let warnings = $state<string[]>([]);

  onMount(async () => {
    // Load hindrances and game config in parallel
    const [hindrancesResult, config] = await Promise.all([
      commands.getHindrances(),
      commands.getGameConfig(),
    ]);

    if (hindrancesResult.status === "ok") {
      hindrances = hindrancesResult.data;
    } else {
      error = hindrancesResult.error.message;
    }

    gameConfig = config;
    loading = false;
  });

  async function handleAddHindrance(hindranceId: number) {
    saving = true;
    error = null;

    // Use bypass_validation = true for edit mode
    const result = await commands.addDraftHindrance(hindranceId, true);
    if (result.status === "ok") {
      onCharacterChanged(result.data.character, result.data.warnings.map(w => w.message));
      warnings = result.data.warnings.map(w => w.message);
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function handleRemoveHindrance(hindranceId: number) {
    saving = true;
    error = null;

    const result = await commands.removeDraftHindrance(hindranceId);
    if (result.status === "ok") {
      onCharacterChanged(result.data, []);
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      onClose();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onClose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div
  class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
  onclick={handleBackdropClick}
  role="dialog"
  aria-modal="true"
  aria-labelledby="hindrance-browser-title"
>
  <div class="bg-white dark:bg-zinc-800 rounded-lg shadow-xl max-w-4xl w-full max-h-[85vh] flex flex-col">
    <!-- Header -->
    <div class="flex items-center justify-between px-6 py-4 border-b border-zinc-200 dark:border-zinc-700">
      <h2 id="hindrance-browser-title" class="text-lg font-semibold text-zinc-900 dark:text-zinc-100">
        Edit Hindrances
      </h2>
      <button
        onclick={onClose}
        class="text-zinc-400 hover:text-zinc-600 dark:hover:text-zinc-300 transition-colors"
        aria-label="Close"
      >
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto px-6 py-4">
      {#if loading}
        <div class="text-center text-zinc-500 py-8">Loading hindrances...</div>
      {:else}
        <HindranceSelector
          {character}
          {hindrances}
          {gameConfig}
          onAddHindrance={handleAddHindrance}
          onRemoveHindrance={handleRemoveHindrance}
          bypassPointLimit={true}
          {saving}
          {error}
          {warnings}
        />
      {/if}
    </div>

    <!-- Footer -->
    <div class="flex justify-end gap-3 px-6 py-4 border-t border-zinc-200 dark:border-zinc-700">
      <button
        onclick={onClose}
        class="px-4 py-2 text-sm font-medium rounded-md bg-zinc-100 text-zinc-700 hover:bg-zinc-200 dark:bg-zinc-700 dark:text-zinc-300 dark:hover:bg-zinc-600 transition-colors"
      >
        Done
      </button>
    </div>
  </div>
</div>
