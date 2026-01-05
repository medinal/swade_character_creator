<script lang="ts">
  import { goto } from "$app/navigation";
  import { getWizardContext } from "$lib/contexts/wizardContext";

  const wizard = getWizardContext();

  let showSavePrompt = $state(false);
  let savingForSheet = $state(false);

  const steps = [
    { num: 1, label: "Basic Info", path: "/create" },
    { num: 2, label: "Ancestry", path: "/create/ancestry" },
    { num: 3, label: "Hindrances", path: "/create/hindrances" },
    { num: 4, label: "Attributes", path: "/create/attributes" },
    { num: 5, label: "Skills", path: "/create/skills" },
    { num: 6, label: "Edges", path: "/create/edges" },
    { num: 7, label: "Powers", path: "/create/powers" },
  ];

  async function navigateTo(path: string) {
    if (wizard.callbacks.onNavigate) {
      await wizard.callbacks.onNavigate();
    }
    goto(path);
  }

  async function navigateToSheet() {
    // Always prompt to save when in wizard (onSaveBeforeSheet is provided)
    if (wizard.callbacks.onSaveBeforeSheet) {
      showSavePrompt = true;
    } else if (wizard.characterId && wizard.characterId > 0) {
      // No save callback - just navigate if we have an ID
      if (wizard.callbacks.onNavigate) {
        await wizard.callbacks.onNavigate();
      }
      goto(`/character/${wizard.characterId}`);
    }
  }

  async function handleSaveAndViewSheet() {
    if (!wizard.callbacks.onSaveBeforeSheet) return;

    savingForSheet = true;
    const savedId = await wizard.callbacks.onSaveBeforeSheet();
    savingForSheet = false;
    showSavePrompt = false;

    if (savedId && savedId > 0) {
      goto(`/character/${savedId}`);
    }
  }

  function cancelSavePrompt() {
    showSavePrompt = false;
  }
</script>

<div>
  <div class="flex items-center gap-1.5 text-xs text-zinc-500 dark:text-zinc-400">
    {#each steps as step, i}
      <button
        onclick={() => navigateTo(step.path)}
        class="flex items-center gap-1.5 hover:text-zinc-700 dark:hover:text-zinc-300 transition-colors group"
      >
        <span
          class="px-2 py-0.5 rounded-full font-medium transition-colors {wizard.currentStep === step.num
            ? 'bg-blue-600 text-white'
            : 'bg-zinc-200 dark:bg-zinc-700 text-zinc-600 dark:text-zinc-400 group-hover:bg-zinc-300 dark:group-hover:bg-zinc-600'}"
        >
          {step.num}
        </span>
        <span
          class="transition-colors {wizard.currentStep === step.num
            ? 'font-medium text-zinc-900 dark:text-zinc-100'
            : 'group-hover:text-zinc-700 dark:group-hover:text-zinc-300'}"
        >
          {step.label}
        </span>
      </button>
      {#if i < steps.length - 1}
        <span class="text-zinc-300 dark:text-zinc-700">—</span>
      {/if}
    {/each}

    <!-- Character Sheet link -->
    <span class="text-zinc-300 dark:text-zinc-700">—</span>
    <button
      onclick={navigateToSheet}
      disabled={!wizard.callbacks.onSaveBeforeSheet && (!wizard.characterId || wizard.characterId === 0)}
      class="flex items-center gap-1.5 transition-colors {(wizard.characterId && wizard.characterId > 0) || wizard.callbacks.onSaveBeforeSheet
        ? 'hover:text-zinc-700 dark:hover:text-zinc-300'
        : 'opacity-50 cursor-not-allowed'} group"
    >
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
      </svg>
      <span class={(wizard.characterId && wizard.characterId > 0) || wizard.callbacks.onSaveBeforeSheet ? 'group-hover:text-zinc-700 dark:group-hover:text-zinc-300' : ''}>
        Sheet
      </span>
    </button>
  </div>

</div>

<!-- Save prompt modal -->
{#if showSavePrompt}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <div class="bg-white dark:bg-zinc-800 rounded-lg shadow-xl p-6 max-w-sm mx-4">
      <h3 class="text-lg font-semibold text-zinc-900 dark:text-zinc-100 mb-2">
        Save Character?
      </h3>
      <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-4">
        Your character needs to be saved before viewing the character sheet. Would you like to save now?
      </p>
      <div class="flex justify-end gap-3">
        <button
          onclick={cancelSavePrompt}
          disabled={savingForSheet}
          class="px-4 py-2 text-sm font-medium text-zinc-700 dark:text-zinc-300 hover:bg-zinc-100 dark:hover:bg-zinc-700 rounded-md transition-colors"
        >
          Cancel
        </button>
        <button
          onclick={handleSaveAndViewSheet}
          disabled={savingForSheet}
          class="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-500 disabled:bg-blue-400 rounded-md transition-colors"
        >
          {savingForSheet ? 'Saving...' : 'Save & View Sheet'}
        </button>
      </div>
    </div>
  </div>
{/if}
