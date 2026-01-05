<script lang="ts">
  import StepNav from "./StepNav.svelte";
  import { getWizardContext } from "$lib/contexts/wizardContext";

  const wizard = getWizardContext();
</script>

<header class="sticky top-0 z-10 bg-white border-b border-zinc-200 dark:bg-zinc-900 dark:border-zinc-800">
  <div class="max-w-4xl mx-auto px-6 py-3 flex items-center justify-between">
    <div class="flex items-center gap-3">
      <button
        onclick={wizard.callbacks.onDiscard}
        class="text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300 transition-colors"
        aria-label="Discard and close"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
      <h1 class="text-sm font-medium text-zinc-500 dark:text-zinc-400">
        {wizard.characterName ?? "New Character"}
      </h1>
    </div>
    <button
      onclick={wizard.callbacks.onSave}
      disabled={wizard.saving || wizard.loading}
      class="bg-blue-600 hover:bg-blue-500 disabled:bg-blue-400 text-white text-sm font-medium px-3 py-1.5 rounded-md transition-colors"
    >
      {wizard.saving ? "Saving..." : "Save Character"}
    </button>
  </div>
  <div class="max-w-4xl mx-auto px-6 pt-1 pb-2">
    <StepNav />
  </div>
</header>
