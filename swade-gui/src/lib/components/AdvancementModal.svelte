<script lang="ts">
  import { onMount } from "svelte";
  import {
    commands,
    type CharacterView,
    type AdvancementOptions,
    type EdgeWithAvailability,
    type EdgeCategory,
    type HindranceAdvanceOption,
  } from "$lib/bindings";

  interface Props {
    character: CharacterView;
    onClose: () => void;
    onAdvanceTaken: (character: CharacterView) => void;
  }

  let { character, onClose, onAdvanceTaken }: Props = $props();

  type Tab = "edge" | "attribute" | "skill" | "hindrance";

  let activeTab = $state<Tab>("edge");
  let loading = $state(true);
  let saving = $state(false);
  let error = $state<string | null>(null);

  // Data from backend
  let options = $state<AdvancementOptions | null>(null);
  let edges = $state<EdgeWithAvailability[]>([]);

  // Selection state
  let selectedEdgeId = $state<number | null>(null);
  let edgeNotes = $state("");
  let selectedAttributeId = $state<number | null>(null);
  let skillMode = $state<"expensive" | "cheap">("expensive");
  let selectedExpensiveSkillId = $state<number | null>(null);
  let selectedCheapSkill1Id = $state<number | null>(null);
  let selectedCheapSkill2Id = $state<number | null>(null);
  let selectedHindrance = $state<HindranceAdvanceOption | null>(null);

  // Edge UI state
  let edgeSearch = $state("");
  let hideLocked = $state(true);
  let expandedEdgeId = $state<number | null>(null);
  let collapsedCategories = $state<Set<EdgeCategory>>(new Set());

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
    await loadData();
  });

  async function loadData() {
    loading = true;
    error = null;

    const [optionsResult, edgesResult] = await Promise.all([
      commands.getAdvancementOptions(character.id),
      commands.getEdges(),
    ]);

    if (optionsResult.status === "ok") {
      options = optionsResult.data;
    } else {
      error = optionsResult.error?.message ?? String(optionsResult.error);
      loading = false;
      return;
    }

    if (edgesResult.status === "ok") {
      edges = edgesResult.data;
    }

    loading = false;
  }

  // Filter and group edges by category
  let edgesByCategory = $derived.by(() => {
    // First filter
    let filtered = edges;
    if (edgeSearch) {
      const search = edgeSearch.toLowerCase();
      filtered = filtered.filter(e =>
        e.edge.name.toLowerCase().includes(search) ||
        e.edge.description.toLowerCase().includes(search)
      );
    }
    if (hideLocked) {
      filtered = filtered.filter(e => e.is_available);
    }

    // Group by category
    const map = new Map<EdgeCategory, EdgeWithAvailability[]>();
    for (const category of categoryOrder) {
      map.set(category, []);
    }
    for (const edgeData of filtered) {
      const category = edgeData.edge.category;
      const existing = map.get(category) ?? [];
      existing.push(edgeData);
      map.set(category, existing);
    }
    // Sort edges alphabetically within each category
    for (const [key, categoryEdges] of map) {
      map.set(key, [...categoryEdges].sort((a, b) => a.edge.name.localeCompare(b.edge.name)));
    }
    return map;
  });

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
    expandedEdgeId = expandedEdgeId === edgeId ? null : edgeId;
  }

  function selectEdge(edgeId: number) {
    selectedEdgeId = edgeId;
    edgeNotes = "";
  }

  // Validation
  let canSubmit = $derived.by(() => {
    if (saving) return false;
    switch (activeTab) {
      case "edge":
        return selectedEdgeId !== null;
      case "attribute":
        return selectedAttributeId !== null && options?.can_increase_attribute;
      case "skill":
        if (skillMode === "expensive") {
          return selectedExpensiveSkillId !== null;
        } else {
          return selectedCheapSkill1Id !== null && selectedCheapSkill2Id !== null &&
                 selectedCheapSkill1Id !== selectedCheapSkill2Id;
        }
      case "hindrance":
        return selectedHindrance !== null;
      default:
        return false;
    }
  });

  async function handleSubmit() {
    if (!canSubmit) return;
    saving = true;
    error = null;

    let result;
    switch (activeTab) {
      case "edge":
        result = await commands.takeEdgeAdvance(
          character.id,
          selectedEdgeId!,
          edgeNotes || null
        );
        break;
      case "attribute":
        result = await commands.takeAttributeAdvance(
          character.id,
          selectedAttributeId!
        );
        break;
      case "skill":
        if (skillMode === "expensive") {
          result = await commands.takeExpensiveSkillAdvance(
            character.id,
            selectedExpensiveSkillId!
          );
        } else {
          result = await commands.takeCheapSkillAdvance(
            character.id,
            selectedCheapSkill1Id!,
            selectedCheapSkill2Id!
          );
        }
        break;
      case "hindrance":
        result = await commands.takeHindranceAdvance(
          character.id,
          selectedHindrance!.id,
          selectedHindrance!.action
        );
        break;
    }

    if (result?.status === "ok") {
      // Reload the character to get updated state
      const charResult = await commands.getCharacter(character.id);
      if (charResult.status === "ok" && charResult.data) {
        onAdvanceTaken(charResult.data);
      }
      onClose();
    } else {
      error = result?.error?.message ?? String(result?.error ?? "Unknown error");
      saving = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onClose();
    }
  }
