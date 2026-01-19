<script lang="ts">
  import {
    commands,
    type CharacterView,
    type ArcaneBackgroundWithAvailability,
    type PowerWithAvailability,
    type ArcaneBackgroundChoiceType,
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

  let arcaneBackgrounds = $state<ArcaneBackgroundWithAvailability[]>([]);
  let powers = $state<PowerWithAvailability[]>([]);
  let loading = $state(true);
  let saving = $state(false);
  let error = $state<string | null>(null);
  let warnings = $state<string[]>([]);

  // Track which power is expanded
  let expandedPower = $state<number | null>(null);
  let expandedArcaneBackground = $state<number | null>(null);

  // Hide locked toggle
  let hideLocked = $state(false);
  let sourceFilter = $state("all");

  // Check if character has an arcane background type
  let hasArcaneBackgroundSelected = $derived(
    (character?.arcane_backgrounds.length ?? 0) > 0
  );

  // Selected power IDs
  let selectedPowerIds = $derived(
    new Set(character?.powers.map((p) => p.power.id) ?? [])
  );

  // Filtered powers based on source filter and hideLocked toggle
  let filteredPowers = $derived.by(() => {
    let result = powers;

    // Filter by source
    if (sourceFilter !== "all") {
      result = result.filter(p => p.power.source === sourceFilter);
    }

    // Filter by locked status if enabled
    if (hideLocked) {
      result = result.filter(powerData => {
        const isSelected = selectedPowerIds.has(powerData.power.id);
        return powerData.is_available || isSelected;
      });
    }

    return result;
  });

  // Filtered arcane backgrounds by source
  let filteredArcaneBackgrounds = $derived(
    sourceFilter === "all"
      ? arcaneBackgrounds
      : arcaneBackgrounds.filter(ab => ab.arcane_background.source === sourceFilter)
  );

  onMount(async () => {
    // Load arcane backgrounds and powers in parallel
    const [abResult, powersResult] = await Promise.all([
      commands.getArcaneBackgrounds(),
      commands.getPowers(),
    ]);

    if (abResult.status === "ok") {
      arcaneBackgrounds = abResult.data;
    } else {
      error = abResult.error.message;
    }

    if (powersResult.status === "ok") {
      powers = powersResult.data;
    } else {
      error = powersResult.error.message;
    }

    loading = false;
  });

  function togglePowerExpand(powerId: number) {
    expandedPower = expandedPower === powerId ? null : powerId;
  }

  function toggleArcaneBackgroundExpand(abId: number) {
    expandedArcaneBackground = expandedArcaneBackground === abId ? null : abId;
  }

  async function addArcaneBackground(abId: number) {
    saving = true;
    error = null;

    // Use bypass_validation = true for edit mode
    const result = await commands.addDraftArcaneBackground(abId, true);
    if (result.status === "ok") {
      onCharacterChanged(result.data.character, result.data.warnings.map(w => w.message));
      warnings = result.data.warnings.map(w => w.message);
      // Refresh powers to update availability
      const powersResult = await commands.getPowers();
      if (powersResult.status === "ok") {
        powers = powersResult.data;
      }
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function removeArcaneBackground(abId: number) {
    saving = true;
    error = null;

    const result = await commands.removeDraftArcaneBackground(abId);
    if (result.status === "ok") {
      onCharacterChanged(result.data, []);
      // Refresh powers to update availability
      const powersResult = await commands.getPowers();
      if (powersResult.status === "ok") {
        powers = powersResult.data;
      }
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function addPower(powerId: number) {
    saving = true;
    error = null;

    // Use bypass_validation = true for edit mode
    const result = await commands.addDraftPower(powerId, true);
    if (result.status === "ok") {
      onCharacterChanged(result.data.character, result.data.warnings.map(w => w.message));
      warnings = result.data.warnings.map(w => w.message);
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function removePower(powerId: number) {
    saving = true;
    error = null;

    const result = await commands.removeDraftPower(powerId);
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
  aria-labelledby="power-browser-title"
>
  <div class="bg-white dark:bg-zinc-800 rounded-lg shadow-xl max-w-4xl w-full max-h-[85vh] flex flex-col">
    <!-- Header -->
    <div class="flex items-center justify-between px-6 py-4 border-b border-zinc-200 dark:border-zinc-700">
      <h2 id="power-browser-title" class="text-lg font-semibold text-zinc-900 dark:text-zinc-100">
        Edit Powers & Arcane Backgrounds
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
        <div class="text-center text-zinc-500 py-8">Loading...</div>
      {:else}
        <div class="space-y-6">
          {#if warnings.length > 0}
            <div class="bg-amber-50 dark:bg-amber-500/10 border border-amber-300 dark:border-amber-500/30 p-3 rounded-md">
              <p class="font-medium text-amber-800 dark:text-amber-400 text-sm">Warnings:</p>
              <ul class="text-sm text-amber-700 dark:text-amber-300 mt-1 list-disc list-inside">
                {#each warnings as warning}
                  <li>{warning}</li>
                {/each}
              </ul>
            </div>
          {/if}

          {#if error}
            <div class="bg-red-50 dark:bg-red-500/10 border border-red-200 dark:border-red-500/20 text-red-600 dark:text-red-400 px-4 py-3 rounded-md text-sm">
              {error}
            </div>
          {/if}

          <div class="flex items-center justify-between">
            <SourceFilter value={sourceFilter} onChange={(v) => sourceFilter = v} />
            <label class="flex items-center gap-2 text-sm text-zinc-600 dark:text-zinc-400 cursor-pointer">
              <input
                type="checkbox"
                bind:checked={hideLocked}
                class="w-4 h-4 rounded border-zinc-300 dark:border-zinc-600 text-blue-600 focus:ring-blue-500 dark:bg-zinc-700"
              />
              Hide Locked
            </label>
          </div>

          <!-- Current Arcane Backgrounds -->
          {#if hasArcaneBackgroundSelected}
            <div>
              <h3 class="text-sm font-semibold text-zinc-700 dark:text-zinc-300 uppercase tracking-wide mb-2">
                Current Arcane Backgrounds
              </h3>
              <div class="space-y-2">
                {#each character.arcane_backgrounds as abValue}
                  <div class="flex items-center justify-between p-3 bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg">
                    <div>
                      <span class="font-medium text-purple-800 dark:text-purple-200">
                        {abValue.arcane_background.name}
                      </span>
                      <span class="text-xs text-purple-600 dark:text-purple-400 ml-2">
                        {abValue.arcane_background.starting_powers} powers, {abValue.arcane_background.starting_power_points} PP
                      </span>
                    </div>
                    <button
                      onclick={() => removeArcaneBackground(abValue.arcane_background.id)}
                      disabled={saving}
                      class="text-xs text-red-600 hover:text-red-500 dark:text-red-400 dark:hover:text-red-300 disabled:opacity-50"
                    >
                      Remove
                    </button>
                  </div>
                {/each}
              </div>
            </div>
          {:else}
            <!-- Add Arcane Background -->
            <div>
              <h3 class="text-sm font-semibold text-zinc-700 dark:text-zinc-300 uppercase tracking-wide mb-2">
                Add Arcane Background
              </h3>
              <div class="space-y-2">
                {#each filteredArcaneBackgrounds as abData}
                  {@const isExpanded = expandedArcaneBackground === abData.arcane_background.id}
                  <div class="rounded-lg border border-zinc-200 dark:border-zinc-700 overflow-hidden">
                    <button
                      onclick={() => toggleArcaneBackgroundExpand(abData.arcane_background.id)}
                      class="w-full px-4 py-3 flex items-center justify-between gap-2 text-left hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors"
                    >
                      <div class="flex items-center gap-2">
                        <span class="font-medium text-sm text-zinc-900 dark:text-zinc-100">
                          {abData.arcane_background.name}
                        </span>
                        <span class="text-xs text-zinc-500 dark:text-zinc-400 bg-zinc-100 dark:bg-zinc-800 px-2 py-0.5 rounded">
                          {abData.arcane_background.starting_powers} powers, {abData.arcane_background.starting_power_points} PP
                        </span>
                      </div>
                      <svg
                        class="w-4 h-4 text-zinc-400 transition-transform {isExpanded ? 'rotate-180' : ''}"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                      >
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                      </svg>
                    </button>

                    {#if isExpanded}
                      <div class="px-4 pb-4 border-t border-zinc-100 dark:border-zinc-800">
                        <p class="text-sm text-zinc-600 dark:text-zinc-400 mt-3 mb-4">
                          {abData.arcane_background.description}
                        </p>
                        <button
                          onclick={() => addArcaneBackground(abData.arcane_background.id)}
                          disabled={saving}
                          class="px-4 py-2 text-sm font-medium rounded-md bg-blue-600 text-white hover:bg-blue-500 disabled:opacity-50 transition-colors"
                        >
                          Add {abData.arcane_background.name}
                        </button>
                      </div>
                    {/if}
                  </div>
                {/each}
              </div>
            </div>
          {/if}

          <!-- Powers -->
          <div>
            <h3 class="text-sm font-semibold text-zinc-700 dark:text-zinc-300 uppercase tracking-wide mb-2">
              Powers
            </h3>
            <div class="space-y-2">
              {#each filteredPowers as powerData}
                {@const isExpanded = expandedPower === powerData.power.id}
                {@const isSelected = selectedPowerIds.has(powerData.power.id)}
                <div
                  class="rounded-lg border border-zinc-200 dark:border-zinc-700 overflow-hidden {isSelected
                    ? 'bg-blue-50 dark:bg-blue-900/20'
                    : ''}"
                >
                  <button
                    onclick={() => togglePowerExpand(powerData.power.id)}
                    class="w-full px-4 py-3 flex items-center justify-between gap-2 text-left hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors"
                  >
                    <div class="flex items-center gap-2 min-w-0">
                      <span class="font-medium text-sm text-zinc-900 dark:text-zinc-100">
                        {powerData.power.name}
                      </span>
                      <span class="text-xs text-zinc-500 dark:text-zinc-400 bg-zinc-100 dark:bg-zinc-800 px-2 py-0.5 rounded">
                        {powerData.power.power_points} PP
                      </span>
                      {#if isSelected}
                        <span class="px-1.5 py-0.5 text-xs rounded bg-blue-100 text-blue-700 dark:bg-blue-500/20 dark:text-blue-400">
                          Selected
                        </span>
                      {/if}
                    </div>
                    <svg
                      class="w-4 h-4 text-zinc-400 transition-transform {isExpanded ? 'rotate-180' : ''}"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                    </svg>
                  </button>

                  {#if isExpanded}
                    <div class="px-4 pb-4 border-t border-zinc-100 dark:border-zinc-800">
                      <!-- Power Stats -->
                      <div class="text-xs text-zinc-500 dark:text-zinc-400 space-y-1 mt-3 mb-3">
                        <div class="flex gap-4">
                          <span><strong>Range:</strong> {powerData.power.range}</span>
                          <span><strong>Duration:</strong> {powerData.power.duration}</span>
                          <span><strong>Cost:</strong> {powerData.power.power_points} PP</span>
                        </div>
                      </div>

                      <!-- Description -->
                      <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-4">
                        {powerData.power.description}
                      </p>

                      <!-- Add/Remove button -->
                      <div class="flex items-center justify-between">
                        <span class="text-xs text-zinc-500 dark:text-zinc-500">
                          {formatSource(powerData.power.source)}
                        </span>
                        {#if isSelected}
                          {@const powerValue = character?.powers.find(p => p.power.id === powerData.power.id)}
                          {#if powerValue?.is_locked}
                            <span class="text-xs text-purple-600 dark:text-purple-400 italic">
                              Required (cannot remove)
                            </span>
                          {:else}
                            <button
                              onclick={() => removePower(powerData.power.id)}
                              disabled={saving}
                              class="px-3 py-1.5 text-sm font-medium rounded-md bg-red-100 text-red-700 hover:bg-red-200 dark:bg-red-500/20 dark:text-red-400 dark:hover:bg-red-500/30 disabled:opacity-50 transition-colors"
                            >
                              Remove Power
                            </button>
                          {/if}
                        {:else}
                          <button
                            onclick={() => addPower(powerData.power.id)}
                            disabled={saving}
                            class="px-3 py-1.5 text-sm font-medium rounded-md bg-blue-600 text-white hover:bg-blue-500 disabled:opacity-50 transition-colors"
                          >
                            Add Power
                          </button>
                        {/if}
                      </div>
                    </div>
                  {/if}
                </div>
              {/each}
            </div>
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
