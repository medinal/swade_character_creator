<script lang="ts">
  import { commands, type CharacterAdvanceValue } from "$lib/bindings";
  import { onMount } from "svelte";

  interface Props {
    characterId: number;
    onUndo?: () => void;
  }

  let { characterId, onUndo }: Props = $props();

  let history = $state<CharacterAdvanceValue[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let expanded = $state(true);
  let undoing = $state(false);

  onMount(async () => {
    await loadHistory();
  });

  async function loadHistory() {
    loading = true;
    error = null;

    const result = await commands.getAdvancementHistory(characterId);
    if (result.status === "ok") {
      history = result.data;
    } else {
      error = result.error?.message ?? String(result.error);
    }
    loading = false;
  }

  async function handleUndo() {
    if (history.length === 0) return;
    undoing = true;
    error = null;

    const result = await commands.undoLastAdvance(characterId);
    if (result.status === "ok" && result.data) {
      await loadHistory();
      onUndo?.();
    } else {
      error = result.status === "ok" ? "No advance to undo" : (result.error?.message ?? String(result.error));
    }
    undoing = false;
  }

  function getAdvanceTypeIcon(type: string): string {
    switch (type) {
      case "edge": return "E";
      case "attribute": return "A";
      case "skill_expensive":
      case "skill_cheap": return "S";
      case "hindrance": return "H";
      default: return "?";
    }
  }

  function getAdvanceTypeColor(type: string): string {
    switch (type) {
      case "edge": return "bg-purple-100 text-purple-700 dark:bg-purple-500/20 dark:text-purple-400";
      case "attribute": return "bg-blue-100 text-blue-700 dark:bg-blue-500/20 dark:text-blue-400";
      case "skill_expensive":
      case "skill_cheap": return "bg-green-100 text-green-700 dark:bg-green-500/20 dark:text-green-400";
      case "hindrance": return "bg-amber-100 text-amber-700 dark:bg-amber-500/20 dark:text-amber-400";
      default: return "bg-zinc-100 text-zinc-700 dark:bg-zinc-500/20 dark:text-zinc-400";
    }
  }

  // Refresh history when characterId changes
  $effect(() => {
    if (characterId) {
      loadHistory();
    }
  });
</script>

<div class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5">
  <button
    class="w-full flex items-center justify-between text-left"
    onclick={() => expanded = !expanded}
  >
    <h3 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 uppercase tracking-wide">
      Advancement History
      {#if history.length > 0}
        <span class="text-zinc-500 dark:text-zinc-400 font-normal normal-case">
          ({history.length} advance{history.length !== 1 ? "s" : ""})
        </span>
      {/if}
    </h3>
    <svg
      class="w-4 h-4 text-zinc-500 transition-transform {expanded ? 'rotate-180' : ''}"
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
      aria-hidden="true"
    >
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
    </svg>
  </button>

  {#if expanded}
    <div class="mt-4">
      {#if loading}
        <p class="text-sm text-zinc-500 italic">Loading history...</p>
      {:else if error}
        <p class="text-sm text-red-500">{error}</p>
      {:else if history.length === 0}
        <p class="text-sm text-zinc-500 dark:text-zinc-400 italic">No advances taken yet</p>
      {:else}
        <div class="space-y-2 max-h-64 overflow-y-auto">
          {#each history as advance, idx}
            {@const isLatest = idx === history.length - 1}
            <div class="flex items-start gap-3 p-2 rounded-md {isLatest ? 'bg-zinc-50 dark:bg-zinc-700/50' : ''}">
              <div class="flex-shrink-0 flex flex-col items-center">
                <span class="text-xs font-mono text-zinc-400 dark:text-zinc-500">#{advance.advance_number}</span>
                <span class="w-6 h-6 rounded flex items-center justify-center text-xs font-medium {getAdvanceTypeColor(advance.advance_type)}">
                  {getAdvanceTypeIcon(advance.advance_type)}
                </span>
              </div>
              <div class="flex-1 min-w-0">
                <p class="text-sm text-zinc-700 dark:text-zinc-300">{advance.description}</p>
                <p class="text-xs text-zinc-400 dark:text-zinc-500 mt-0.5">
                  {new Date(advance.created_at).toLocaleDateString()}
                </p>
              </div>
              {#if isLatest}
                <button
                  onclick={handleUndo}
                  disabled={undoing}
                  class="flex-shrink-0 text-xs text-red-600 hover:text-red-700 dark:text-red-400 dark:hover:text-red-300 disabled:opacity-50 disabled:cursor-not-allowed"
                  title="Undo this advance"
                >
                  {undoing ? "..." : "Undo"}
                </button>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>
