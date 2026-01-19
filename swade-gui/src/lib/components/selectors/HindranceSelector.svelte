<script lang="ts">
  import { type CharacterView, type HindranceView, type GameConfig } from "$lib/bindings";
  import SourceFilter from "$lib/components/SourceFilter.svelte";
  import { SEVERITY_STYLES, STATUS_STYLES } from "$lib/utils/badgeStyles";
  import { formatSource } from "$lib/utils/formatting";

  interface Props {
    character: CharacterView;
    hindrances: HindranceView[];
    gameConfig: GameConfig | null;
    onAddHindrance: (hindranceId: number) => Promise<void>;
    onRemoveHindrance: (hindranceId: number) => Promise<void>;
    // Mode flags
    bypassPointLimit?: boolean;
    // State
    saving: boolean;
    error: string | null;
    warnings?: string[];
  }

  let {
    character,
    hindrances,
    gameConfig,
    onAddHindrance,
    onRemoveHindrance,
    bypassPointLimit = false,
    saving,
    error,
    warnings = [],
  }: Props = $props();

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
    if (bypassPointLimit) return true;
    return pointsEarned + hindrance.point_value <= maxHindrancePoints;
  }

  async function addHindrance(group: HindranceGroup) {
    const hindrance = getViewingHindrance(group);
    if (!hindrance) return;
    await onAddHindrance(hindrance.id);
  }

  async function removeHindrance(group: HindranceGroup) {
    const selectedSeverity = getSelectedSeverity(group);
    if (!selectedSeverity) return;

    const hindrance = selectedSeverity === "major" ? group.major : group.minor;
    if (!hindrance) return;
    await onRemoveHindrance(hindrance.id);
  }
</script>

<div class="space-y-4">
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
