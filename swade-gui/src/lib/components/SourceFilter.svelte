<script lang="ts">
  import { formatSource } from "$lib/utils/formatting";

  interface Props {
    value: string;
    onChange: (value: string) => void;
  }

  let { value, onChange }: Props = $props();

  const sources = [
    { value: "all", label: "All" },
    { value: "core", label: formatSource("core") },
    { value: "fantasy_companion", label: formatSource("fantasy_companion") },
  ];
</script>

<div class="flex gap-1 p-1 bg-zinc-100 dark:bg-zinc-800 rounded-lg">
  {#each sources as source}
    <button
      onclick={() => onChange(source.value)}
      class="px-3 py-1.5 text-sm font-medium rounded-md transition-colors {value === source.value
        ? 'bg-white dark:bg-zinc-700 text-zinc-900 dark:text-zinc-100 shadow-sm'
        : 'text-zinc-600 dark:text-zinc-400 hover:text-zinc-900 dark:hover:text-zinc-200'}"
    >
      {source.label}
    </button>
  {/each}
</div>
