<script lang="ts">
  import type { CharacterGearValue } from "$lib/bindings";

  interface Props {
    item: CharacterGearValue;
    onToggleEquip: (id: number) => void;
    onSell: (id: number, quantity: number) => void;
    onRemove: (id: number, quantity: number) => void;
  }

  let { item, onToggleEquip, onSell, onRemove }: Props = $props();

  function getCategoryIcon(categoryName: string): string {
    const icons: Record<string, string> = {
      "Melee Weapons": "M7 21h10a2 2 0 002-2V9a2 2 0 00-2-2h-2l-2-2H9L7 7H5a2 2 0 00-2 2v10a2 2 0 002 2z",
      "Ranged Weapons": "M12 19l9 2-9-18-9 18 9-2zm0 0v-8",
      "Armor": "M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z",
      "Shields": "M20.618 5.984A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z",
      "Common Gear": "M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4",
      "Ammunition": "M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"
    };
    return icons[categoryName] ?? icons["Common Gear"];
  }

  function formatStats(): string[] {
    const stats: string[] = [];

    if (item.gear.weapon_stats) {
      const w = item.gear.weapon_stats;
      stats.push(`Dmg: ${w.damage}`);
      if (w.ap > 0) stats.push(`AP ${w.ap}`);
      if (w.range_short) stats.push(`Range: ${w.range_short}/${w.range_medium}/${w.range_long}`);
      if (w.rof) stats.push(`RoF ${w.rof}`);
      if (w.reach) stats.push(`Reach ${w.reach}`);
    }

    if (item.gear.armor_stats) {
      const a = item.gear.armor_stats;
      stats.push(`Coverage: ${a.coverage}`);
    }

    if (item.gear.shield_stats) {
      const s = item.gear.shield_stats;
      stats.push(`Cover: -${s.cover_penalty}`);
    }

    return stats;
  }

  function formatModifiers(): string[] {
    return item.gear.modifiers
      .filter(m => m.value !== null)
      .map(m => m.description);
  }
</script>

<div class="flex items-start gap-3 p-3 rounded-lg bg-zinc-50 dark:bg-zinc-700/50 {item.is_equipped ? 'ring-1 ring-blue-500' : ''}">
  <!-- Equip checkbox -->
  <button
    onclick={() => onToggleEquip(item.id)}
    class="flex-shrink-0 mt-0.5"
    aria-label={item.is_equipped ? "Unequip" : "Equip"}
  >
    <div class="w-5 h-5 rounded border-2 flex items-center justify-center transition-colors {item.is_equipped
      ? 'bg-blue-600 border-blue-600 text-white'
      : 'border-zinc-300 dark:border-zinc-500 hover:border-blue-500'}">
      {#if item.is_equipped}
        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7" />
        </svg>
      {/if}
    </div>
  </button>

  <!-- Item info -->
  <div class="flex-1 min-w-0">
    <div class="flex items-center gap-2">
      <svg class="w-4 h-4 text-zinc-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={getCategoryIcon(item.gear.category.name)} />
      </svg>
      <span class="font-medium text-zinc-900 dark:text-zinc-100">{item.gear.name}</span>
      {#if item.quantity > 1}
        <span class="text-xs text-zinc-500">x{item.quantity}</span>
      {/if}
    </div>

    <!-- Stats -->
    {#if true}
      {@const stats = formatStats()}
      {@const mods = formatModifiers()}
      {#if stats.length > 0 || mods.length > 0}
      <div class="text-xs text-zinc-500 dark:text-zinc-400 mt-1">
        {#if stats.length > 0}
          <span>{stats.join(" | ")}</span>
        {/if}
        {#if mods.length > 0}
          {#each mods as mod}
            <span class="text-green-600 dark:text-green-400 ml-2">{mod}</span>
          {/each}
        {/if}
      </div>
      {/if}
    {/if}

    <!-- Weight -->
    <div class="text-xs text-zinc-400 dark:text-zinc-500 mt-1">
      {item.total_weight.toFixed(1)} lbs | ${item.gear.cost}
    </div>
  </div>

  <!-- Actions -->
  <div class="flex-shrink-0 flex items-center gap-1">
    <button
      onclick={() => onSell(item.id, 1)}
      class="p-1 text-zinc-400 hover:text-green-600 dark:hover:text-green-400 transition-colors"
      title="Sell (50% value)"
    >
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
    </button>
    <button
      onclick={() => onRemove(item.id, 1)}
      class="p-1 text-zinc-400 hover:text-red-600 dark:hover:text-red-400 transition-colors"
      title="Remove"
    >
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
      </svg>
    </button>
  </div>
</div>
