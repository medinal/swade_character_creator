<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** Total hindrance points earned by the character */
    hindrancePointsEarned: number;
    /** Hindrance points available (not yet allocated) */
    hindrancePointsAvailable: number;
    /** Hindrance points currently allocated to this target */
    pointsAllocated: number;
    /** Label for the allocation target (e.g., "attributes", "skills", "edges") */
    targetLabel: string;
    /** Description explaining the conversion rate */
    description: string;
    /** How many hindrance points per unit (2 for attributes/edges, 1 for skills) */
    costPerUnit: number;
    /** Whether controls are disabled (e.g., during save) */
    disabled?: boolean;
    /** Whether the decrease button should be enabled */
    canDecrease: boolean;
    /** Called when user increases allocation */
    onIncrease: () => void;
    /** Called when user decreases allocation */
    onDecrease: () => void;
    /** Optional extra content to display below the controls */
    extraContent?: Snippet;
    /** Optional message to show when no hindrance points are earned */
    emptyMessage?: string;
  }

  let {
    hindrancePointsEarned,
    hindrancePointsAvailable,
    pointsAllocated,
    targetLabel,
    description,
    costPerUnit,
    disabled = false,
    canDecrease,
    onIncrease,
    onDecrease,
    extraContent,
    emptyMessage = "No hindrance points available. Select hindrances to earn points.",
  }: Props = $props();

  let canIncrease = $derived(hindrancePointsAvailable >= costPerUnit);

  function formatCost(points: number): string {
    return `${points} hindrance point${points !== 1 ? "s" : ""}`;
  }
</script>

<div class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5">
  <h3 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 mb-3">
    Convert Hindrance Points
  </h3>

  {#if hindrancePointsEarned === 0}
    <p class="text-sm text-zinc-500 dark:text-zinc-400 italic">
      {emptyMessage}
    </p>
  {:else}
    <p class="text-xs text-zinc-500 dark:text-zinc-400 mb-3">
      {description}
    </p>

    <div class="flex items-center justify-between mb-3">
      <span class="text-sm text-zinc-600 dark:text-zinc-400">Available</span>
      <span class="font-semibold text-amber-600 dark:text-amber-400">
        {hindrancePointsAvailable} pts
      </span>
    </div>

    <div class="flex items-center justify-between">
      <span class="text-sm text-zinc-600 dark:text-zinc-400">Allocated to {targetLabel}</span>
      <div class="flex items-center gap-1">
        <button
          onclick={onDecrease}
          disabled={disabled || !canDecrease}
          class="w-7 h-7 flex items-center justify-center rounded text-zinc-500 hover:bg-zinc-100 dark:hover:bg-zinc-700 disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
          aria-label="Remove hindrance points from {targetLabel}"
        >
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4" />
          </svg>
        </button>
        <span class="w-8 text-center font-semibold text-zinc-900 dark:text-zinc-100">
          {pointsAllocated}
        </span>
        <button
          onclick={onIncrease}
          disabled={disabled || !canIncrease}
          class="w-7 h-7 flex items-center justify-center rounded text-zinc-500 hover:bg-zinc-100 dark:hover:bg-zinc-700 disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
          aria-label="Add hindrance points to {targetLabel}"
        >
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
        </button>
      </div>
    </div>

    {#if pointsAllocated > 0}
      <p class="text-xs text-amber-600 dark:text-amber-400 mt-2">
        Cost: {formatCost(pointsAllocated)}
      </p>
    {/if}

    {#if extraContent}
      {@render extraContent()}
    {/if}
  {/if}
</div>
