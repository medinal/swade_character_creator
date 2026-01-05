<script lang="ts">
  import { onMount } from "svelte";

  interface Props {
    targetName: string;
    affectedEdges: string[];
    onCancel: () => void;
    onConfirm: () => void;
  }

  let { targetName, affectedEdges, onCancel, onConfirm }: Props = $props();

  let cancelButton: HTMLButtonElement;

  onMount(() => {
    // Focus the cancel button when modal opens
    cancelButton?.focus();
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onCancel();
    }
  }
</script>

<div
  class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
  onclick={onCancel}
  onkeydown={handleKeydown}
  role="dialog"
  aria-modal="true"
  aria-labelledby="edge-warning-title"
  tabindex="-1"
>
  <div
    class="bg-white dark:bg-zinc-800 rounded-lg shadow-xl p-6 max-w-md mx-4 w-full"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
    role="none"
  >
    <div class="flex items-center gap-3 mb-4">
      <div class="w-10 h-10 rounded-full bg-amber-100 dark:bg-amber-500/20 flex items-center justify-center">
        <svg class="w-5 h-5 text-amber-600 dark:text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
        </svg>
      </div>
      <h3 id="edge-warning-title" class="text-lg font-semibold text-zinc-900 dark:text-zinc-100">
        Edge Requirements Warning
      </h3>
    </div>

    <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-4">
      Lowering <span class="font-medium text-zinc-900 dark:text-zinc-100">{targetName}</span> will
      cause the following edge{affectedEdges.length > 1 ? 's' : ''} to no longer meet requirements.
      {affectedEdges.length > 1 ? 'They' : 'It'} will be removed and hindrance points refunded:
    </p>

    <ul class="mb-6 space-y-1">
      {#each affectedEdges as edgeName}
        <li class="flex items-center gap-2 text-sm">
          <span class="w-1.5 h-1.5 rounded-full bg-red-500"></span>
          <span class="text-zinc-700 dark:text-zinc-300">{edgeName}</span>
        </li>
      {/each}
    </ul>

    <div class="flex justify-end gap-3">
      <button
        bind:this={cancelButton}
        onclick={onCancel}
        class="px-4 py-2 text-sm font-medium text-zinc-700 dark:text-zinc-300 hover:bg-zinc-100 dark:hover:bg-zinc-700 rounded-md transition-colors"
      >
        Cancel
      </button>
      <button
        onclick={onConfirm}
        class="px-4 py-2 text-sm font-medium text-white bg-amber-600 hover:bg-amber-500 rounded-md transition-colors"
      >
        Continue Anyway
      </button>
    </div>
  </div>
</div>
