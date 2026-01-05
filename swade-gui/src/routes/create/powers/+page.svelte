<script lang="ts">
  import {
    commands,
    type CharacterView,
    type ArcaneBackgroundWithAvailability,
    type PowerWithAvailability,
    type ArcaneBackgroundChoiceType,
  } from "$lib/bindings";
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
  let arcaneBackgrounds = $state<ArcaneBackgroundWithAvailability[]>([]);
  let powers = $state<PowerWithAvailability[]>([]);
  let loading = $state(true);
  let saving = $state(false);
  let error = $state<string | null>(null);

  // Track which power is expanded
  let expandedPower = $state<number | null>(null);

  // Track which arcane background is expanded
  let expandedArcaneBackground = $state<number | null>(null);

  // Hide locked powers toggle
  let hideLocked = $state(false);

  // Source filter
  let sourceFilter = $state("all");

  // Check if character has the Arcane Background edge
  let hasArcaneBackgroundEdge = $derived(
    character?.edges.some((e) => e.edge.name === "Arcane Background") ?? false
  );

  // Check if character has selected an arcane background type
  let hasArcaneBackgroundSelected = $derived(
    (character?.arcane_backgrounds.length ?? 0) > 0
  );

  // Calculate power slots (base from arcane backgrounds + bonus from edges like "New Powers")
  let basePowerSlots = $derived(
    character?.arcane_backgrounds.reduce(
      (sum, ab) => sum + ab.arcane_background.starting_powers,
      0
    ) ?? 0
  );
  let bonusPowerSlots = $derived(
    character?.edges.reduce((sum, edgeVal) => {
      return sum + edgeVal.edge.modifiers
        .filter(m => m.target_type === 'power_slots' && m.value !== null)
        .reduce((s, m) => s + (m.value ?? 0), 0);
    }, 0) ?? 0
  );
  let totalStartingPowers = $derived(basePowerSlots + bonusPowerSlots);
  let currentPowerCount = $derived(character?.powers.length ?? 0);
  let powersRemaining = $derived(totalStartingPowers - currentPowerCount);

  // Power points tracking (base from character + bonus from edges like "Power Points")
  let basePowerPoints = $derived(character?.power_points ?? 0);
  let bonusPowerPoints = $derived(
    character?.edges.reduce((sum, edgeVal) => {
      return sum + edgeVal.edge.modifiers
        .filter(m => m.target_type === 'power_points' && m.value !== null)
        .reduce((s, m) => s + (m.value ?? 0), 0);
    }, 0) ?? 0
  );
  let totalPowerPoints = $derived(basePowerPoints + bonusPowerPoints);

  onMount(async () => {
    const draft = await loadDraftOrRedirect();
    if (!draft) return;

    character = draft;

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

  // Selected arcane background IDs
  let selectedArcaneBackgroundIds = $derived(
    new Set(character?.arcane_backgrounds.map((ab) => ab.arcane_background.id) ?? [])
  );

  // Selected power IDs
  let selectedPowerIds = $derived(
    new Set(character?.powers.map((p) => p.power.id) ?? [])
  );

  // Check if any arcane background has a power list restriction
  let hasPowerListRestriction = $derived(
    character?.arcane_backgrounds.some(ab => ab.arcane_background.has_power_list) ?? false
  );

  // Get available power IDs from arcane background choices
  let availablePowerIds = $derived.by(() => {
    if (!hasPowerListRestriction) return null;
    const ids = new Set<number>();
    for (const choiceValue of character?.arcane_background_choices ?? []) {
      if (choiceValue.choice.choice_type === "available_power") {
        for (const option of choiceValue.choice.options) {
          if (option.option_type === "power" && option.option_id !== null) {
            ids.add(option.option_id);
          }
        }
      }
    }
    return ids;
  });

  // Filtered powers based on source filter, hideLocked toggle, and available_power restriction
  let filteredPowers = $derived.by(() => {
    // First filter by available powers if has_power_list
    let result = powers;
    if (availablePowerIds !== null) {
      result = result.filter(p => availablePowerIds.has(p.power.id));
    }

    // Then filter by source
    if (sourceFilter !== "all") {
      result = result.filter(p => p.power.source === sourceFilter);
    }

    // Then filter by locked status if enabled
    if (hideLocked) {
      result = result.filter(powerData => {
        const isSelected = selectedPowerIds.has(powerData.power.id);
        // Show if available OR already selected
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

  function togglePowerExpand(powerId: number) {
    expandedPower = expandedPower === powerId ? null : powerId;
  }

  function toggleArcaneBackgroundExpand(abId: number) {
    expandedArcaneBackground = expandedArcaneBackground === abId ? null : abId;
  }

  async function addArcaneBackground(abId: number) {
    saving = true;
    error = null;

    const result = await commands.addDraftArcaneBackground(abId);
    if (result.status === "ok") {
      character = result.data;
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
      character = result.data;
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

    const result = await commands.addDraftPower(powerId);
    if (result.status === "ok") {
      character = result.data;
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
      character = result.data;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function updateArcaneBackgroundChoice(choiceId: number, optionId: number, isSelecting: boolean) {
    saving = true;
    error = null;

    const result = await commands.updateDraftArcaneBackgroundChoice(choiceId, optionId, isSelecting);
    if (result.status === "ok") {
      character = result.data;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  function getChoiceTypeLabel(choiceType: ArcaneBackgroundChoiceType): string {
    switch (choiceType) {
      case "available_power": return "Available Powers";
      case "required_starting_power": return "Required Power";
      case "choosable_starting_power": return "Choose Starting Powers";
      case "built_in_hindrance": return "Built-in Hindrance";
      case "special_ability": return "Special Ability";
      case "edge_category": return "Edge Access";
      default: return "Choice";
    }
  }

  function getChoiceTypeBadgeColor(choiceType: ArcaneBackgroundChoiceType): string {
    switch (choiceType) {
      case "built_in_hindrance": return "bg-red-100 text-red-700 dark:bg-red-500/20 dark:text-red-400";
      case "required_starting_power": return "bg-purple-100 text-purple-700 dark:bg-purple-500/20 dark:text-purple-400";
      case "choosable_starting_power": return "bg-blue-100 text-blue-700 dark:bg-blue-500/20 dark:text-blue-400";
      case "special_ability": return "bg-amber-100 text-amber-700 dark:bg-amber-500/20 dark:text-amber-400";
      case "edge_category": return "bg-green-100 text-green-700 dark:bg-green-500/20 dark:text-green-400";
      default: return "bg-zinc-100 text-zinc-700 dark:bg-zinc-500/20 dark:text-zinc-400";
    }
  }

  // Check if an option is selected in a choice
  function isOptionSelected(choiceId: number, optionId: number): boolean {
    const choiceValue = character?.arcane_background_choices.find(c => c.choice.id === choiceId);
    return choiceValue?.selected_options.some(o => o.id === optionId) ?? false;
  }

  // Get count of selected options for a choice
  function getSelectedCount(choiceId: number): number {
    const choiceValue = character?.arcane_background_choices.find(c => c.choice.id === choiceId);
    return choiceValue?.selected_options.length ?? 0;
  }

  async function handleBack() {
    goto(WIZARD_STEPS.edges);
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
    // Powers is the last step - save and go to character sheet
    const id = await handleSaveBeforeSheet();
    if (id) {
      goto(`/character/${id}`);
    }
  }
</script>

<WizardLayout
  characterName={character?.name}
  characterId={character?.id ?? 0}
  currentStep={7}
  {saving}
  {loading}
  {error}
  onDiscard={handleDiscard}
  onSave={handleSave}
  onSaveBeforeSheet={handleSaveBeforeSheet}
  onBack={handleBack}
  onContinue={handleContinue}
  continueLabel="Finish"
>
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
    <!-- Main Content -->
    <div class="lg:col-span-2">
      <div class="mb-4">
        <div class="flex items-center justify-between mb-1">
          <h2 class="text-lg font-semibold text-zinc-900 dark:text-zinc-100">
            Powers
            <span class="text-sm font-normal text-zinc-400 dark:text-zinc-500"
              >(optional)</span
            >
          </h2>
          <SourceFilter value={sourceFilter} onChange={(v) => sourceFilter = v} />
        </div>
        <p class="text-sm text-zinc-500 dark:text-zinc-400">
          Select an Arcane Background and choose your starting powers.
        </p>
      </div>

      {#if !hasArcaneBackgroundEdge}
        <!-- No Arcane Background Edge -->
        <div
          class="bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800 rounded-lg p-6 text-center"
        >
          <svg
            class="w-12 h-12 mx-auto mb-4 text-amber-500 dark:text-amber-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <h3
            class="text-lg font-semibold text-amber-800 dark:text-amber-200 mb-2"
          >
            Powers Not Available
          </h3>
          <p class="text-sm text-amber-700 dark:text-amber-300 mb-4">
            To select powers, your character must first take the <strong
              >Arcane Background</strong
            > edge. You can add this edge on the Edges step.
          </p>
          <button
            onclick={() => goto("/create/edges")}
            class="px-4 py-2 text-sm font-medium text-amber-800 dark:text-amber-200 bg-amber-100 dark:bg-amber-800/50 hover:bg-amber-200 dark:hover:bg-amber-700/50 rounded-md transition-colors"
          >
            Go to Edges
          </button>
        </div>
      {:else if !hasArcaneBackgroundSelected}
        <!-- Has Edge but no Arcane Background selected -->
        <div class="space-y-4">
          <div
            class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4 mb-4"
          >
            <p class="text-sm text-blue-700 dark:text-blue-300">
              You have the Arcane Background edge! Select which type of arcane
              background below.
            </p>
          </div>

          <h3
            class="text-sm font-semibold text-zinc-700 dark:text-zinc-300 uppercase tracking-wide"
          >
            Select Arcane Background Type
          </h3>

          <div class="space-y-2">
            {#each filteredArcaneBackgrounds as abData}
              {@const isExpanded =
                expandedArcaneBackground === abData.arcane_background.id}
              <div
                class="rounded-lg border border-zinc-200 dark:border-zinc-700 overflow-hidden"
              >
                <button
                  onclick={() =>
                    toggleArcaneBackgroundExpand(abData.arcane_background.id)}
                  class="w-full px-4 py-3 flex items-center justify-between gap-2 text-left hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors"
                >
                  <div class="flex items-center gap-2">
                    <span
                      class="font-medium text-sm text-zinc-900 dark:text-zinc-100"
                    >
                      {abData.arcane_background.name}
                    </span>
                    <span
                      class="text-xs text-zinc-500 dark:text-zinc-400 bg-zinc-100 dark:bg-zinc-800 px-2 py-0.5 rounded"
                    >
                      {abData.arcane_background.starting_powers} powers, {abData
                        .arcane_background.starting_power_points} PP
                    </span>
                  </div>
                  <svg
                    class="w-4 h-4 text-zinc-400 transition-transform {isExpanded
                      ? 'rotate-180'
                      : ''}"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M19 9l-7 7-7-7"
                    />
                  </svg>
                </button>

                {#if isExpanded}
                  <div
                    class="px-4 pb-4 border-t border-zinc-100 dark:border-zinc-800"
                  >
                    <p
                      class="text-sm text-zinc-600 dark:text-zinc-400 mt-3 mb-4"
                    >
                      {abData.arcane_background.description}
                    </p>

                    <div
                      class="text-xs text-zinc-500 dark:text-zinc-400 space-y-1 mb-4"
                    >
                      <div class="flex justify-between">
                        <span>Arcane Skill:</span>
                        <span class="font-medium"
                          >{abData.arcane_background.arcane_skill_name ??
                            "Unknown"}</span
                        >
                      </div>
                      <div class="flex justify-between">
                        <span>Starting Powers:</span>
                        <span class="font-medium"
                          >{abData.arcane_background.starting_powers}</span
                        >
                      </div>
                      <div class="flex justify-between">
                        <span>Starting Power Points:</span>
                        <span class="font-medium"
                          >{abData.arcane_background
                            .starting_power_points}</span
                        >
                      </div>
                    </div>

                    <button
                      onclick={() =>
                        addArcaneBackground(abData.arcane_background.id)}
                      disabled={saving}
                      class="px-4 py-2 text-sm font-medium rounded-md bg-blue-600 text-white hover:bg-blue-500 disabled:opacity-50 transition-colors"
                    >
                      Select {abData.arcane_background.name}
                    </button>
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        </div>
      {:else}
        <!-- Has Arcane Background - show powers -->
        <div class="space-y-4">
          <!-- Selected Arcane Background Info -->
          {#each character?.arcane_backgrounds ?? [] as abValue}
            <div
              class="bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-4 mb-4"
            >
              <div class="flex items-center justify-between">
                <div>
                  <span
                    class="text-sm font-semibold text-purple-800 dark:text-purple-200"
                  >
                    {abValue.arcane_background.name}
                  </span>
                  <p class="text-xs text-purple-600 dark:text-purple-400 mt-1">
                    Skill: {abValue.arcane_background.arcane_skill_name ??
                      "Unknown"}
                    | {abValue.arcane_background.starting_powers} starting powers
                    | {abValue.arcane_background.starting_power_points} PP
                  </p>
                </div>
                <button
                  onclick={() =>
                    removeArcaneBackground(abValue.arcane_background.id)}
                  disabled={saving}
                  class="text-xs text-red-600 hover:text-red-500 dark:text-red-400 dark:hover:text-red-300 disabled:opacity-50"
                >
                  Change
                </button>
              </div>
            </div>
          {/each}

          <!-- Arcane Background Choices -->
          {#if character?.arcane_background_choices && character.arcane_background_choices.length > 0}
            {@const displayableChoices = character.arcane_background_choices
              .filter(c => c.choice.choice_type !== "available_power")
              .sort((a, b) => {
                // Sort order: special_ability, edge_category, built_in_hindrance first; power choices last
                const order: Record<string, number> = {
                  special_ability: 0,
                  edge_category: 1,
                  built_in_hindrance: 2,
                  required_starting_power: 3,
                  choosable_starting_power: 4,
                };
                return (order[a.choice.choice_type] ?? 99) - (order[b.choice.choice_type] ?? 99);
              })}
            {#if displayableChoices.length > 0}
              <div class="space-y-3 mb-4">
                {#each displayableChoices as choiceValue}
                  {@const choiceType = choiceValue.choice.choice_type}
                  {@const selectedCount = getSelectedCount(choiceValue.choice.id)}

                  <div class="border border-zinc-200 dark:border-zinc-700 rounded-lg p-3">
                    <div class="flex items-center gap-2 mb-2">
                      <span class="text-xs font-medium px-1.5 py-0.5 rounded {getChoiceTypeBadgeColor(choiceType)}">
                        {getChoiceTypeLabel(choiceType)}
                      </span>
                      {#if choiceType === "choosable_starting_power"}
                        <span class="text-xs text-zinc-500 dark:text-zinc-400">
                          ({selectedCount}/{choiceValue.choice.max_selections} selected)
                        </span>
                      {/if}
                    </div>

                    {#if choiceType === "built_in_hindrance"}
                      <!-- Built-in hindrance: show choice description only (hindrance name is in the description) -->
                      <p class="text-sm text-zinc-600 dark:text-zinc-400">
                        {choiceValue.choice.description}
                      </p>
                    {:else if choiceType === "required_starting_power"}
                      <!-- Required starting powers: show description and power names -->
                      <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-2">
                        {choiceValue.choice.description}
                      </p>
                      <div class="flex flex-wrap gap-1.5">
                        {#each choiceValue.selected_options as option}
                          {@const powerName = character?.powers.find(p => p.power.id === option.option_id)?.power.name}
                          <span class="px-2 py-1 text-xs rounded-md bg-purple-100 text-purple-700 dark:bg-purple-500/20 dark:text-purple-400">
                            {powerName ?? option.description ?? `Power`}
                            <span class="ml-1 opacity-60">(Required)</span>
                          </span>
                        {/each}
                      </div>
                    {:else if choiceType === "special_ability"}
                      <!-- Special ability: just show the description (no duplicate from options) -->
                      <p class="text-sm text-zinc-700 dark:text-zinc-300">
                        {choiceValue.choice.description}
                      </p>
                    {:else if choiceType === "edge_category"}
                      <!-- Edge category: show what edge categories are accessible -->
                      <p class="text-sm text-zinc-600 dark:text-zinc-400">
                        {choiceValue.choice.description}
                      </p>
                    {:else if choiceType === "choosable_starting_power"}
                      <!-- Multi-select for choosable powers -->
                      <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-2">
                        {choiceValue.choice.description}
                      </p>
                      <div class="space-y-1.5">
                        {#each choiceValue.choice.options as option}
                          {@const isSelected = isOptionSelected(choiceValue.choice.id, option.id)}
                          {@const canSelect = !isSelected && selectedCount < choiceValue.choice.max_selections}
                          {@const powerName = powers.find(p => p.power.id === option.option_id)?.power.name}
                          <button
                            onclick={() => updateArcaneBackgroundChoice(choiceValue.choice.id, option.id, !isSelected)}
                            disabled={saving || (!isSelected && !canSelect)}
                            class="w-full text-left px-3 py-2 rounded-md border transition-all text-sm
                              {isSelected
                                ? 'border-blue-500 bg-blue-50 dark:bg-blue-500/10 text-zinc-900 dark:text-zinc-100'
                                : canSelect
                                  ? 'border-zinc-200 dark:border-zinc-700 hover:border-zinc-300 dark:hover:border-zinc-600 text-zinc-700 dark:text-zinc-300'
                                  : 'border-zinc-200 dark:border-zinc-700 opacity-50 cursor-not-allowed text-zinc-500 dark:text-zinc-500'}"
                          >
                            <div class="flex items-center justify-between">
                              <span>{powerName ?? option.description ?? `Power`}</span>
                              {#if isSelected}
                                <span class="text-xs text-blue-600 dark:text-blue-400">Selected</span>
                              {/if}
                            </div>
                          </button>
                        {/each}
                      </div>
                    {/if}
                  </div>
                {/each}
              </div>
            {/if}
          {/if}

          <div class="flex items-center justify-between gap-4">
            <h3
              class="text-sm font-semibold text-zinc-700 dark:text-zinc-300 uppercase tracking-wide"
            >
              Select Powers ({currentPowerCount}/{totalStartingPowers})
            </h3>
            <label class="flex items-center gap-2 text-sm text-zinc-600 dark:text-zinc-400 cursor-pointer whitespace-nowrap">
              <input
                type="checkbox"
                bind:checked={hideLocked}
                class="w-4 h-4 rounded border-zinc-300 dark:border-zinc-600 text-blue-600 focus:ring-blue-500 dark:bg-zinc-700"
              />
              Hide Locked
            </label>
          </div>

          <!-- Powers List -->
          <div class="space-y-2">
            {#each filteredPowers as powerData}
              {@const isExpanded = expandedPower === powerData.power.id}
              {@const isSelected = selectedPowerIds.has(powerData.power.id)}
              {@const canAdd =
                powerData.is_available && !isSelected && powersRemaining > 0}
              <div
                class="rounded-lg border border-zinc-200 dark:border-zinc-700 overflow-hidden {isSelected
                  ? 'bg-blue-50 dark:bg-blue-900/20'
                  : ''} {!powerData.is_available ? 'opacity-60' : ''}"
              >
                <button
                  onclick={() => togglePowerExpand(powerData.power.id)}
                  class="w-full px-4 py-3 flex items-center justify-between gap-2 text-left hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors"
                >
                  <div class="flex items-center gap-2 min-w-0">
                    <span
                      class="font-medium text-sm {powerData.is_available
                        ? 'text-zinc-900 dark:text-zinc-100'
                        : 'text-zinc-500 dark:text-zinc-500'}"
                    >
                      {powerData.power.name}
                    </span>
                    <span
                      class="text-xs text-zinc-500 dark:text-zinc-400 bg-zinc-100 dark:bg-zinc-800 px-2 py-0.5 rounded"
                    >
                      {powerData.power.power_points} PP
                    </span>
                    {#if isSelected}
                      {@const powerValue = character?.powers.find(p => p.power.id === powerData.power.id)}
                      <span
                        class="px-1.5 py-0.5 text-xs rounded {powerValue?.is_locked
                          ? 'bg-purple-100 text-purple-700 dark:bg-purple-500/20 dark:text-purple-400'
                          : 'bg-blue-100 text-blue-700 dark:bg-blue-500/20 dark:text-blue-400'}"
                      >
                        {powerValue?.is_locked ? 'Required' : 'Selected'}
                      </span>
                    {/if}
                    {#if !powerData.is_available}
                      <span
                        class="px-1.5 py-0.5 text-xs rounded bg-zinc-200 text-zinc-600 dark:bg-zinc-700 dark:text-zinc-400"
                      >
                        Locked
                      </span>
                    {/if}
                  </div>
                  <svg
                    class="w-4 h-4 text-zinc-400 transition-transform {isExpanded
                      ? 'rotate-180'
                      : ''}"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M19 9l-7 7-7-7"
                    />
                  </svg>
                </button>

                {#if isExpanded}
                  <div
                    class="px-4 pb-4 border-t border-zinc-100 dark:border-zinc-800"
                  >
                    <!-- Requirements -->
                    {#if powerData.requirement_statuses.length > 0}
                      <div class="mt-3 mb-3">
                        <span
                          class="text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide"
                          >Requirements:</span
                        >
                        <div class="flex flex-wrap gap-1 mt-1">
                          {#each powerData.requirement_statuses as req}
                            <span
                              class="px-2 py-0.5 text-xs rounded {req.is_met
                                ? 'bg-green-100 text-green-700 dark:bg-green-500/20 dark:text-green-400'
                                : 'bg-red-100 text-red-700 dark:bg-red-500/20 dark:text-red-400'}"
                            >
                              {#if req.is_met}
                                <span class="mr-1">&#10003;</span>
                              {:else}
                                <span class="mr-1">&#10007;</span>
                              {/if}
                              {req.description}
                            </span>
                          {/each}
                        </div>
                      </div>
                    {/if}

                    <!-- Power Stats -->
                    <div
                      class="text-xs text-zinc-500 dark:text-zinc-400 space-y-1 mb-3"
                    >
                      <div class="flex gap-4">
                        <span
                          ><strong>Range:</strong>
                          {powerData.power.range}</span
                        >
                        <span
                          ><strong>Duration:</strong>
                          {powerData.power.duration}</span
                        >
                        <span
                          ><strong>Cost:</strong>
                          {powerData.power.power_points} PP</span
                        >
                      </div>
                    </div>

                    <!-- Description -->
                    <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-4">
                      {powerData.power.description}
                    </p>

                    <!-- Modifiers -->
                    {#if powerData.power.modifiers.length > 0}
                      <div class="mb-4">
                        <span
                          class="text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide"
                          >Effects:</span
                        >
                        <ul
                          class="mt-1 text-xs text-zinc-600 dark:text-zinc-400 space-y-0.5"
                        >
                          {#each powerData.power.modifiers as mod}
                            <li>• {mod.description}</li>
                          {/each}
                        </ul>
                      </div>
                    {/if}

                    <!-- Add/Remove button -->
                    <div class="flex items-center justify-between">
                      <span class="text-xs text-zinc-500 dark:text-zinc-500">
                        {formatSource(powerData.power.source)}
                      </span>
                      {#if isSelected}
                        {@const powerValue = character?.powers.find(p => p.power.id === powerData.power.id)}
                        {#if powerValue?.is_locked}
                          <span class="text-xs text-purple-600 dark:text-purple-400 italic">
                            Required starting power (cannot remove)
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
                      {:else if canAdd}
                        <button
                          onclick={() => addPower(powerData.power.id)}
                          disabled={saving}
                          class="px-3 py-1.5 text-sm font-medium rounded-md bg-blue-600 text-white hover:bg-blue-500 disabled:opacity-50 transition-colors"
                        >
                          Add Power
                        </button>
                      {:else if !powerData.is_available}
                        <span
                          class="text-xs text-zinc-500 dark:text-zinc-400 italic"
                        >
                          Requirements not met
                        </span>
                      {:else if powersRemaining <= 0}
                        <span
                          class="text-xs text-zinc-500 dark:text-zinc-400 italic"
                        >
                          No power slots remaining
                        </span>
                      {/if}
                    </div>
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>

    <!-- Sidebar -->
    <div>
      <div class="sticky top-24 space-y-4">
        <!-- Power Points Summary -->
        <div
          class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5"
        >
          <h3
            class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 mb-3"
          >
            Power Points
          </h3>

          {#if hasArcaneBackgroundSelected}
            <div class="text-2xl font-bold text-purple-600 dark:text-purple-400">
              {totalPowerPoints} PP
            </div>
            <p class="text-xs text-zinc-500 dark:text-zinc-400 mt-1">
              Available to cast powers
            </p>
          {:else}
            <p class="text-sm text-zinc-500 dark:text-zinc-400 italic">
              Select an arcane background to gain power points.
            </p>
          {/if}
        </div>

        <!-- Selected Powers -->
        <div
          class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5"
        >
          <h3
            class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 mb-3"
          >
            Selected Powers ({currentPowerCount}/{totalStartingPowers})
          </h3>

          {#if character?.powers && character.powers.length > 0}
            <ul class="space-y-2">
              {#each character.powers as powerValue}
                <li class="flex items-start justify-between text-sm">
                  <div>
                    <span class="text-zinc-700 dark:text-zinc-300"
                      >{powerValue.power.name}</span
                    >
                    <span class="text-xs text-zinc-500 dark:text-zinc-400 block">
                      {powerValue.power.power_points} PP
                      {#if powerValue.is_locked}
                        <span class="text-purple-600 dark:text-purple-400">(Required)</span>
                      {/if}
                    </span>
                  </div>
                  {#if powerValue.is_locked}
                    <span class="text-xs text-zinc-400 dark:text-zinc-500 italic">
                      Locked
                    </span>
                  {:else}
                    <button
                      onclick={() => removePower(powerValue.power.id)}
                      disabled={saving}
                      class="text-xs text-red-600 hover:text-red-500 dark:text-red-400 dark:hover:text-red-300 disabled:opacity-50"
                    >
                      Remove
                    </button>
                  {/if}
                </li>
              {/each}
            </ul>
          {:else if hasArcaneBackgroundSelected}
            <p class="text-sm text-zinc-500 dark:text-zinc-400 italic">
              No powers selected
            </p>
          {:else}
            <p class="text-sm text-zinc-500 dark:text-zinc-400 italic">
              Select an arcane background first.
            </p>
          {/if}
        </div>

        <!-- Quick Reference -->
        <div
          class="bg-zinc-50 dark:bg-zinc-800/50 rounded-lg border border-zinc-200 dark:border-zinc-700 p-4"
        >
          <h4
            class="text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide mb-2"
          >
            Quick Reference
          </h4>
          <ul class="text-xs text-zinc-600 dark:text-zinc-400 space-y-1">
            <li>
              <span class="text-green-600 dark:text-green-400">&#10003;</span> = Requirement
              met
            </li>
            <li>
              <span class="text-red-600 dark:text-red-400">&#10007;</span> = Requirement
              not met
            </li>
            <li>
              <span
                class="px-1 py-0.5 text-xs rounded bg-zinc-200 text-zinc-600 dark:bg-zinc-700 dark:text-zinc-400"
                >PP</span
              > = Power Points cost
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</WizardLayout>
