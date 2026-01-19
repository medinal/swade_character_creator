<script lang="ts">
  import { commands, type CharacterView, type HindranceView, type GameConfig } from "$lib/bindings";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import WizardLayout from "$lib/components/WizardLayout.svelte";
  import SourceFilter from "$lib/components/SourceFilter.svelte";
  import { SEVERITY_STYLES, STATUS_STYLES } from "$lib/utils/badgeStyles";
  import { formatSource } from "$lib/utils/formatting";
  import {
    loadDraftOrRedirect,
    discardDraftAndNavigate,
    saveCharacter,
    saveCharacterAndGetId,
    WIZARD_STEPS,
  } from "$lib/utils/wizardOperations";

  let character = $state<CharacterView | null>(null);
  let hindrances = $state<HindranceView[]>([]);
  let gameConfig = $state<GameConfig | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let error = $state<string | null>(null);

  let sourceFilter = $state("all");

  let maxHindrancePoints = $derived(gameConfig?.max_hindrance_points ?? 4);

  let filteredHindrances = $derived(
    sourceFilter === "all"
      ? hindrances
      : hindrances.filter(h => h.source === sourceFilter)
  );

  type HindranceGroup = {
    name: string;
    minor: HindranceView | null;
    major: HindranceView | null;
  };

  // Track which hindrance is expanded and which severity is being viewed
  let expandedGroup = $state<string | null>(null);
  let viewingSeverity = $state<Map<string, "minor" | "major">>(new Map());

  onMount(async () => {
    const draft = await loadDraftOrRedirect();
    if (!draft) return;

    character = draft;

    // Load hindrances and game config in parallel
    const [hindrancesResult, config] = await Promise.all([
      commands.getHindrances(),
      commands.getGameConfig()
    ]);

    if (hindrancesResult.status === "ok") {
      hindrances = hindrancesResult.data;
    } else {
      error = hindrancesResult.error.message;
    }

    gameConfig = config;

    loading = false;
  });

  // Group hindrances by name, pairing major/minor variants
  let hindranceGroups = $derived.by(() => {
    const groups = new Map<string, HindranceGroup>();
    const processed = new Set<number>();

    for (const h of filteredHindrances) {
      if (processed.has(h.id)) continue;

      // Find companion if exists
      let companion: HindranceView | null = null;
      if (h.companion_hindrance_id) {
        companion = filteredHindrances.find(c => c.id === h.companion_hindrance_id) ?? null;
        if (companion) processed.add(companion.id);
      }
      processed.add(h.id);

      const group: HindranceGroup = {
        name: h.name,
        minor: h.severity === "minor" ? h : (companion?.severity === "minor" ? companion : null),
        major: h.severity === "major" ? h : (companion?.severity === "major" ? companion : null),
      };

      groups.set(h.name, group);
    }

    // Sort by name
    return Array.from(groups.values()).sort((a, b) => a.name.localeCompare(b.name));
  });

  let selectedHindranceIds = $derived(
    new Set(character?.hindrances.map(h => h.hindrance.id) ?? [])
  );

  // Track which hindrances are from ancestry (not removable)
  let ancestryHindranceIds = $derived(
    new Set(character?.hindrances.filter(h => h.source === "ancestry").map(h => h.hindrance.id) ?? [])
  );

  let pointsEarned = $derived(character?.hindrance_points_earned ?? 0);

  function getSelectedSeverity(group: HindranceGroup): "minor" | "major" | null {
    if (group.minor && selectedHindranceIds.has(group.minor.id)) return "minor";
    if (group.major && selectedHindranceIds.has(group.major.id)) return "major";
    return null;
  }

  function isAncestryHindrance(group: HindranceGroup): boolean {
    if (group.minor && ancestryHindranceIds.has(group.minor.id)) return true;
    if (group.major && ancestryHindranceIds.has(group.major.id)) return true;
    return false;
  }

  function getViewingSeverity(group: HindranceGroup): "minor" | "major" {
    const explicit = viewingSeverity.get(group.name);
    if (explicit) return explicit;
    // Default to selected severity, or major if available, otherwise minor
    const selected = getSelectedSeverity(group);
    if (selected) return selected;
    return group.major ? "major" : "minor";
  }

  function getViewingHindrance(group: HindranceGroup): HindranceView | null {
    const severity = getViewingSeverity(group);
    return severity === "major" ? group.major : group.minor;
  }

  function toggleGroupExpand(groupName: string) {
    expandedGroup = expandedGroup === groupName ? null : groupName;
  }

  function setViewingSeverity(groupName: string, severity: "minor" | "major") {
    const newMap = new Map(viewingSeverity);
    newMap.set(groupName, severity);
    viewingSeverity = newMap;
  }

  function canAddHindrance(hindrance: HindranceView): boolean {
    return pointsEarned + hindrance.point_value <= maxHindrancePoints;
  }

  async function addHindrance(group: HindranceGroup) {
    const hindrance = getViewingHindrance(group);
    if (!hindrance) return;

    saving = true;
    error = null;

    const result = await commands.addDraftHindrance(hindrance.id, null);
    if (result.status === "ok") {
      character = result.data.character;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function removeHindrance(group: HindranceGroup) {
    const selectedSeverity = getSelectedSeverity(group);
    if (!selectedSeverity) return;

    const hindrance = selectedSeverity === "major" ? group.major : group.minor;
    if (!hindrance) return;

    saving = true;
    error = null;

    const result = await commands.removeDraftHindrance(hindrance.id);
    if (result.status === "ok") {
      character = result.data;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function handleBack() {
    goto(WIZARD_STEPS.ancestry);
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
    goto(WIZARD_STEPS.attributes);
  }
</script>

<WizardLayout
  characterName={character?.name}
  characterId={character?.id ?? 0}
  currentStep={3}
  {saving}
  {loading}
  {error}
  onDiscard={handleDiscard}
  onSave={handleSave}
  onSaveBeforeSheet={handleSaveBeforeSheet}
  onBack={handleBack}
  onContinue={handleContinue}
>
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
    <!-- Hindrance List -->
    <div class="lg:col-span-2">
      <div class="mb-4">
        <div class="flex items-center justify-between mb-1">
          <h2 class="text-lg font-semibold text-zinc-900 dark:text-zinc-100">
            Select Hindrances
            <span class="text-sm font-normal text-zinc-400 dark:text-zinc-500">(optional)</span>
          </h2>
          <SourceFilter value={sourceFilter} onChange={(v) => sourceFilter = v} />
        </div>
        <p class="text-sm text-zinc-500 dark:text-zinc-400">
          Choose hindrances to earn points for edges, attributes, or skills. You can earn up to {maxHindrancePoints} points.
        </p>
      </div>

      <div class="space-y-1">
        {#each hindranceGroups as group}
          {@const selectedSeverity = getSelectedSeverity(group)}
          {@const isFromAncestry = isAncestryHindrance(group)}
          {@const isExpanded = expandedGroup === group.name}
          {@const hasBothVariants = group.minor && group.major}
          {@const viewingHindrance = getViewingHindrance(group)}
          {@const currentViewingSeverity = getViewingSeverity(group)}
          <div
            class="rounded-lg border transition-all {selectedSeverity
              ? (selectedSeverity === 'major' ? 'border-red-400 dark:border-red-500/50' : 'border-amber-400 dark:border-amber-500/50')
              : 'border-zinc-200 dark:border-zinc-700'}"
          >
            <!-- Collapsed header - clickable to expand -->
            <button
              onclick={() => toggleGroupExpand(group.name)}
              class="w-full px-3 py-2 flex items-center justify-between gap-2 text-left hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors rounded-lg"
            >
              <div class="flex items-center gap-2 min-w-0">
                <span class="font-medium text-sm text-zinc-900 dark:text-zinc-100">{group.name}</span>
                {#if hasBothVariants}
                  <span class="inline-flex text-xs rounded overflow-hidden">
                    <span class="px-1 py-0.5 {SEVERITY_STYLES.minor}">Minor</span>
                    <span class="px-1 py-0.5 {SEVERITY_STYLES.major}">Major</span>
                  </span>
                {:else}
                  <span class="px-1.5 py-0.5 text-xs rounded {group.major ? SEVERITY_STYLES.major : SEVERITY_STYLES.minor}">
                    {group.major ? 'Major' : 'Minor'}
                  </span>
                {/if}
                {#if selectedSeverity}
                  {#if isFromAncestry}
                    <span class="px-1.5 py-0.5 text-xs rounded bg-purple-100 text-purple-700 dark:bg-purple-500/20 dark:text-purple-400">
                      From Ancestry
                    </span>
                  {:else}
                    <span class="px-1.5 py-0.5 text-xs rounded {STATUS_STYLES.info}">
                      Selected
                    </span>
                  {/if}
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

            <!-- Expanded content -->
            {#if isExpanded && viewingHindrance}
              <div class="px-3 pb-3 border-t border-zinc-100 dark:border-zinc-800">
                <!-- Severity toggle (only for hindrances with both variants) -->
                {#if hasBothVariants}
                  <div class="flex gap-1 mt-3 mb-3">
                    <button
                      onclick={() => setViewingSeverity(group.name, 'minor')}
                      class="px-3 py-1.5 text-xs font-medium rounded-md transition-colors
                        {currentViewingSeverity === 'minor'
                          ? 'bg-amber-500 text-white'
                          : 'bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-400 hover:bg-zinc-200 dark:hover:bg-zinc-700'}"
                    >
                      Minor (1 pt)
                    </button>
                    <button
                      onclick={() => setViewingSeverity(group.name, 'major')}
                      class="px-3 py-1.5 text-xs font-medium rounded-md transition-colors
                        {currentViewingSeverity === 'major'
                          ? 'bg-red-500 text-white'
                          : 'bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-400 hover:bg-zinc-200 dark:hover:bg-zinc-700'}"
                    >
                      Major (2 pts)
                    </button>
                  </div>
                {/if}

                <!-- Description -->
                <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-4 {!hasBothVariants ? 'mt-3' : ''}">
                  {viewingHindrance.description}
                </p>

                <!-- Add/Remove button -->
                <div class="flex items-center justify-between">
                  <span class="text-xs text-zinc-500 dark:text-zinc-500">
                    {formatSource(viewingHindrance.source)}
                  </span>
                  {#if isFromAncestry && selectedSeverity}
                    <!-- Ancestry hindrance - cannot be removed -->
                    <span class="px-3 py-1.5 text-sm font-medium text-purple-600 dark:text-purple-400">
                      From {character?.ancestry?.name ?? 'Ancestry'}
                    </span>
                  {:else if selectedSeverity === currentViewingSeverity}
                    <button
                      onclick={() => removeHindrance(group)}
                      disabled={saving}
                      class="px-3 py-1.5 text-sm font-medium rounded-md bg-red-100 text-red-700 hover:bg-red-200 dark:bg-red-500/20 dark:text-red-400 dark:hover:bg-red-500/30 disabled:opacity-50 transition-colors"
                    >
                      Remove Hindrance
                    </button>
                  {:else if selectedSeverity}
                    <!-- Different severity is selected - need to switch -->
                    <button
                      onclick={() => { removeHindrance(group).then(() => addHindrance(group)); }}
                      disabled={saving || !canAddHindrance(viewingHindrance)}
                      class="px-3 py-1.5 text-sm font-medium rounded-md bg-blue-600 text-white hover:bg-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                    >
                      Switch to {currentViewingSeverity === 'major' ? 'Major' : 'Minor'} (+{viewingHindrance.point_value} pt)
                    </button>
                  {:else}
                    <button
                      onclick={() => addHindrance(group)}
                      disabled={saving || !canAddHindrance(viewingHindrance)}
                      class="px-3 py-1.5 text-sm font-medium rounded-md bg-blue-600 text-white hover:bg-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                    >
                      Add Hindrance (+{viewingHindrance.point_value} pt)
                    </button>
                  {/if}
                </div>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    </div>

    <!-- Selected Hindrances Summary -->
    <div>
      <div class="sticky top-24">
        <div class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5">
          <h3 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 mb-3">
            Hindrance Points
          </h3>

          <!-- Points Display -->
          <div class="mb-4">
            <div class="flex items-center justify-between mb-2">
              <span class="text-sm text-zinc-600 dark:text-zinc-400">Points Earned</span>
              <span class="text-lg font-semibold text-zinc-900 dark:text-zinc-100">
                {pointsEarned} / {maxHindrancePoints}
              </span>
            </div>
            <div class="w-full bg-zinc-200 dark:bg-zinc-700 rounded-full h-2">
              <div
                class="bg-blue-600 h-2 rounded-full transition-all"
                style="width: {(pointsEarned / maxHindrancePoints) * 100}%"
              ></div>
            </div>
          </div>

          <!-- Selected Hindrances List -->
          {#if character?.hindrances && character.hindrances.length > 0}
            <div class="border-t border-zinc-200 dark:border-zinc-700 pt-4">
              <h4 class="text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide mb-2">
                Selected
              </h4>
              <ul class="space-y-2">
                {#each character.hindrances as h}
                  <li class="flex items-center justify-between text-sm">
                    <div class="flex items-center gap-2">
                      <span class="px-1.5 py-0.5 text-xs rounded {h.hindrance.severity === 'major'
                        ? 'bg-red-100 text-red-700 dark:bg-red-500/20 dark:text-red-400'
                        : 'bg-amber-100 text-amber-700 dark:bg-amber-500/20 dark:text-amber-400'}">
                        {h.hindrance.severity === 'major' ? 'M' : 'm'}
                      </span>
                      <span class="text-zinc-700 dark:text-zinc-300">{h.hindrance.name}</span>
                      {#if h.source === "ancestry"}
                        <span class="px-1.5 py-0.5 text-xs rounded bg-purple-100 text-purple-700 dark:bg-purple-500/20 dark:text-purple-400">
                          Ancestry
                        </span>
                      {/if}
                    </div>
                    <span class="text-zinc-500 dark:text-zinc-400">+{h.hindrance.point_value}</span>
                  </li>
                {/each}
              </ul>
            </div>
          {:else}
            <p class="text-sm text-zinc-500 dark:text-zinc-400 italic">
              No hindrances selected
            </p>
          {/if}

          <!-- Points Usage Info -->
          <div class="border-t border-zinc-200 dark:border-zinc-700 pt-4 mt-4">
            <h4 class="text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide mb-2">
              Spend Points On
            </h4>
            <ul class="text-xs text-zinc-500 dark:text-zinc-400 space-y-1">
              <li>• 2 points = 1 Edge</li>
              <li>• 2 points = 1 Attribute step</li>
              <li>• 1 point = 1 Skill point</li>
              <li>• 1 point = Starting funds x2</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  </div>
</WizardLayout>
