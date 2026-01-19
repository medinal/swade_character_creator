<script lang="ts">
  import {
    commands,
    type CharacterView,
    type AncestryView,
    type Modifier,
  } from "$lib/bindings";
  import { onMount } from "svelte";
  import SourceFilter from "./SourceFilter.svelte";
  import { formatSource } from "$lib/utils/formatting";

  interface Props {
    character: CharacterView;
    onClose: () => void;
    onCharacterChanged: (character: CharacterView, warnings: string[]) => void;
  }

  let { character, onClose, onCharacterChanged }: Props = $props();

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
    selectedAncestryId = character.ancestry?.id ?? null;

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
      onCharacterChanged(result.data, []);
      selectedAncestryId = ancestryId;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function selectChoiceOption(choiceId: number, optionId: number | null) {
    saving = true;
    error = null;

    const result = await commands.updateDraftAncestryChoice(choiceId, optionId);
    if (result.status === "ok") {
      onCharacterChanged(result.data, []);
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  function formatModifier(mod: Modifier): string {
    return mod.description;
  }

  function getChoiceTypeLabel(choiceType: string): string {
    switch (choiceType) {
      case "free_edge": return "Free Edge";
      case "mandatory_hindrance": return "Mandatory Hindrance";
      case "ancestral_enemy": return "Ancestral Enemy";
      default: return "Choice";
    }
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
  aria-labelledby="ancestry-browser-title"
>
  <div class="bg-white dark:bg-zinc-800 rounded-lg shadow-xl max-w-4xl w-full max-h-[85vh] flex flex-col">
    <!-- Header -->
    <div class="flex items-center justify-between px-6 py-4 border-b border-zinc-200 dark:border-zinc-700">
      <h2 id="ancestry-browser-title" class="text-lg font-semibold text-zinc-900 dark:text-zinc-100">
        Change Ancestry
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
        <div class="text-center text-zinc-500 py-8">Loading ancestries...</div>
      {:else}
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <!-- Ancestry List -->
          <div>
            {#if error}
              <div class="bg-red-50 dark:bg-red-500/10 border border-red-200 dark:border-red-500/20 text-red-600 dark:text-red-400 px-4 py-3 rounded-md text-sm mb-4">
                {error}
              </div>
            {/if}

            <div class="flex items-center justify-between mb-4">
              <h3 class="text-sm font-semibold text-zinc-700 dark:text-zinc-300 uppercase tracking-wide">
                Select Ancestry
              </h3>
              <SourceFilter value={sourceFilter} onChange={(v) => sourceFilter = v} />
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
                  No ancestry selected
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
              <div class="sticky top-0">
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
                            <span class="text-blue-500 mt-0.5">&bull;</span>
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
