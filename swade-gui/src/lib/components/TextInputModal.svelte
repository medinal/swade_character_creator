<script lang="ts">
  import { onMount } from "svelte";

  interface Props {
    title: string;
    description: string;
    placeholder: string;
    submitLabel?: string;
    value: string;
    onCancel: () => void;
    onSubmit: () => void;
  }

  let {
    title,
    description,
    placeholder,
    submitLabel = "Submit",
    value = $bindable(""),
    onCancel,
    onSubmit
  }: Props = $props();

  let inputElement: HTMLInputElement;

  onMount(() => {
    // Focus the input when modal opens
    inputElement?.focus();
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onCancel();
    }
  }

  function handleInputKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && value.trim()) {
      onSubmit();
    }
  }
</script>

<div
  class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
  onclick={onCancel}
  onkeydown={handleKeydown}
  role="dialog"
  aria-modal="true"
  aria-labelledby="text-input-modal-title"
  tabindex="-1"
>
  <div
    class="bg-white dark:bg-zinc-800 rounded-lg shadow-xl p-6 max-w-md mx-4 w-full"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
    role="none"
  >
    <h3 id="text-input-modal-title" class="text-lg font-semibold text-zinc-900 dark:text-zinc-100 mb-2">
      {title}
    </h3>
    <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-4">
      {description}
    </p>
    <input
      bind:this={inputElement}
      type="text"
      bind:value={value}
      {placeholder}
      class="w-full px-3 py-2 rounded-md border border-zinc-300 dark:border-zinc-600 bg-white dark:bg-zinc-700 text-zinc-900 dark:text-zinc-100 placeholder-zinc-400 focus:outline-none focus:ring-2 focus:ring-blue-500 mb-4"
      onkeydown={handleInputKeydown}
    />
    <div class="flex justify-end gap-3">
      <button
        onclick={onCancel}
        class="px-4 py-2 text-sm font-medium text-zinc-700 dark:text-zinc-300 hover:bg-zinc-100 dark:hover:bg-zinc-700 rounded-md transition-colors"
      >
        Cancel
      </button>
      <button
        onclick={onSubmit}
        disabled={!value.trim()}
        class="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-500 disabled:bg-zinc-400 disabled:cursor-not-allowed rounded-md transition-colors"
      >
        {submitLabel}
      </button>
    </div>
  </div>
</div>
