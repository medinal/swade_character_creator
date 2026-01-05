<script lang="ts">
  import { getWizardContext } from "$lib/contexts/wizardContext";

  const wizard = getWizardContext();
</script>

<footer class="sticky bottom-0 bg-white border-t border-zinc-200 dark:bg-zinc-900 dark:border-zinc-800">
  <div class="max-w-4xl mx-auto px-6 py-4 flex justify-between">
    {#if wizard.callbacks.onBack}
      <button
        onclick={wizard.callbacks.onBack}
        class="text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300 text-sm font-medium transition-colors flex items-center gap-2"
      >
        {#if wizard.showBackArrow}
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
          </svg>
        {/if}
        {wizard.backLabel}
      </button>
    {:else}
      <div></div>
    {/if}
    <button
      onclick={wizard.callbacks.onContinue}
      disabled={wizard.saving}
      class="bg-blue-600 hover:bg-blue-500 disabled:bg-blue-400 text-white text-sm font-medium px-4 py-2 rounded-md transition-colors flex items-center gap-2"
    >
      {wizard.saving ? "Saving..." : wizard.continueLabel}
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    </button>
  </div>
</footer>
