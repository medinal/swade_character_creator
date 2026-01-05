<script lang="ts">
  import {
    commands,
    type CharacterView,
    type EdgeWithAvailability,
    type EdgeCategory,
    type GameConfig,
  } from "$lib/bindings";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import WizardLayout from "$lib/components/WizardLayout.svelte";
  import TextInputModal from "$lib/components/TextInputModal.svelte";
  import HindrancePointsCard from "$lib/components/HindrancePointsCard.svelte";
  import SourceFilter from "$lib/components/SourceFilter.svelte";
  import { calculateHindrancePointsAllocated, calculateHindrancePointsAvailable } from "$lib/utils/hindrancePoints";
  import { formatSource } from "$lib/utils/formatting";
  import {
    loadDraftOrRedirect,
    discardDraftAndNavigate,
    saveCharacter,
    saveCharacterAndGetId,
    WIZARD_STEPS,
  } from "$lib/utils/wizardOperations";

  let character = $state<CharacterView | null>(null);
  let edges = $state<EdgeWithAvailability[]>([]);
  let gameConfig = $state<GameConfig | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let error = $state<string | null>(null);

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

  onMount(async () => {
    const draft = await loadDraftOrRedirect();
    if (!draft) return;

    character = draft;

    // Load edges and game config in parallel
    const [edgesResult, config] = await Promise.all([
      commands.getEdges(),
      commands.getGameConfig(),
    ]);

    if (edgesResult.status === "ok") {
      edges = edgesResult.data;
    } else {
      error = edgesResult.error.message;
    }

    gameConfig = config;
    loading = false;
  });

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

  // Hindrance points tracking
  let hindrancePointsEarned = $derived(character?.hindrance_points_earned ?? 0);
  let hindrancePointsToEdges = $derived(character?.hindrance_points_to_edges ?? 0);
  let hindrancePointsAllocated = $derived(calculateHindrancePointsAllocated(character));
  let hindrancePointsAvailable = $derived(calculateHindrancePointsAvailable(character));
  let edgeCost = $derived(gameConfig?.hindrance_points_per_edge ?? 2);

  // Track how many points have actually been spent on edges (vs allocated)
  let edgesFromHindrancePoints = $derived(
    character?.edges.filter((e) => e.source === "hindrance_points").length ?? 0
  );
  let pointsSpentOnEdges = $derived(edgesFromHindrancePoints * edgeCost);
  let pointsAvailableForEdges = $derived(hindrancePointsToEdges - pointsSpentOnEdges);

  // Edges are locked if no hindrance points earned OR no points allocated to edges
  let edgesLocked = $derived(hindrancePointsEarned === 0 || hindrancePointsToEdges === 0);

  function canAffordEdge(): boolean {
    return pointsAvailableForEdges >= edgeCost;
  }

  async function addHindrancePointToEdges() {
    saving = true;
    error = null;

    const result = await commands.allocateHindrancePointsToEdges(edgeCost);
    if (result.status === "ok") {
      character = result.data;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function removeHindrancePointFromEdges() {
    saving = true;
    error = null;

    const result = await commands.allocateHindrancePointsToEdges(-edgeCost);
    if (result.status === "ok") {
      character = result.data;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

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

  async function addEdge(edgeId: number, notes: string | null = null) {
    saving = true;
    error = null;

    const result = await commands.addDraftEdge(edgeId, notes);
    if (result.status === "ok") {
      character = result.data;
      // Refresh edges to update availability
      const edgesResult = await commands.getEdges();
      if (edgesResult.status === "ok") {
        edges = edgesResult.data;
      }
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function removeEdge(edgeId: number) {
    saving = true;
    error = null;

    const result = await commands.removeDraftEdge(edgeId);
    if (result.status === "ok") {
      character = result.data;
      // Refresh edges to update availability
      const edgesResult = await commands.getEdges();
      if (edgesResult.status === "ok") {
        edges = edgesResult.data;
      }
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  function handleAddClick(edgeData: EdgeWithAvailability) {
    if (edgeData.edge.can_take_multiple_times) {
      // Open modal to get notes
      notesModal = { edgeId: edgeData.edge.id, edgeName: edgeData.edge.name };
      notesInput = "";
    } else {
      addEdge(edgeData.edge.id);
    }
  }

  function handleNotesSubmit() {
    if (notesModal && notesInput.trim()) {
      addEdge(notesModal.edgeId, notesInput.trim());
      notesModal = null;
      notesInput = "";
    }
  }

  function closeNotesModal() {
    notesModal = null;
    notesInput = "";
  }

  async function handleBack() {
    goto(WIZARD_STEPS.skills);
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
    goto(WIZARD_STEPS.powers);
  }
</script>

<WizardLayout
  characterName={character?.name}
  characterId={character?.id ?? 0}
  currentStep={6}
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
    <!-- Edge List -->
    <div class="lg:col-span-2">
      <div class="mb-4">
        <div class="flex items-start justify-between gap-4 mb-1">
          <h2 class="text-lg font-semibold text-zinc-900 dark:text-zinc-100">
            Select Edges
            <span class="text-sm font-normal text-zinc-400 dark:text-zinc-500"
              >(optional)</span
            >
          </h2>
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
        <p class="text-sm text-zinc-500 dark:text-zinc-400">
          Spend hindrance points to gain edges. Each edge costs {edgeCost} hindrance
          points. Edges with unmet requirements are grayed out.
        </p>
      </div>

      <div class="space-y-4">
        {#each categoryOrder as category}
          {@const categoryEdges = edgesByCategory.get(category) ?? []}
          {@const filteredCategoryEdges = hideLocked
            ? categoryEdges.filter(edgeData => {
                const isSelected = selectedEdgeIds.has(edgeData.edge.id);
                const selectedEdgeValue = character?.edges.find(e => e.edge.id === edgeData.edge.id);
                const isFromAncestry = selectedEdgeValue?.source === "ancestry";
                const isLockedForPurchase = edgesLocked && !isFromAncestry;
                // Show if: available AND not locked, OR already selected
                return (edgeData.is_available && !isLockedForPurchase) || isSelected;
              })
            : categoryEdges}
          {#if filteredCategoryEdges.length > 0}
            {@const isCollapsed = collapsedCategories.has(category)}
            <div
              class="rounded-lg border border-zinc-200 dark:border-zinc-700 overflow-hidden"
            >
              <!-- Category Header -->
              <button
                onclick={() => toggleCategoryCollapse(category)}
                class="w-full px-4 py-3 flex items-center justify-between bg-zinc-100 dark:bg-zinc-800 hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors"
              >
                <div class="flex items-center gap-3">
                  <span
                    class="font-semibold text-zinc-900 dark:text-zinc-100"
                  >
                    {category}
                  </span>
                  <span class="text-xs text-zinc-500 dark:text-zinc-400">
                    ({filteredCategoryEdges.length} edge{filteredCategoryEdges.length !== 1
                      ? "s"
                      : ""})
                  </span>
                </div>
                <svg
                  class="w-4 h-4 text-zinc-400 transition-transform {isCollapsed
                    ? ''
                    : 'rotate-180'}"
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

              <!-- Edges in this category -->
              {#if !isCollapsed}
                <div class="divide-y divide-zinc-100 dark:divide-zinc-800">
                  {#each filteredCategoryEdges as edgeData}
                    {@const isExpanded = expandedEdge === edgeData.edge.id}
                    {@const isSelected = isEdgeSelected(edgeData.edge.id)}
                    {@const selectedEdgeValue = character?.edges.find(e => e.edge.id === edgeData.edge.id)}
                    {@const isFromAncestry = selectedEdgeValue?.source === "ancestry"}
                    {@const isLockedForPurchase = edgesLocked && !isFromAncestry}
                    {@const canAdd =
                      !isLockedForPurchase &&
                      edgeData.is_available &&
                      canAffordEdge() &&
                      (edgeData.edge.can_take_multiple_times || !isSelected)}
                    <div
                      class="{isSelected
                        ? 'bg-blue-50 dark:bg-blue-900/20'
                        : ''} {!edgeData.is_available || isLockedForPurchase
                        ? 'opacity-60'
                        : ''}"
                    >
                      <!-- Edge Header -->
                      <button
                        onclick={() => toggleEdgeExpand(edgeData.edge.id)}
                        class="w-full px-4 py-3 flex items-center justify-between gap-2 text-left hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors"
                      >
                        <div class="flex items-center gap-2 min-w-0">
                          <span
                            class="font-medium text-sm {edgeData.is_available
                              ? 'text-zinc-900 dark:text-zinc-100'
                              : 'text-zinc-500 dark:text-zinc-500'}"
                          >
                            {edgeData.edge.name}
                          </span>
                          {#if edgeData.edge.can_take_multiple_times}
                            <span
                              class="px-1.5 py-0.5 text-xs rounded bg-purple-100 text-purple-700 dark:bg-purple-500/20 dark:text-purple-400"
                            >
                              Multi
                            </span>
                          {/if}
                          {#if isSelected}
                            <span
                              class="px-1.5 py-0.5 text-xs rounded bg-blue-100 text-blue-700 dark:bg-blue-500/20 dark:text-blue-400"
                            >
                              Selected
                            </span>
                          {/if}
                          {#if !edgeData.is_available || isLockedForPurchase}
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

                      <!-- Expanded content -->
                      {#if isExpanded}
                        <div
                          class="px-4 pb-4 border-t border-zinc-100 dark:border-zinc-800"
                        >
                          <!-- Requirements -->
                          {#if edgeData.requirement_statuses.length > 0}
                            <div class="mt-3 mb-3">
                              <span
                                class="text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide"
                                >Requirements:</span
                              >
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
                          <p
                            class="text-sm text-zinc-600 dark:text-zinc-400 mb-4"
                          >
                            {edgeData.edge.description}
                          </p>

                          <!-- Modifiers -->
                          {#if edgeData.edge.modifiers.length > 0}
                            <div class="mb-4">
                              <span
                                class="text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide"
                                >Effects:</span
                              >
                              <ul
                                class="mt-1 text-xs text-zinc-600 dark:text-zinc-400 space-y-0.5"
                              >
                                {#each edgeData.edge.modifiers as mod}
                                  <li>• {mod.description}</li>
                                {/each}
                              </ul>
                            </div>
                          {/if}

                          <!-- Add/Remove button -->
                          <div class="flex items-center justify-between">
                            <span
                              class="text-xs text-zinc-500 dark:text-zinc-500"
                            >
                              {formatSource(edgeData.edge.source)}
                            </span>
                            {#if isSelected && !edgeData.edge.can_take_multiple_times && selectedEdgeValue?.source === "hindrance_points"}
                              <button
                                onclick={() => removeEdge(edgeData.edge.id)}
                                disabled={saving}
                                class="px-3 py-1.5 text-sm font-medium rounded-md bg-red-100 text-red-700 hover:bg-red-200 dark:bg-red-500/20 dark:text-red-400 dark:hover:bg-red-500/30 disabled:opacity-50 transition-colors"
                              >
                                Remove Edge
                              </button>
                            {:else if isSelected && !edgeData.edge.can_take_multiple_times}
                              <span
                                class="text-xs text-zinc-500 dark:text-zinc-400 italic"
                              >
                                Granted by {formatSource(selectedEdgeValue?.source ?? "")}
                              </span>
                            {:else if canAdd}
                              <button
                                onclick={() => handleAddClick(edgeData)}
                                disabled={saving}
                                class="px-3 py-1.5 text-sm font-medium rounded-md bg-blue-600 text-white hover:bg-blue-500 disabled:opacity-50 transition-colors"
                              >
                                Add Edge ({edgeCost} pts)
                              </button>
                            {:else if isLockedForPurchase}
                              <span
                                class="text-xs text-zinc-500 dark:text-zinc-400 italic"
                              >
                                Allocate hindrance points first
                              </span>
                            {:else if !edgeData.is_available}
                              <span
                                class="text-xs text-zinc-500 dark:text-zinc-400 italic"
                              >
                                Requirements not met
                              </span>
                            {:else if !canAffordEdge()}
                              <span
                                class="text-xs text-zinc-500 dark:text-zinc-400 italic"
                              >
                                Not enough allocated points
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
    </div>

    <!-- Sidebar -->
    <div>
      <div class="sticky top-24 space-y-4">
        <!-- Convert Hindrance Points to Edges -->
        <HindrancePointsCard
          {hindrancePointsEarned}
          {hindrancePointsAvailable}
          pointsAllocated={hindrancePointsToEdges}
          targetLabel="edges"
          description="Allocate {edgeCost} hindrance points per edge you want to purchase."
          costPerUnit={edgeCost}
          disabled={saving}
          canDecrease={hindrancePointsToEdges > pointsSpentOnEdges}
          onIncrease={addHindrancePointToEdges}
          onDecrease={removeHindrancePointFromEdges}
        >
          {#snippet extraContent()}
            <div
              class="text-xs text-zinc-500 dark:text-zinc-400 space-y-1 border-t border-zinc-200 dark:border-zinc-700 pt-3 mt-3"
            >
              <div class="flex justify-between">
                <span>Spent on edges:</span>
                <span>{pointsSpentOnEdges} pts</span>
              </div>
              <div class="flex justify-between">
                <span>Available for edges:</span>
                <span class="{pointsAvailableForEdges >= edgeCost ? 'text-green-600 dark:text-green-400' : ''}">{pointsAvailableForEdges} pts</span>
              </div>
              <div class="flex justify-between">
                <span>Cost per edge:</span>
                <span>{edgeCost} pts</span>
              </div>
            </div>
          {/snippet}
        </HindrancePointsCard>

        <!-- Selected Edges -->
        <div
          class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5"
        >
          <h3
            class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 mb-3"
          >
            Selected Edges
          </h3>

          {#if character?.edges && character.edges.length > 0}
            <ul class="space-y-2">
              {#each character.edges as edgeValue}
                <li class="flex items-start justify-between text-sm">
                  <div>
                    <span class="text-zinc-700 dark:text-zinc-300"
                      >{edgeValue.edge.name}</span
                    >
                    {#if edgeValue.notes}
                      <span
                        class="text-xs text-zinc-500 dark:text-zinc-400 block"
                        >({edgeValue.notes})</span
                      >
                    {/if}
                  </div>
                  {#if edgeValue.source === "hindrance_points"}
                    <button
                      onclick={() => removeEdge(edgeValue.edge.id)}
                      disabled={saving}
                      class="text-xs text-red-600 hover:text-red-500 dark:text-red-400 dark:hover:text-red-300 disabled:opacity-50"
                    >
                      Remove
                    </button>
                  {:else}
                    <span class="text-xs text-zinc-400">{formatSource(edgeValue.source)}</span
                    >
                  {/if}
                </li>
              {/each}
            </ul>
          {:else}
            <p class="text-sm text-zinc-500 dark:text-zinc-400 italic">
              No edges selected
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
                class="px-1 py-0.5 text-xs rounded bg-purple-100 text-purple-700 dark:bg-purple-500/20 dark:text-purple-400"
                >Multi</span
              > = Can take multiple times
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</WizardLayout>

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
