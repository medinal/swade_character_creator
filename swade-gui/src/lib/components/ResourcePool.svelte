<script lang="ts">
  interface Props {
    /** Label displayed above the pool */
    label: string;
    /** Current available count */
    available: number;
    /** Maximum count */
    max: number;
    /** Color theme */
    color: "amber" | "purple" | "blue" | "emerald";
    /** Number of columns in the grid */
    columns?: number;
    /** Default value for reset (if different from max) */
    resetValue?: number;
    /** Called when a token is toggled */
    onToggle: (index: number) => void;
    /** Called when reset is clicked */
    onReset?: () => void;
  }

  let {
    label,
    available,
    max,
    color,
    columns = 6,
    resetValue,
    onToggle,
    onReset,
  }: Props = $props();

  const colorClasses: Record<string, { active: string; inactive: string; text: string }> = {
    amber: {
      active: "border-amber-400 dark:border-amber-500 bg-amber-50 dark:bg-amber-500/10 hover:bg-amber-100 dark:hover:bg-amber-500/20",
      inactive: "border-zinc-300 dark:border-zinc-600 bg-zinc-100 dark:bg-zinc-700 hover:border-zinc-400 dark:hover:border-zinc-500",
      text: "text-amber-500",
    },
    purple: {
      active: "border-purple-400 dark:border-purple-500 bg-purple-50 dark:bg-purple-500/10 hover:bg-purple-100 dark:hover:bg-purple-500/20",
      inactive: "border-zinc-300 dark:border-zinc-600 bg-zinc-100 dark:bg-zinc-700 hover:border-zinc-400 dark:hover:border-zinc-500",
      text: "text-purple-500",
    },
    blue: {
      active: "border-blue-400 dark:border-blue-500 bg-blue-50 dark:bg-blue-500/10 hover:bg-blue-100 dark:hover:bg-blue-500/20",
      inactive: "border-zinc-300 dark:border-zinc-600 bg-zinc-100 dark:bg-zinc-700 hover:border-zinc-400 dark:hover:border-zinc-500",
      text: "text-blue-500",
    },
    emerald: {
      active: "border-emerald-400 dark:border-emerald-500 bg-emerald-50 dark:bg-emerald-500/10 hover:bg-emerald-100 dark:hover:bg-emerald-500/20",
      inactive: "border-zinc-300 dark:border-zinc-600 bg-zinc-100 dark:bg-zinc-700 hover:border-zinc-400 dark:hover:border-zinc-500",
      text: "text-emerald-500",
    },
  };

  const resetTextClasses: Record<string, string> = {
    amber: "text-amber-600 hover:text-amber-700 dark:text-amber-400 dark:hover:text-amber-300",
    purple: "text-purple-600 hover:text-purple-700 dark:text-purple-400 dark:hover:text-purple-300",
    blue: "text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300",
    emerald: "text-emerald-600 hover:text-emerald-700 dark:text-emerald-400 dark:hover:text-emerald-300",
  };

  let effectiveResetValue = $derived(resetValue ?? max);
  let showReset = $derived(onReset && available !== effectiveResetValue);

  // Use explicit class names so Tailwind can include them in the build
  const gridColsMap: Record<number, string> = {
    3: "grid-cols-3",
    4: "grid-cols-4",
    5: "grid-cols-5",
    6: "grid-cols-6",
    7: "grid-cols-7",
    8: "grid-cols-8",
  };
  let gridCols = $derived(gridColsMap[columns] ?? "grid-cols-6");
</script>

<div class="grid {gridCols} gap-2 w-fit">
  {#each Array(max) as _, index}
    <button
      type="button"
      onclick={() => onToggle(index)}
      class="w-8 h-8 rounded-full border-2 transition-colors flex items-center justify-center
        {index < available
          ? colorClasses[color].active
          : colorClasses[color].inactive}"
      aria-label="{index < available ? 'Use' : 'Add'} {label.toLowerCase()} {index + 1}"
    >
      {#if index < available}
        <svg class="w-4 h-4 {colorClasses[color].text}" fill="currentColor" viewBox="0 0 20 20">
          <circle cx="10" cy="10" r="6" />
        </svg>
      {/if}
    </button>
  {/each}
</div>
{#if showReset}
  <button
    type="button"
    onclick={onReset}
    class="mt-2 text-xs {resetTextClasses[color]}"
  >
    Reset to {effectiveResetValue}
  </button>
{/if}
