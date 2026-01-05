<script lang="ts">
  import { onMount } from "svelte";

  interface Props {
    title: string;
    message: string;
    itemName: string;
    onCancel: () => void;
    onConfirm: () => void;
  }

  let { title, message, itemName, onCancel, onConfirm }: Props = $props();

  let cancelButton: HTMLButtonElement;

  onMount(() => {
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
  aria-labelledby="confirm-delete-title"
  tabindex="-1"
>
  <div
    class="bg-white dark:bg-zinc-800 rounded-lg shadow-xl p-6 max-w-md mx-4 w-full"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
    role="none"
  >
    <div class="flex items-center gap-3 mb-4">
      <div class="w-10 h-10 rounded-full bg-red-100 dark:bg-red-500/20 flex items-center justify-center">
        <svg class="w-5 h-5 text-red-600 dark:text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
        </svg>
      </div>
      <h3 id="confirm-delete-title" class="text-lg font-semibold text-zinc-900 dark:text-zinc-100">
        {title}
      </h3>
    </div>

    <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-2">
      {message}
    </p>

    <p class="text-sm font-medium text-zinc-900 dark:text-zinc-100 mb-6">
      "{itemName}"
    </p>

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
        class="px-4 py-2 text-sm font-medium text-white bg-red-600 hover:bg-red-500 rounded-md transition-colors"
      >
        Delete
      </button>
    </div>
  </div>
</div>
