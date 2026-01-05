<script lang="ts">
  import { onMount } from "svelte";
  import { commands, type GearView, type GearCategoryView, type CharacterView } from "$lib/bindings";
  import SourceFilter from "./SourceFilter.svelte";
  import { formatSource } from "$lib/utils/formatting";

  interface Props {
    character: CharacterView;
    onClose: () => void;
    onGearChanged: (character: CharacterView) => void;
  }

  let { character, onClose, onGearChanged }: Props = $props();

  let categories = $state<GearCategoryView[]>([]);
  let allGear = $state<GearView[]>([]);
  let selectedCategory = $state<number | null>(null);
  let selectedEra = $state<string>("all");
  let selectedSource = $state<string>("all");
  let searchQuery = $state("");
  let loading = $state(true);
  let error = $state<string | null>(null);
  let recentlyAdded = $state<Set<number>>(new Set());

  onMount(async () => {
    // Load categories and all gear
    const [catResult, gearResult] = await Promise.all([
      commands.getGearCategories(),
      commands.getAllGear()
    ]);

    if (catResult.status === "ok") {
      categories = catResult.data;
    }

    if (gearResult.status === "ok") {
      allGear = gearResult.data;
    }

    loading = false;
  });

  // Filtered gear based on category, era, source, and search
  let filteredGear = $derived(
    allGear.filter(g => {
      if (selectedCategory !== null && g.category.id !== selectedCategory) return false;
      if (selectedEra !== "all" && g.era !== selectedEra && g.era !== "any") return false;
      if (selectedSource !== "all" && g.source !== selectedSource) return false;
      if (searchQuery && !g.name.toLowerCase().includes(searchQuery.toLowerCase())) return false;
      return true;
    })
  );

  // Map of gear_id -> quantity owned
  let ownedGear = $derived(
    new Map(character.gear.map(g => [g.gear.id, g.quantity]))
  );

  function flashAdded(gearId: number) {
    recentlyAdded = new Set([...recentlyAdded, gearId]);
    setTimeout(() => {
      recentlyAdded = new Set([...recentlyAdded].filter(id => id !== gearId));
    }, 1500);
  }

  async function handlePurchase(gearId: number) {
    const result = await commands.purchaseGear(character.id, gearId, 1);
    if (result.status === "ok") {
      onGearChanged(result.data);
      flashAdded(gearId);
      error = null;
    } else {
      error = typeof result.error === "string" ? result.error : result.error.message;
    }
  }

  async function handleAddFree(gearId: number) {
    const result = await commands.addGear(character.id, gearId, 1);
    if (result.status === "ok") {
      onGearChanged(result.data);
      flashAdded(gearId);
      error = null;
    } else {
      error = typeof result.error === "string" ? result.error : result.error.message;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onClose();
    }
  }

  function formatGearStats(gear: GearView): string {
    const parts: string[] = [];

    if (gear.weapon_stats) {
      parts.push(`Dmg: ${gear.weapon_stats.damage}`);
      if (gear.weapon_stats.range_short) {
        parts.push(`Range: ${gear.weapon_stats.range_short}/${gear.weapon_stats.range_medium}/${gear.weapon_stats.range_long}`);
      }
    }

    if (gear.armor_stats) {
      parts.push(`Coverage: ${gear.armor_stats.coverage}`);
    }

    if (gear.shield_stats) {
      parts.push(`Cover: -${gear.shield_stats.cover_penalty}`);
    }

    return parts.join(" | ");
  }
</script>

<div
  class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
  onclick={onClose}
  onkeydown={handleKeydown}
  role="dialog"
  aria-modal="true"
  aria-labelledby="gear-browser-title"
  tabindex="-1"
