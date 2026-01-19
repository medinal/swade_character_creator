<script lang="ts">
  import {
    type CharacterView,
    type EdgeWithAvailability,
    type EdgeCategory,
    type GameConfig,
  } from "$lib/bindings";
  import TextInputModal from "$lib/components/TextInputModal.svelte";
  import SourceFilter from "$lib/components/SourceFilter.svelte";
  import { formatSource } from "$lib/utils/formatting";

  interface Props {
    character: CharacterView;
    edges: EdgeWithAvailability[];
    gameConfig: GameConfig | null;
    // Callbacks
    onAddEdge: (edgeId: number, notes: string | null) => Promise<void>;
    onRemoveEdge: (edgeId: number) => Promise<void>;
    // Mode flags
    bypassRequirements?: boolean;
    // State
    saving: boolean;
    error: string | null;
    warnings?: string[];
  }

  let {
    character,
    edges,
    gameConfig,
    onAddEdge,
    onRemoveEdge,
    bypassRequirements = false,
    saving,
    error,
    warnings = [],
  }: Props = $props();

  // Track which edge is expanded
  let expandedEdge = $state<number | null>(null);

  // Track which category is collapsed
  let collapsedCategories = $state<Set<EdgeCategory>>(new Set());

  // Hide locked edges toggle
  let hideLocked = $state(false);

  // Source filter
  let sourceFilter = $state("all");

  // Modal for notes input (for edges that can be taken multiple times)
  let notesModal = $state<{ edgeId: number; edgeName: string } | null>(null);
  let notesInput = $state("");

  // Filter edges by source
  let filteredEdges = $derived(
    sourceFilter === "all"
      ? edges
      : edges.filter(e => e.edge.source === sourceFilter)
  );

  const categoryOrder: EdgeCategory[] = [
    "Background",
    "Combat",
    "Leadership",
    "Power",
    "Professional",
    "Social",
    "Weird",
  ];

  // Group edges by category
  let edgesByCategory = $derived.by(() => {
    const map = new Map<EdgeCategory, EdgeWithAvailability[]>();
    for (const category of categoryOrder) {
      map.set(category, []);
    }
    for (const edgeData of filteredEdges) {
      const category = edgeData.edge.category;
      const existing = map.get(category) ?? [];
      existing.push(edgeData);
      map.set(category, existing);
    }
    // Sort edges alphabetically within each category
    for (const [key, categoryEdges] of map) {
      map.set(
        key,
        categoryEdges.sort((a, b) => a.edge.name.localeCompare(b.edge.name))
      );
    }
    return map;
  });

  // Selected edge IDs (for quick lookup)
  let selectedEdgeIds = $derived(
    new Set(character?.edges.map((e) => e.edge.id) ?? [])
  );

  // Cost per edge from game config
  let edgeCost = $derived(gameConfig?.hindrance_points_per_edge ?? 2);

  function isEdgeSelected(edgeId: number): boolean {
    return selectedEdgeIds.has(edgeId);
  }

  function toggleCategoryCollapse(category: EdgeCategory) {
    const newSet = new Set(collapsedCategories);
    if (newSet.has(category)) {
      newSet.delete(category);
    } else {
      newSet.add(category);
    }
    collapsedCategories = newSet;
  }

  function toggleEdgeExpand(edgeId: number) {
    expandedEdge = expandedEdge === edgeId ? null : edgeId;
  }

  function handleAddClick(edgeData: EdgeWithAvailability) {
    if (edgeData.edge.can_take_multiple_times) {
      // Open modal to get notes
      notesModal = { edgeId: edgeData.edge.id, edgeName: edgeData.edge.name };
      notesInput = "";
    } else {
      onAddEdge(edgeData.edge.id, null);
    }
  }

  function handleNotesSubmit() {
    if (notesModal && notesInput.trim()) {
      onAddEdge(notesModal.edgeId, notesInput.trim());
      notesModal = null;
      notesInput = "";
    }
  }

  function closeNotesModal() {
    notesModal = null;
    notesInput = "";
  }

  // In bypass mode, edges are always available for purchase
  function canAddEdge(edgeData: EdgeWithAvailability, isSelected: boolean): boolean {
    if (bypassRequirements) {
      // In edit mode: can add if not already selected (or can take multiple times)
      return edgeData.edge.can_take_multiple_times || !isSelected;
    }
    // Normal mode: check availability
    return edgeData.is_available && (edgeData.edge.can_take_multiple_times || !isSelected);
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

  <div class="flex items-start justify-between gap-4">
    <div class="flex items-center gap-4">
      <SourceFilter value={sourceFilter} onChange={(v) => sourceFilter = v} />
      <label class="flex items-center gap-2 text-sm text-zinc-600 dark:text-zinc-400 cursor-pointer whitespace-nowrap">
        <input
          type="checkbox"
          bind:checked={hideLocked}
          class="w-4 h-4 rounded border-zinc-300 dark:border-zinc-600 text-blue-600 focus:ring-blue-500 dark:bg-zinc-700"
        />
        Hide Locked
      </label>
    </div>
  </div>

  {#each categoryOrder as category}
    {@const categoryEdges = edgesByCategory.get(category) ?? []}
    {@const filteredCategoryEdges = hideLocked
      ? categoryEdges.filter(edgeData => {
          const isSelected = selectedEdgeIds.has(edgeData.edge.id);
          // Show if: available (or bypass mode), OR already selected
          return bypassRequirements || edgeData.is_available || isSelected;
        })
      : categoryEdges}
    {#if filteredCategoryEdges.length > 0}
      {@const isCollapsed = collapsedCategories.has(category)}
      <div class="rounded-lg border border-zinc-200 dark:border-zinc-700 overflow-hidden">
        <!-- Category Header -->
        <button
          onclick={() => toggleCategoryCollapse(category)}
          class="w-full px-4 py-3 flex items-center justify-between bg-zinc-100 dark:bg-zinc-800 hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors"
        >
          <div class="flex items-center gap-3">
            <span class="font-semibold text-zinc-900 dark:text-zinc-100">
              {category}
            </span>
            <span class="text-xs text-zinc-500 dark:text-zinc-400">
              ({filteredCategoryEdges.length} edge{filteredCategoryEdges.length !== 1 ? "s" : ""})
            </span>
          </div>
          <svg
            class="w-4 h-4 text-zinc-400 transition-transform {isCollapsed ? '' : 'rotate-180'}"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
          </svg>
        </button>

        <!-- Edges in this category -->
        {#if !isCollapsed}
          <div class="divide-y divide-zinc-100 dark:divide-zinc-800">
            {#each filteredCategoryEdges as edgeData}
              {@const isExpanded = expandedEdge === edgeData.edge.id}
              {@const isSelected = isEdgeSelected(edgeData.edge.id)}
              {@const selectedEdgeValue = character?.edges.find(e => e.edge.id === edgeData.edge.id)}
              {@const isFromAncestry = selectedEdgeValue?.source === "ancestry"}
              {@const canAdd = canAddEdge(edgeData, isSelected)}
              <div
                class="{isSelected ? 'bg-blue-50 dark:bg-blue-900/20' : ''} {!bypassRequirements && !edgeData.is_available ? 'opacity-60' : ''}"
              >
                <!-- Edge Header -->
                <button
                  onclick={() => toggleEdgeExpand(edgeData.edge.id)}
                  class="w-full px-4 py-3 flex items-center justify-between gap-2 text-left hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors"
                >
                  <div class="flex items-center gap-2 min-w-0">
                    <span
                      class="font-medium text-sm {bypassRequirements || edgeData.is_available
                        ? 'text-zinc-900 dark:text-zinc-100'
                        : 'text-zinc-500 dark:text-zinc-500'}"
                    >
                      {edgeData.edge.name}
                    </span>
                    {#if edgeData.edge.can_take_multiple_times}
                      <span class="px-1.5 py-0.5 text-xs rounded bg-purple-100 text-purple-700 dark:bg-purple-500/20 dark:text-purple-400">
                        Multi
                      </span>
                    {/if}
                    {#if isSelected}
                      <span class="px-1.5 py-0.5 text-xs rounded bg-blue-100 text-blue-700 dark:bg-blue-500/20 dark:text-blue-400">
                        Selected
                      </span>
                    {/if}
                    {#if !bypassRequirements && !edgeData.is_available}
                      <span class="px-1.5 py-0.5 text-xs rounded bg-zinc-200 text-zinc-600 dark:bg-zinc-700 dark:text-zinc-400">
                        Locked
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

                <!-- Expanded content -->
                {#if isExpanded}
                  <div class="px-4 pb-4 border-t border-zinc-100 dark:border-zinc-800">
                    <!-- Requirements -->
                    {#if edgeData.requirement_statuses.length > 0}
                      <div class="mt-3 mb-3">
                        <span class="text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide">
                          Requirements:
                        </span>
                        <div class="flex flex-wrap gap-1 mt-1">
                          {#each edgeData.requirement_statuses as req}
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

                    <!-- Description -->
                    <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-4">
                      {edgeData.edge.description}
                    </p>

                    <!-- Modifiers -->
                    {#if edgeData.edge.modifiers.length > 0}
                      <div class="mb-4">
                        <span class="text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide">
                          Effects:
                        </span>
                        <ul class="mt-1 text-xs text-zinc-600 dark:text-zinc-400 space-y-0.5">
                          {#each edgeData.edge.modifiers as mod}
                            <li>&bull; {mod.description}</li>
                          {/each}
                        </ul>
                      </div>
                    {/if}

                    <!-- Add/Remove button -->
                    <div class="flex items-center justify-between">
                      <span class="text-xs text-zinc-500 dark:text-zinc-500">
                        {formatSource(edgeData.edge.source)}
                      </span>
                      {#if isSelected && !edgeData.edge.can_take_multiple_times && selectedEdgeValue?.source === "hindrance_points"}
                        <button
                          onclick={() => onRemoveEdge(edgeData.edge.id)}
                          disabled={saving}
                          class="px-3 py-1.5 text-sm font-medium rounded-md bg-red-100 text-red-700 hover:bg-red-200 dark:bg-red-500/20 dark:text-red-400 dark:hover:bg-red-500/30 disabled:opacity-50 transition-colors"
                        >
                          Remove Edge
                        </button>
                      {:else if isSelected && !edgeData.edge.can_take_multiple_times}
                        <span class="text-xs text-zinc-500 dark:text-zinc-400 italic">
                          Granted by {formatSource(selectedEdgeValue?.source ?? "")}
                        </span>
                      {:else if canAdd}
                        <button
                          onclick={() => handleAddClick(edgeData)}
                          disabled={saving}
                          class="px-3 py-1.5 text-sm font-medium rounded-md bg-blue-600 text-white hover:bg-blue-500 disabled:opacity-50 transition-colors"
                        >
                          {#if bypassRequirements}
                            Add Edge
                          {:else}
                            Add Edge ({edgeCost} pts)
                          {/if}
                        </button>
                      {:else if !bypassRequirements && !edgeData.is_available}
                        <span class="text-xs text-zinc-500 dark:text-zinc-400 italic">
                          Requirements not met
                        </span>
                      {/if}
                    </div>
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  {/each}
</div>

{#if notesModal}
  <TextInputModal
    title={notesModal.edgeName}
    description="This edge can be taken multiple times. Please specify what this instance applies to (e.g., a specific skill, weapon, or other detail)."
    placeholder="e.g., Fighting, Longsword, Persuasion..."
    submitLabel="Add Edge"
    bind:value={notesInput}
    onCancel={closeNotesModal}
    onSubmit={handleNotesSubmit}
  />
{/if}