</script>

<div
  class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
  onclick={onClose}
  onkeydown={handleKeydown}
  role="dialog"
  aria-modal="true"
  aria-labelledby="advancement-modal-title"
  tabindex="-1"
>
  <div
    class="bg-white dark:bg-zinc-800 rounded-lg shadow-xl w-full max-w-3xl max-h-[85vh] mx-4 flex flex-col"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
    role="none"
  >
    <!-- Header -->
    <div class="p-4 border-b border-zinc-200 dark:border-zinc-700">
      <h2 id="advancement-modal-title" class="text-lg font-semibold text-zinc-900 dark:text-zinc-100">
        Take Advancement
      </h2>
      {#if options}
        <p class="text-sm text-zinc-600 dark:text-zinc-400 mt-1">
          Advance #{options.next_advance_number}
          {#if options.current_rank !== options.rank_after_advance}
            <span class="text-green-600 dark:text-green-400">
              — Rank up to {options.rank_after_advance}!
            </span>
          {:else}
            — {options.current_rank}
          {/if}
        </p>
      {/if}
    </div>

    {#if loading}
      <div class="p-8 text-center text-zinc-500">Loading...</div>
    {:else if error}
      <div class="p-4">
        <div class="bg-red-50 dark:bg-red-500/10 border border-red-200 dark:border-red-500/20 text-red-600 dark:text-red-400 px-4 py-3 rounded-md text-sm">
          {error}
        </div>
      </div>
    {:else}
      <!-- Tabs -->
      <div class="flex border-b border-zinc-200 dark:border-zinc-700">
        <button
          class="px-4 py-2 text-sm font-medium transition-colors {activeTab === 'edge' ? 'text-blue-600 border-b-2 border-blue-600' : 'text-zinc-600 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-zinc-100'}"
          onclick={() => activeTab = "edge"}
        >
          Edge
        </button>
        <button
          class="px-4 py-2 text-sm font-medium transition-colors {activeTab === 'attribute' ? 'text-blue-600 border-b-2 border-blue-600' : 'text-zinc-600 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-zinc-100'} {!options?.can_increase_attribute ? 'opacity-50' : ''}"
          onclick={() => activeTab = "attribute"}
          disabled={!options?.can_increase_attribute}
          title={options?.attribute_blocked_reason ?? undefined}
        >
          Attribute
          {#if !options?.can_increase_attribute}
            <span class="text-xs text-zinc-400">(locked)</span>
          {/if}
        </button>
        <button
          class="px-4 py-2 text-sm font-medium transition-colors {activeTab === 'skill' ? 'text-blue-600 border-b-2 border-blue-600' : 'text-zinc-600 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-zinc-100'}"
          onclick={() => activeTab = "skill"}
        >
          Skill
        </button>
        <button
          class="px-4 py-2 text-sm font-medium transition-colors {activeTab === 'hindrance' ? 'text-blue-600 border-b-2 border-blue-600' : 'text-zinc-600 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-zinc-100'} {!options?.can_modify_hindrance ? 'opacity-50' : ''}"
          onclick={() => activeTab = "hindrance"}
          disabled={!options?.can_modify_hindrance}
        >
          Hindrance
          {#if !options?.can_modify_hindrance}
            <span class="text-xs text-zinc-400">(none)</span>
          {/if}
        </button>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-4">
        {#if activeTab === "edge"}
          <!-- Edge Tab -->
          <div class="space-y-3">
            <!-- Search and filter -->
            <div class="flex gap-2">
              <input
                type="text"
                bind:value={edgeSearch}
                placeholder="Search edges..."
                class="flex-1 px-3 py-2 text-sm rounded-md border border-zinc-300 dark:border-zinc-600 bg-white dark:bg-zinc-700 text-zinc-900 dark:text-zinc-100 placeholder-zinc-400"
              />
              <label class="flex items-center gap-2 text-sm text-zinc-600 dark:text-zinc-400 whitespace-nowrap">
                <input type="checkbox" bind:checked={hideLocked} class="rounded" />
                Hide locked
              </label>
            </div>

            <!-- Notes input for multi-take edges -->
            {#if selectedEdgeId}
              {@const selectedEdge = edges.find(e => e.edge.id === selectedEdgeId)}
              {#if selectedEdge?.edge.can_take_multiple_times}
                <div class="bg-blue-50 dark:bg-blue-500/10 border border-blue-200 dark:border-blue-500/20 rounded-md p-3">
                  <label for="edge-notes-input" class="block text-sm font-medium text-blue-700 dark:text-blue-300 mb-1">
                    Notes for {selectedEdge.edge.name}
                  </label>
                  <input
                    id="edge-notes-input"
                    type="text"
                    bind:value={edgeNotes}
                    placeholder="e.g., Athletics, or specific weapon"
                    class="w-full px-3 py-2 text-sm rounded-md border border-blue-300 dark:border-blue-500/30 bg-white dark:bg-zinc-700"
                  />
                </div>
              {/if}
            {/if}

            <!-- Category list -->
            <div class="space-y-2">
              {#each categoryOrder as category}
                {@const categoryEdges = edgesByCategory.get(category) ?? []}
                {#if categoryEdges.length > 0}
                  {@const isCollapsed = collapsedCategories.has(category)}
                  <div class="rounded-lg border border-zinc-200 dark:border-zinc-700 overflow-hidden">
                    <!-- Category Header -->
                    <button
                      onclick={() => toggleCategoryCollapse(category)}
                      class="w-full px-3 py-2 flex items-center justify-between bg-zinc-100 dark:bg-zinc-800 hover:bg-zinc-200 dark:hover:bg-zinc-700 transition-colors"
                    >
                      <div class="flex items-center gap-2">
                        <span class="font-medium text-sm text-zinc-900 dark:text-zinc-100">
                          {category}
                        </span>
                        <span class="text-xs text-zinc-500 dark:text-zinc-400">
                          ({categoryEdges.length})
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
                        {#each categoryEdges as edgeData}
                          {@const isExpanded = expandedEdgeId === edgeData.edge.id}
                          {@const isSelected = selectedEdgeId === edgeData.edge.id}
                          <div class="{isSelected ? 'bg-blue-50 dark:bg-blue-900/20' : ''} {!edgeData.is_available ? 'opacity-60' : ''}">
                            <!-- Edge Header -->
                            <button
                              onclick={() => toggleEdgeExpand(edgeData.edge.id)}
                              class="w-full px-3 py-2 flex items-center justify-between gap-2 text-left hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors"
                            >
                              <div class="flex items-center gap-2 min-w-0">
                                <span class="font-medium text-sm {edgeData.is_available ? 'text-zinc-900 dark:text-zinc-100' : 'text-zinc-500'}">
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
                                {#if !edgeData.is_available}
                                  <span class="px-1.5 py-0.5 text-xs rounded bg-zinc-200 text-zinc-600 dark:bg-zinc-700 dark:text-zinc-400">
                                    Locked
                                  </span>
                                {/if}
                              </div>
                              <svg
                                class="w-4 h-4 text-zinc-400 transition-transform flex-shrink-0 {isExpanded ? 'rotate-180' : ''}"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                              >
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                              </svg>
                            </button>

                            <!-- Expanded content -->
                            {#if isExpanded}
                              <div class="px-3 pb-3 border-t border-zinc-100 dark:border-zinc-800">
                                <!-- Requirements -->
                                {#if edgeData.requirement_statuses.length > 0}
                                  <div class="mt-2 mb-2">
                                    <span class="text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide">Requirements:</span>
                                    <div class="flex flex-wrap gap-1 mt-1">
                                      {#each edgeData.requirement_statuses as req}
                                        <span class="px-2 py-0.5 text-xs rounded {req.is_met ? 'bg-green-100 text-green-700 dark:bg-green-500/20 dark:text-green-400' : 'bg-red-100 text-red-700 dark:bg-red-500/20 dark:text-red-400'}">
                                          {#if req.is_met}
                                            <span class="mr-1">✓</span>
                                          {:else}
                                            <span class="mr-1">✗</span>
                                          {/if}
                                          {req.description}
                                        </span>
                                      {/each}
                                    </div>
                                  </div>
                                {/if}

                                <!-- Description -->
                                <p class="text-sm text-zinc-600 dark:text-zinc-400 mt-2 mb-3">
                                  {edgeData.edge.description}
                                </p>

                                <!-- Modifiers -->
                                {#if edgeData.edge.modifiers.length > 0}
                                  <div class="mb-3">
                                    <span class="text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide">Effects:</span>
                                    <ul class="mt-1 text-xs text-zinc-600 dark:text-zinc-400 space-y-0.5">
                                      {#each edgeData.edge.modifiers as mod}
                                        <li>• {mod.description}</li>
                                      {/each}
                                    </ul>
                                  </div>
                                {/if}

                                <!-- Select button -->
                                <div class="flex items-center justify-between">
                                  <span class="text-xs text-zinc-500">
                                    {edgeData.edge.source}
                                  </span>
                                  {#if edgeData.is_available}
                                    <button
                                      onclick={() => selectEdge(edgeData.edge.id)}
                                      class="px-3 py-1.5 text-sm font-medium rounded-md {isSelected ? 'bg-green-600 text-white' : 'bg-blue-600 text-white hover:bg-blue-500'} transition-colors"
                                    >
                                      {isSelected ? "✓ Selected" : "Select Edge"}
                                    </button>
                                  {:else}
                                    <span class="text-xs text-zinc-500 italic">Requirements not met</span>
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

              {#if [...edgesByCategory.values()].every(arr => arr.length === 0)}
                <p class="text-center text-zinc-500 py-4">No edges match your search</p>
              {/if}
            </div>
          </div>

        {:else if activeTab === "attribute"}
          <!-- Attribute Tab -->
          <div class="space-y-4">
            {#if options?.attribute_blocked_reason}
              <div class="bg-amber-50 dark:bg-amber-500/10 border border-amber-200 dark:border-amber-500/20 text-amber-700 dark:text-amber-400 px-4 py-3 rounded-md text-sm">
                {options.attribute_blocked_reason}
              </div>
            {:else}
              <p class="text-sm text-zinc-600 dark:text-zinc-400">
                Select an attribute to increase by one die type. You can only increase one attribute per rank
                (Legendary: every other advance).
              </p>
              <div class="grid grid-cols-2 gap-3">
                {#each options?.attribute_options ?? [] as attr}
                  {@const isSelected = selectedAttributeId === attr.id}
                  <button
                    class="p-4 rounded-md border text-left transition-colors {isSelected ? 'border-blue-500 bg-blue-50 dark:bg-blue-500/10' : attr.is_maxed ? 'border-zinc-200 dark:border-zinc-700 opacity-50 cursor-not-allowed' : 'border-zinc-200 dark:border-zinc-700 hover:border-zinc-300 dark:hover:border-zinc-600'}"
                    onclick={() => !attr.is_maxed && (selectedAttributeId = attr.id)}
                    disabled={attr.is_maxed}
                  >
                    <div class="flex items-center justify-between">
                      <span class="font-medium text-zinc-900 dark:text-zinc-100">
                        {attr.name}
                      </span>
                      <span class="font-mono font-bold text-lg">
                        d{attr.effective_die}
                        {#if !attr.is_maxed}
                          <span class="text-green-600 dark:text-green-400">→ d{attr.effective_next_die}</span>
                        {:else}
                          <span class="text-zinc-400">(max)</span>
                        {/if}
                      </span>
                    </div>
                  </button>
                {/each}
              </div>
            {/if}
          </div>

        {:else if activeTab === "skill"}
          <!-- Skill Tab -->
          <div class="space-y-4">
            <div class="flex gap-4 border-b border-zinc-200 dark:border-zinc-700 pb-3">
              <label class="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  bind:group={skillMode}
                  value="expensive"
                  class="text-blue-600"
                />
                <span class="text-sm text-zinc-700 dark:text-zinc-300">
                  One skill (at/above attribute)
                </span>
              </label>
              <label class="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  bind:group={skillMode}
                  value="cheap"
                  class="text-blue-600"
                />
                <span class="text-sm text-zinc-700 dark:text-zinc-300">
                  Two skills (below attribute)
                </span>
              </label>
            </div>

            {#if skillMode === "expensive"}
              <p class="text-sm text-zinc-600 dark:text-zinc-400">
                Select a skill that is at or above its linked attribute to increase by one die type.
              </p>
              {#if (options?.expensive_skill_options ?? []).length === 0}
                <p class="text-center text-zinc-500 py-4">
                  No skills qualify. A skill must be at or above its linked attribute.
                </p>
              {:else}
                <div class="space-y-2 max-h-64 overflow-y-auto">
                  {#each options?.expensive_skill_options ?? [] as skill}
                    {@const isSelected = selectedExpensiveSkillId === skill.id}
                    <button
                      class="w-full p-3 rounded-md border text-left transition-colors {isSelected ? 'border-blue-500 bg-blue-50 dark:bg-blue-500/10' : skill.is_maxed ? 'opacity-50 cursor-not-allowed border-zinc-200 dark:border-zinc-700' : 'border-zinc-200 dark:border-zinc-700 hover:border-zinc-300'}"
                      onclick={() => !skill.is_maxed && (selectedExpensiveSkillId = skill.id)}
                      disabled={skill.is_maxed}
                    >
                      <div class="flex items-center justify-between">
                        <span class="font-medium text-zinc-900 dark:text-zinc-100">
                          {skill.name}
                        </span>
                        <span class="font-mono">
                          d{skill.effective_die}
                          {#if !skill.is_maxed}
                            <span class="text-green-600">→ d{skill.effective_next_die}</span>
                          {:else}
                            <span class="text-zinc-400">(max)</span>
                          {/if}
                        </span>
                      </div>
                    </button>
                  {/each}
                </div>
              {/if}
            {:else}
              <p class="text-sm text-zinc-600 dark:text-zinc-400">
                Select two skills below their linked attribute to increase (or start at d4).
              </p>
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <span id="first-skill-label" class="block text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-2">
                    First Skill
                  </span>
                  <div class="space-y-2 max-h-48 overflow-y-auto" role="group" aria-labelledby="first-skill-label">
                    {#each options?.cheap_skill_options ?? [] as skill}
                      {@const isSelected = selectedCheapSkill1Id === skill.id}
                      {@const isOther = selectedCheapSkill2Id === skill.id}
                      <button
                        class="w-full p-2 rounded-md border text-left text-sm transition-colors {isSelected ? 'border-blue-500 bg-blue-50 dark:bg-blue-500/10' : isOther ? 'opacity-50 cursor-not-allowed border-zinc-200 dark:border-zinc-700' : 'border-zinc-200 dark:border-zinc-700 hover:border-zinc-300'}"
                        onclick={() => !isOther && (selectedCheapSkill1Id = skill.id)}
                        disabled={isOther}
                      >
                        <div class="flex items-center justify-between">
                          <span>{skill.name}</span>
                          <span class="font-mono text-xs">
                            {skill.effective_die > 0 ? `d${skill.effective_die}` : "—"}
                            → d{skill.effective_next_die}
                          </span>
                        </div>
                      </button>
                    {/each}
                  </div>
                </div>
                <div>
                  <span id="second-skill-label" class="block text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-2">
                    Second Skill
                  </span>
                  <div class="space-y-2 max-h-48 overflow-y-auto" role="group" aria-labelledby="second-skill-label">
                    {#each options?.cheap_skill_options ?? [] as skill}
                      {@const isSelected = selectedCheapSkill2Id === skill.id}
                      {@const isOther = selectedCheapSkill1Id === skill.id}
                      <button
                        class="w-full p-2 rounded-md border text-left text-sm transition-colors {isSelected ? 'border-blue-500 bg-blue-50 dark:bg-blue-500/10' : isOther ? 'opacity-50 cursor-not-allowed border-zinc-200 dark:border-zinc-700' : 'border-zinc-200 dark:border-zinc-700 hover:border-zinc-300'}"
                        onclick={() => !isOther && (selectedCheapSkill2Id = skill.id)}
                        disabled={isOther}
                      >
                        <div class="flex items-center justify-between">
                          <span>{skill.name}</span>
                          <span class="font-mono text-xs">
                            {skill.effective_die > 0 ? `d${skill.effective_die}` : "—"}
                            → d{skill.effective_next_die}
                          </span>
                        </div>
                      </button>
                    {/each}
                  </div>
                </div>
              </div>
            {/if}
          </div>

        {:else if activeTab === "hindrance"}
          <!-- Hindrance Tab -->
          <div class="space-y-4">
            <p class="text-sm text-zinc-600 dark:text-zinc-400">
              Remove a minor hindrance, reduce a major to minor, or spend 2 advances to remove a major entirely.
            </p>

            {#if (options?.hindrance_options ?? []).length === 0}
              <p class="text-center text-zinc-500 py-4">No hindrances can be modified.</p>
            {:else}
              <div class="space-y-2">
                {#each options?.hindrance_options ?? [] as h}
                  {@const isSelected = selectedHindrance?.id === h.id}
                  <button
                    class="w-full p-3 rounded-md border text-left transition-colors {isSelected ? 'border-blue-500 bg-blue-50 dark:bg-blue-500/10' : 'border-zinc-200 dark:border-zinc-700 hover:border-zinc-300'}"
                    onclick={() => selectedHindrance = h}
                  >
                    <div class="flex items-start justify-between gap-2">
                      <div class="flex items-center gap-2">
                        <span class="flex-shrink-0 px-1.5 py-0.5 text-xs rounded {h.severity.toLowerCase() === 'major' ? 'bg-red-100 text-red-700 dark:bg-red-500/20 dark:text-red-400' : 'bg-amber-100 text-amber-700 dark:bg-amber-500/20 dark:text-amber-400'}">
                          {h.severity.toLowerCase() === "major" ? "M" : "m"}
                        </span>
                        <span class="font-medium text-zinc-900 dark:text-zinc-100">
                          {h.name}
                        </span>
                        {#if h.is_banked}
                          <span class="px-1.5 py-0.5 text-xs rounded bg-green-100 text-green-700 dark:bg-green-500/20 dark:text-green-400">
                            1/2 banked
                          </span>
                        {/if}
                      </div>
                      <span class="text-xs text-zinc-500 dark:text-zinc-400 whitespace-nowrap">
                        {h.action_label}
                      </span>
                    </div>
                    <p class="text-sm text-zinc-600 dark:text-zinc-400 mt-1 line-clamp-2">
                      {h.description}
                    </p>
                  </button>
                {/each}
              </div>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Footer -->
      <div class="p-4 border-t border-zinc-200 dark:border-zinc-700 flex justify-end gap-3">
        <button
          onclick={onClose}
          class="px-4 py-2 text-sm font-medium text-zinc-700 dark:text-zinc-300 hover:bg-zinc-100 dark:hover:bg-zinc-700 rounded-md transition-colors"
        >
          Cancel
        </button>
        <button
          onclick={handleSubmit}
          disabled={!canSubmit}
          class="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-500 disabled:bg-zinc-400 disabled:cursor-not-allowed rounded-md transition-colors"
        >
          {saving ? "Taking Advance..." : "Take Advance"}
        </button>
      </div>
    {/if}
  </div>
</div>