>
  <div
    class="bg-white dark:bg-zinc-800 rounded-lg shadow-xl w-full max-w-4xl max-h-[80vh] flex flex-col mx-4"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
    role="none"
  >
    <!-- Header -->
    <div class="p-4 border-b border-zinc-200 dark:border-zinc-700">
      <div class="flex items-center justify-between mb-4">
        <h2 id="gear-browser-title" class="text-lg font-semibold text-zinc-900 dark:text-zinc-100">
          Gear Browser
        </h2>
        <div class="text-sm text-zinc-600 dark:text-zinc-400">
          Current wealth: <span class="font-bold text-green-600 dark:text-green-400">${character.wealth}</span>
        </div>
      </div>

      <!-- Filters -->
      <div class="flex flex-wrap gap-3">
        <!-- Category tabs -->
        <div class="flex flex-wrap gap-1">
          <button
            onclick={() => selectedCategory = null}
            class="px-3 py-1.5 text-sm rounded-md transition-colors {selectedCategory === null
              ? 'bg-blue-600 text-white'
              : 'bg-zinc-100 dark:bg-zinc-700 text-zinc-700 dark:text-zinc-300 hover:bg-zinc-200 dark:hover:bg-zinc-600'}"
          >
            All
          </button>
          {#each categories as cat}
            <button
              onclick={() => selectedCategory = cat.id}
              class="px-3 py-1.5 text-sm rounded-md transition-colors {selectedCategory === cat.id
                ? 'bg-blue-600 text-white'
                : 'bg-zinc-100 dark:bg-zinc-700 text-zinc-700 dark:text-zinc-300 hover:bg-zinc-200 dark:hover:bg-zinc-600'}"
            >
              {cat.name}
            </button>
          {/each}
        </div>

        <!-- Era filter -->
        <select
          bind:value={selectedEra}
          class="px-3 py-1.5 text-sm rounded-md border border-zinc-300 dark:border-zinc-600 bg-white dark:bg-zinc-700 text-zinc-900 dark:text-zinc-100"
        >
          <option value="all">All Eras</option>
          <option value="medieval">Medieval</option>
          <option value="modern">Modern</option>
          <option value="futuristic">Futuristic</option>
        </select>

        <!-- Source filter -->
        <SourceFilter value={selectedSource} onChange={(v) => selectedSource = v} />

        <!-- Search -->
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Search gear..."
          class="flex-1 min-w-[200px] px-3 py-1.5 text-sm rounded-md border border-zinc-300 dark:border-zinc-600 bg-white dark:bg-zinc-700 text-zinc-900 dark:text-zinc-100 placeholder-zinc-400"
        />
      </div>

      <!-- Error -->
      {#if error}
        <div class="mt-3 p-2 bg-red-50 dark:bg-red-500/10 border border-red-200 dark:border-red-500/20 rounded text-sm text-red-600 dark:text-red-400">
          {error}
        </div>
      {/if}
    </div>

    <!-- Gear list -->
    <div class="flex-1 overflow-y-auto p-4">
      {#if loading}
        <div class="text-center text-zinc-500 py-8">Loading gear...</div>
      {:else if filteredGear.length === 0}
        <div class="text-center text-zinc-500 py-8">No gear found</div>
      {:else}
        <div class="space-y-2">
          {#each filteredGear as gear}
            {@const stats = formatGearStats(gear)}
            {@const canAfford = character.wealth >= gear.cost}
            {@const quantityOwned = ownedGear.get(gear.id) ?? 0}
            {@const justAdded = recentlyAdded.has(gear.id)}
            <div class="flex items-center gap-3 p-3 rounded-lg transition-all duration-300 {justAdded
              ? 'bg-green-100 dark:bg-green-900/30 ring-2 ring-green-500'
              : quantityOwned > 0
                ? 'bg-blue-50 dark:bg-blue-900/20'
                : 'bg-zinc-50 dark:bg-zinc-700/50 hover:bg-zinc-100 dark:hover:bg-zinc-700'}">
              <!-- Item info -->
              <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2">
                  <span class="font-medium text-zinc-900 dark:text-zinc-100">{gear.name}</span>
                  {#if quantityOwned > 0}
                    <span class="text-xs px-1.5 py-0.5 rounded bg-blue-500 text-white font-medium">
                      Owned{quantityOwned > 1 ? ` x${quantityOwned}` : ''}
                    </span>
                  {/if}
                  <span class="text-xs px-1.5 py-0.5 rounded bg-zinc-200 dark:bg-zinc-600 text-zinc-600 dark:text-zinc-400">
                    {gear.category.name}
                  </span>
                  <span class="text-xs px-1.5 py-0.5 rounded {gear.source === 'fantasy_companion'
                    ? 'bg-purple-100 dark:bg-purple-900/30 text-purple-700 dark:text-purple-300'
                    : 'bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-300'}">
                    {formatSource(gear.source)}
                  </span>
                  {#if gear.era !== "any"}
                    <span class="text-xs text-zinc-400">{gear.era}</span>
                  {/if}
                </div>
                {#if stats}
                  <div class="text-xs text-zinc-500 dark:text-zinc-400 mt-0.5">{stats}</div>
                {/if}
                {#if gear.modifiers.length > 0}
                  <div class="text-xs text-green-600 dark:text-green-400 mt-0.5">
                    {gear.modifiers.map(m => m.description).join(", ")}
                  </div>
                {/if}
                {#if gear.pack_contents && gear.pack_contents.length > 0}
                  <div class="text-xs text-blue-600 dark:text-blue-400 mt-0.5">
                    <span class="font-medium">Contains:</span>
                    {gear.pack_contents.map(pc =>
                      `${pc.item.name}${pc.quantity > 1 ? ` x${pc.quantity}` : ''}`
                    ).join(', ')}
                  </div>
                {/if}
                <div class="text-xs text-zinc-400 mt-0.5">
                  ${gear.cost} | {gear.weight} lbs
                </div>
              </div>

              <!-- Actions -->
              <div class="flex items-center gap-2">
                <button
                  onclick={() => handlePurchase(gear.id)}
                  disabled={!canAfford}
                  class="px-3 py-1.5 text-xs font-medium rounded transition-colors {canAfford
                    ? 'bg-green-600 hover:bg-green-500 text-white'
                    : 'bg-zinc-200 dark:bg-zinc-600 text-zinc-400 cursor-not-allowed'}"
                >
                  {quantityOwned > 0 ? '+1' : 'Buy'} ${gear.cost}
                </button>
                <button
                  onclick={() => handleAddFree(gear.id)}
                  class="px-3 py-1.5 text-xs font-medium rounded bg-zinc-200 dark:bg-zinc-600 text-zinc-700 dark:text-zinc-300 hover:bg-zinc-300 dark:hover:bg-zinc-500 transition-colors"
                >
                  {quantityOwned > 0 ? '+1 Free' : 'Add Free'}
                </button>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Footer -->
    <div class="p-4 border-t border-zinc-200 dark:border-zinc-700 flex justify-end">
      <button
        onclick={onClose}
        class="px-4 py-2 text-sm font-medium text-zinc-700 dark:text-zinc-300 hover:bg-zinc-100 dark:hover:bg-zinc-700 rounded-md transition-colors"
      >
        Close
      </button>
    </div>
  </div>
</div>
