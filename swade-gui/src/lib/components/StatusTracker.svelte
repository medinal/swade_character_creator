<script lang="ts">
  interface Props {
    /** Label displayed above the tracker */
    label: string;
    /** Current value (number of filled slots) */
    value: number;
    /** Number of regular slots (not including incapacitated) */
    regularSlots: number;
    /** Color theme for filled slots */
    color: "red" | "amber" | "blue" | "purple";
    /** Whether to show an incapacitated slot */
    showIncapacitated?: boolean;
    /** Label for the incapacitated slot */
    incapacitatedLabel?: string;
    /** Called when a slot is toggled */
    onToggle: (index: number) => void;
  }

  let {
    label,
    value,
    regularSlots,
    color,
    showIncapacitated = false,
    incapacitatedLabel = "INC",
    onToggle,
  }: Props = $props();

  // Color classes for regular slots
  const colorClasses: Record<string, { active: string; inactive: string }> = {
    red: {
      active: "bg-red-500 border-red-500 dark:bg-red-600 dark:border-red-600",
      inactive: "border-zinc-300 dark:border-zinc-600 hover:border-zinc-400 dark:hover:border-zinc-500",
    },
    amber: {
      active: "bg-amber-500 border-amber-500 dark:bg-amber-600 dark:border-amber-600",
      inactive: "border-zinc-300 dark:border-zinc-600 hover:border-zinc-400 dark:hover:border-zinc-500",
    },
    blue: {
      active: "bg-blue-500 border-blue-500 dark:bg-blue-600 dark:border-blue-600",
      inactive: "border-zinc-300 dark:border-zinc-600 hover:border-zinc-400 dark:hover:border-zinc-500",
    },
    purple: {
      active: "bg-purple-500 border-purple-500 dark:bg-purple-600 dark:border-purple-600",
      inactive: "border-zinc-300 dark:border-zinc-600 hover:border-zinc-400 dark:hover:border-zinc-500",
    },
  };

  // Color classes for incapacitated slot
  const incapColorClasses: Record<string, { active: string; inactive: string }> = {
    red: {
      active: "bg-red-700 border-red-700 dark:bg-red-800 dark:border-red-800",
      inactive: "border-red-300 dark:border-red-600 hover:border-red-400 dark:hover:border-red-500",
    },
    amber: {
      active: "bg-amber-700 border-amber-700 dark:bg-amber-800 dark:border-amber-800",
      inactive: "border-amber-300 dark:border-amber-600 hover:border-amber-400 dark:hover:border-amber-500",
    },
    blue: {
      active: "bg-blue-700 border-blue-700 dark:bg-blue-800 dark:border-blue-800",
      inactive: "border-blue-300 dark:border-blue-600 hover:border-blue-400 dark:hover:border-blue-500",
    },
    purple: {
      active: "bg-purple-700 border-purple-700 dark:bg-purple-800 dark:border-purple-800",
      inactive: "border-purple-300 dark:border-purple-600 hover:border-purple-400 dark:hover:border-purple-500",
    },
  };

  // Text color for incapacitated label
  const incapTextColors: Record<string, string> = {
    red: "text-red-500 dark:text-red-500",
    amber: "text-amber-500 dark:text-amber-500",
    blue: "text-blue-500 dark:text-blue-500",
    purple: "text-purple-500 dark:text-purple-500",
  };

  let totalSlots = $derived(regularSlots + (showIncapacitated ? 1 : 0));

  // Use explicit class names so Tailwind can include them in the build
  const gridColsMap: Record<number, string> = {
    1: "grid-cols-1",
    2: "grid-cols-2",
    3: "grid-cols-3",
    4: "grid-cols-4",
  };
  let gridCols = $derived(gridColsMap[Math.min(totalSlots, 4)] ?? "grid-cols-4");

  function isSlotActive(index: number): boolean {
    return value > index;
  }

  function isIncapacitated(): boolean {
    return showIncapacitated && value > regularSlots;
  }
</script>

<div>
  <div class="text-xs text-zinc-500 dark:text-zinc-400 uppercase tracking-wide mb-2">{label}</div>
  <div class="grid {gridCols} gap-2 w-fit">
    {#each Array(regularSlots) as _, index}
      <div class="text-center">
        <button
          type="button"
          onclick={() => onToggle(index)}
          class="w-10 h-10 rounded-lg border-2 transition-colors flex items-center justify-center
            {isSlotActive(index)
              ? colorClasses[color].active
              : colorClasses[color].inactive}"
          aria-label="{isSlotActive(index) ? 'Remove' : 'Add'} {label.toLowerCase()} {index + 1}"
        >
          {#if isSlotActive(index)}
            <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M6 18L18 6M6 6l12 12" />
            </svg>
          {/if}
        </button>
        <div class="text-xs text-zinc-500 dark:text-zinc-500 mt-1">-{index + 1}</div>
      </div>
    {/each}
    {#if showIncapacitated}
      <div class="text-center">
        <button
          type="button"
          onclick={() => onToggle(regularSlots)}
          class="w-10 h-10 rounded-lg border-2 transition-colors flex items-center justify-center
            {isIncapacitated()
              ? incapColorClasses[color].active
              : incapColorClasses[color].inactive}"
          aria-label="{isIncapacitated() ? 'Remove' : 'Mark'} incapacitated"
        >
          {#if isIncapacitated()}
            <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M6 18L18 6M6 6l12 12" />
            </svg>
          {/if}
        </button>
        <div class="text-xs {incapTextColors[color]} mt-1">{incapacitatedLabel}</div>
      </div>
    {/if}
  </div>
</div>
