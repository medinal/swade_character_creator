<script lang="ts">
  import { commands, type CharacterView, type AncestryView, type Modifier } from "$lib/bindings";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import WizardLayout from "$lib/components/WizardLayout.svelte";
  import SourceFilter from "$lib/components/SourceFilter.svelte";
  import { formatSource } from "$lib/utils/formatting";
  import {
    loadDraftOrRedirect,
    discardDraftAndNavigate,
    saveCharacter,
    saveCharacterAndGetId,
    WIZARD_STEPS,
  } from "$lib/utils/wizardOperations";

  let character = $state<CharacterView | null>(null);
  let ancestries = $state<AncestryView[]>([]);
  let loading = $state(true);
  let saving = $state(false);
  let error = $state<string | null>(null);

  let selectedAncestryId = $state<number | null>(null);
  let sourceFilter = $state("all");

  let filteredAncestries = $derived(
    sourceFilter === "all"
      ? ancestries
      : ancestries.filter(a => a.source === sourceFilter)
  );

  onMount(async () => {
    const draft = await loadDraftOrRedirect();
    if (!draft) return;

    character = draft;
    selectedAncestryId = draft.ancestry?.id ?? null;

    // Load ancestries
    const ancestriesResult = await commands.getAncestries();
    if (ancestriesResult.status === "ok") {
      ancestries = ancestriesResult.data;
    } else {
      error = ancestriesResult.error.message;
    }

    loading = false;
  });

  let selectedAncestry = $derived(
    ancestries.find(a => a.id === selectedAncestryId) ?? null
  );

  async function selectAncestry(ancestryId: number | null) {
    saving = true;
    error = null;

    const result = await commands.updateDraftAncestry(ancestryId);
    if (result.status === "ok") {
      character = result.data;
      selectedAncestryId = ancestryId;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function handleBack() {
    goto(WIZARD_STEPS.basics);
  }

  async function handleDiscard() {
    const err = await discardDraftAndNavigate();
    if (err) error = err;
  }

  async function handleSave() {
    saving = true;
    error = null;

    const result = await saveCharacter();
    if (result.ok) {
      character = result.data;
    } else {
      error = result.error;
    }
    saving = false;
  }

  async function handleSaveBeforeSheet(): Promise<number | null> {
    saving = true;
    error = null;

    const { id, error: err } = await saveCharacterAndGetId();
    saving = false;

    if (id !== null) {
      return id;
    }
    error = err;
    return null;
  }

  async function handleContinue() {
    goto(WIZARD_STEPS.hindrances);
  }

  function formatModifier(mod: Modifier): string {
    return mod.description;
  }

  async function selectChoiceOption(choiceId: number, optionId: number | null) {
    saving = true;
    error = null;

    const result = await commands.updateDraftAncestryChoice(choiceId, optionId);
    if (result.status === "ok") {
      character = result.data;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  function getChoiceTypeLabel(choiceType: string): string {
    switch (choiceType) {
      case "free_edge": return "Free Edge";
      case "mandatory_hindrance": return "Mandatory Hindrance";
      case "ancestral_enemy": return "Ancestral Enemy";
      default: return "Choice";
    }
  }
</script>

<WizardLayout
  characterName={character?.name}
  characterId={character?.id ?? 0}
  currentStep={2}
  {saving}
  {loading}
  {error}
  onDiscard={handleDiscard}
  onSave={handleSave}
  onSaveBeforeSheet={handleSaveBeforeSheet}
  onBack={handleBack}
  onContinue={handleContinue}
>
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
    <!-- Ancestry List -->
    <div>
      <div class="mb-4">
        <div class="flex items-center justify-between mb-1">
          <h2 class="text-lg font-semibold text-zinc-900 dark:text-zinc-100">Select Ancestry</h2>
          <SourceFilter value={sourceFilter} onChange={(v) => sourceFilter = v} />
        </div>
        <p class="text-sm text-zinc-500 dark:text-zinc-400">
          Choose your character's ancestry. Each ancestry provides unique abilities and traits.
        </p>
      </div>

      <div class="space-y-2">
        <!-- No Ancestry Option -->
        <button
          onclick={() => selectAncestry(null)}
          disabled={saving}
          class="w-full text-left px-4 py-3 rounded-lg border-2 transition-all {selectedAncestryId === null
            ? 'border-blue-500 bg-blue-50 dark:bg-blue-500/10'
            : 'border-zinc-200 dark:border-zinc-700 hover:border-zinc-300 dark:hover:border-zinc-600'}"
        >
          <div class="font-medium text-zinc-900 dark:text-zinc-100">None</div>
          <p class="text-xs text-zinc-500 dark:text-zinc-400 mt-0.5">
            Skip ancestry selection for now
          </p>
        </button>

        {#each filteredAncestries as ancestry}
          <button
            onclick={() => selectAncestry(ancestry.id)}
            disabled={saving}
            class="w-full text-left px-4 py-3 rounded-lg border-2 transition-all {selectedAncestryId === ancestry.id
              ? 'border-blue-500 bg-blue-50 dark:bg-blue-500/10'
              : 'border-zinc-200 dark:border-zinc-700 hover:border-zinc-300 dark:hover:border-zinc-600'}"
          >
            <div class="flex items-center justify-between">
              <span class="font-medium text-zinc-900 dark:text-zinc-100">{ancestry.name}</span>
              <span class="text-xs text-zinc-400 dark:text-zinc-500">{formatSource(ancestry.source)}</span>
            </div>
            {#if ancestry.modifiers.length > 0 || ancestry.choices.length > 0}
              <p class="text-xs text-zinc-500 dark:text-zinc-400 mt-1">
                {#if ancestry.modifiers.length > 0}
                  {ancestry.modifiers.length} trait{ancestry.modifiers.length !== 1 ? 's' : ''}
                {/if}
                {#if ancestry.modifiers.length > 0 && ancestry.choices.length > 0}
                  &middot;
                {/if}
                {#if ancestry.choices.length > 0}
                  {ancestry.choices.length} choice{ancestry.choices.length !== 1 ? 's' : ''}
                {/if}
              </p>
            {/if}
          </button>
        {/each}
      </div>
    </div>

    <!-- Ancestry Details -->
    <div>
      {#if selectedAncestry}
        <div class="sticky top-24">
          <div class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5">
            <h3 class="text-lg font-semibold text-zinc-900 dark:text-zinc-100 mb-2">
              {selectedAncestry.name}
            </h3>
            <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-4">
              {selectedAncestry.description}
            </p>

            {#if selectedAncestry.modifiers.length > 0}
              <div class="mb-4">
                <h4 class="text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide mb-2">
                  Racial Abilities
                </h4>
                <ul class="space-y-1.5">
                  {#each selectedAncestry.modifiers as modifier}
                    <li class="text-sm text-zinc-700 dark:text-zinc-300 flex items-start gap-2">
                      <span class="text-blue-500 mt-0.5">•</span>
                      <span>{formatModifier(modifier)}</span>
                    </li>
                  {/each}
                </ul>
              </div>
            {/if}

            {#if character?.ancestry_choices && character.ancestry_choices.length > 0}
              <div>
                <h4 class="text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide mb-3">
                  Choices to Make
                </h4>
                <div class="space-y-4">
                  {#each character.ancestry_choices as choiceValue}
                    <div class="border border-zinc-200 dark:border-zinc-700 rounded-lg p-3">
                      <div class="flex items-center gap-2 mb-2">
                        <span class="text-xs font-medium px-1.5 py-0.5 rounded bg-amber-100 text-amber-700 dark:bg-amber-500/20 dark:text-amber-400">
                          {getChoiceTypeLabel(choiceValue.choice.choice_type)}
                        </span>
                      </div>
                      <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-2">
                        {choiceValue.choice.description}
                      </p>
                      {#if choiceValue.choice.options.length > 0}
                        <select
                          onchange={(e) => {
                            const target = e.target as HTMLSelectElement;
                            const value = target.value;
                            selectChoiceOption(choiceValue.choice.id, value ? parseInt(value) : null);
                          }}
                          disabled={saving}
                          class="w-full px-3 py-2 bg-white dark:bg-zinc-900 border border-zinc-300 dark:border-zinc-600 rounded-md text-sm text-zinc-900 dark:text-zinc-100 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent disabled:opacity-50"
                        >
                          <option value="">Select an option...</option>
                          {#each choiceValue.choice.options as option}
                            <option
                              value={option.id}
                              selected={choiceValue.selected_option?.id === option.id}
                            >
                              {option.description ?? `Option ${option.id}`}
                            </option>
                          {/each}
                        </select>
                      {:else}
                        <p class="text-xs text-zinc-500 dark:text-zinc-500 italic">
                          No predefined options - make your selection during play
                        </p>
                      {/if}
                    </div>
                  {/each}
                </div>
              </div>
            {/if}
          </div>
        </div>
      {:else}
        <div class="bg-zinc-50 dark:bg-zinc-800/50 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5 text-center">
          <p class="text-sm text-zinc-500 dark:text-zinc-400">
            Select an ancestry to see its details
          </p>
        </div>
      {/if}
    </div>
  </div>
</WizardLayout>
