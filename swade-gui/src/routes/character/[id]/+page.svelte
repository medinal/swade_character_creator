<script lang="ts">
  import { commands, type CharacterView, type CharacterNoteValue } from "$lib/bindings";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { formatDie, diceEqual } from "$lib/character-utils";
  import { getModifiersFor, type ModifierWithSource } from "$lib/utils/modifiers";
  import { formatSource } from "$lib/utils/formatting";
  import StatusTracker from "$lib/components/StatusTracker.svelte";
  import ResourcePool from "$lib/components/ResourcePool.svelte";
  import NotesPanel from "$lib/components/NotesPanel.svelte";
  import AdvancementModal from "$lib/components/AdvancementModal.svelte";
  import AdvancementHistory from "$lib/components/AdvancementHistory.svelte";
  import GearItem from "$lib/components/GearItem.svelte";
  import GearBrowserModal from "$lib/components/GearBrowserModal.svelte";
  import PortraitUpload from "$lib/components/PortraitUpload.svelte";
  import ConfirmDeleteModal from "$lib/components/ConfirmDeleteModal.svelte";

  let character = $state<CharacterView | null>(null);
  let showDeleteConfirm = $state(false);
  let loading = $state(true);
  let error = $state<string | null>(null);

  onMount(async () => {
    const id = parseInt($page.params.id ?? "");
    if (isNaN(id)) {
      error = "Invalid character ID";
      loading = false;
      return;
    }

    const result = await commands.getCharacter(id);
    if (result.status === "ok") {
      if (result.data) {
        character = result.data;
      } else {
        error = "Character not found";
      }
    } else {
      error = result.error.message;
    }
    loading = false;
  });

  function getRankAbbrev(rankName: string): string {
    const abbrevs: Record<string, string> = {
      "Novice": "N",
      "Seasoned": "S",
      "Veteran": "V",
      "Heroic": "H",
      "Legendary": "L"
    };
    return abbrevs[rankName] ?? rankName.charAt(0);
  }

  function formatChoiceType(choiceType: string): string {
    const labels: Record<string, string> = {
      "free_edge": "Free Edge",
      "mandatory_hindrance": "Mandatory Hindrance",
      "ancestral_enemy": "Ancestral Enemy"
    };
    return labels[choiceType] ?? choiceType;
  }

  function formatModifierDescription(mod: { target_type: string | null; target_identifier: string | null; value_type: string; value: number | null; description: string }): string {
    // For descriptive modifiers (no target), just use description
    if (!mod.target_type || mod.value_type === "description") {
      return mod.description;
    }
    // For modifiers with values, show a formatted string
    const sign = mod.value && mod.value > 0 ? "+" : "";
    const valueStr = mod.value !== null ? `${sign}${mod.value}` : "";
    if (mod.value_type === "die_increment") {
      return `${valueStr} die step to ${mod.target_identifier}`;
    } else if (mod.value_type === "flat_bonus") {
      return `${valueStr} ${mod.target_identifier}`;
    }
    return mod.description;
  }

  // Get all skills sorted alphabetically with attribute lookup
  let sortedSkills = $derived(
    [...(character?.skills ?? [])]
      .filter(s => !hideUntrainedSkills || s.die !== null)
      .sort((a, b) => a.skill.name.localeCompare(b.skill.name))
  );

  // Get attribute name by ID
  function getAttributeName(attrId: number): string {
    const attr = character?.attributes.find(a => a.attribute.id === attrId);
    return attr?.attribute.name ?? "";
  }

  // Helper to get modifiers for display - wraps the utility function
  function getModifiers(targetType: string, targetIdentifier: string): ModifierWithSource[] {
    return getModifiersFor(character, targetType, targetIdentifier);
  }

  function handleBack() {
    goto("/");
  }

  async function handleDelete() {
    if (!character) return;
    const result = await commands.deleteCharacter(character.id);
    if (result.status === "ok") {
      goto("/");
    } else {
      error = typeof result.error === "string" ? result.error : result.error.message;
      showDeleteConfirm = false;
    }
  }

  let editing = $state(false);
  let hideUntrainedSkills = $state(false);
  let ancestryExpanded = $state(false);
  let powerPointsUsed = $state(0);
  let wounds = $state(0); // 0-4 (4 = incapacitated)
  let fatigue = $state(0); // 0-3 (3 = incapacitated)
  let benniesAvailable = $state(3); // Start with 3, can earn up to 6
  let statusInitialized = $state(false);

  // Notes panel state
  let notesOpen = $state(false);
  let notes = $state<CharacterNoteValue[]>([]);

  // Advancement modal state
  let advancementModalOpen = $state(false);
  let advancementHistoryKey = $state(0); // Increment to refresh history

  // Gear modal state
  let gearBrowserOpen = $state(false);

  // Calculate total power points (base from character + bonus from edges like "Power Points")
  let basePowerPoints = $derived(character?.power_points ?? 0);
  let bonusPowerPoints = $derived(
    character?.edges.reduce((sum, edgeVal) => {
      return sum + edgeVal.edge.modifiers
        .filter(m => m.target_type === 'power_points' && m.value !== null)
        .reduce((s, m) => s + (m.value ?? 0), 0);
    }, 0) ?? 0
  );
  let totalPowerPoints = $derived(basePowerPoints + bonusPowerPoints);

  // Initialize status values from character when loaded
  $effect(() => {
    if (character && !statusInitialized) {
      wounds = character.wounds;
      fatigue = character.fatigue;
      powerPointsUsed = character.power_points_used;
      notes = character.notes;
      statusInitialized = true;
    }
  });

  // Save status changes to backend
  async function saveStatus() {
    if (!character || !statusInitialized) return;
    await commands.updateCharacterStatus(character.id, wounds, fatigue, powerPointsUsed);
  }

  function togglePowerPoint(index: number) {
    // Click to add/remove used power points
    const targetLevel = index + 1;
    if (powerPointsUsed >= targetLevel) {
      powerPointsUsed = index; // Remove this used point and all after (restore)
    } else {
      powerPointsUsed = Math.min(targetLevel, totalPowerPoints); // Mark as used up to this one
    }
    saveStatus();
  }

  function toggleWound(index: number) {
    // index 0-3 for wounds 1-3 and INC (4)
    const woundLevel = index + 1;
    if (wounds >= woundLevel) {
      wounds = index; // Remove this wound and all after
    } else {
      wounds = woundLevel; // Add wounds up to this level
    }
    saveStatus();
  }

  function toggleFatigue(index: number) {
    // index 0-2 for fatigue 1-2 and INC (3)
    const fatigueLevel = index + 1;
    if (fatigue >= fatigueLevel) {
      fatigue = index; // Remove this fatigue and all after
    } else {
      fatigue = fatigueLevel; // Add fatigue up to this level
    }
    saveStatus();
  }

  function toggleBenny(index: number) {
    // Click to add/remove bennies
    const targetLevel = index + 1;
    if (benniesAvailable >= targetLevel) {
      benniesAvailable = index; // Remove this benny and all after
    } else {
      benniesAvailable = targetLevel; // Add bennies up to this one
    }
  }

  async function handleEdit() {
    if (!character) return;
    editing = true;

    const result = await commands.loadCharacterIntoDraft(character.id);
    if (result.status === "ok") {
      goto("/create");
    } else {
      error = result.error.message;
      editing = false;
    }
  }

  // Notes handlers
  async function handleCreateNote(title: string, body: string) {
    if (!character) return;
    const result = await commands.createCharacterNote(character.id, title, body);
    if (result.status === "ok") {
      // Add to beginning of list (newest first)
      notes = [result.data, ...notes];
    }
  }

  async function handleUpdateNote(noteId: number, title: string, body: string) {
    const result = await commands.updateCharacterNote(noteId, title, body);
    if (result.status === "ok") {
      notes = notes.map(n => n.id === noteId ? result.data : n);
    }
  }

  async function handleDeleteNote(noteId: number) {
    const result = await commands.deleteCharacterNote(noteId);
    if (result.status === "ok") {
      notes = notes.filter(n => n.id !== noteId);
    }
  }

  function handleAdvanceTaken(updatedCharacter: CharacterView) {
    character = updatedCharacter;
    notes = updatedCharacter.notes;
    advancementHistoryKey++; // Trigger history refresh
  }

  // Gear handlers
  function handleGearChanged(updatedCharacter: CharacterView) {
    character = updatedCharacter;
  }

  // Portrait handler
  function handlePortraitChanged(updatedCharacter: CharacterView) {
    character = updatedCharacter;
  }

  async function handleToggleEquip(gearId: number) {
    if (!character) return;
    const result = await commands.toggleGearEquipped(gearId);
    if (result.status === "ok") {
      character = result.data;
    }
  }

  async function handleSellGear(gearId: number, quantity: number) {
    if (!character) return;
    const result = await commands.sellGear(gearId, quantity);
    if (result.status === "ok") {
      character = result.data;
    }
  }

  async function handleRemoveGear(gearId: number, quantity: number) {
    if (!character) return;
    const result = await commands.removeGear(gearId, quantity);
    if (result.status === "ok") {
      character = result.data;
    }
  }

  // Wealth editing
  let editingWealth = $state(false);
  let wealthInput = $state("");

  function startEditingWealth() {
    if (!character) return;
    wealthInput = character.wealth.toString();
    editingWealth = true;
  }

  async function saveWealth() {
    if (!character) return;
    const newWealth = parseInt(wealthInput);
    if (isNaN(newWealth) || newWealth < 0) {
      editingWealth = false;
      return;
    }
    const result = await commands.updateCharacterWealth(character.id, newWealth);
    if (result.status === "ok") {
      character = result.data;
    }
    editingWealth = false;
  }

  function handleWealthKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      saveWealth();
    } else if (e.key === "Escape") {
      editingWealth = false;
    }
  }
</script>

<div class="min-h-screen bg-zinc-100 text-zinc-900 dark:bg-zinc-900 dark:text-zinc-100">
  <!-- Header -->
  <header class="bg-white border-b border-zinc-200 dark:bg-zinc-900 dark:border-zinc-800">
    <div class="max-w-6xl mx-auto px-6 py-3 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <button
          onclick={handleBack}
          class="text-zinc-500 hover:text-zinc-700 dark:hover:text-zinc-300 transition-colors"
          aria-label="Back to characters"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        <h1 class="text-sm font-medium text-zinc-500 dark:text-zinc-400">
          Character Sheet
        </h1>
      </div>
      <div class="flex items-center gap-3">
        <button
          onclick={() => advancementModalOpen = true}
          disabled={!character}
          class="bg-green-600 hover:bg-green-500 disabled:bg-green-400 text-white text-sm font-medium px-3 py-1.5 rounded-md transition-colors flex items-center gap-1.5"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
          </svg>
          Advance
        </button>
        <button
          onclick={handleEdit}
          disabled={editing || !character}
          class="bg-blue-600 hover:bg-blue-500 disabled:bg-blue-400 text-white text-sm font-medium px-3 py-1.5 rounded-md transition-colors flex items-center gap-1.5"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
          </svg>
          {editing ? "Loading..." : "Edit"}
        </button>
        <button
          onclick={() => showDeleteConfirm = true}
          class="bg-red-600 hover:bg-red-500 text-white text-sm font-medium px-3 py-1.5 rounded-md transition-colors flex items-center gap-1.5"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
          Delete
        </button>
      </div>
    </div>
  </header>

  <main class="max-w-6xl mx-auto px-6 py-6">
    {#if loading}
      <div class="text-zinc-500 text-sm">Loading character...</div>
    {:else if error}
      <div class="bg-red-50 border border-red-200 text-red-600 dark:bg-red-500/10 dark:border-red-500/20 dark:text-red-400 px-4 py-3 rounded-md text-sm">
        Error: {error}
      </div>
    {:else if character}
      <!-- Character Header -->
      <div class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5 mb-6">
        <div class="flex flex-wrap items-start justify-between gap-4">
          <div class="flex items-start gap-4">
            <!-- Portrait -->
            <PortraitUpload {character} onPortraitChanged={handlePortraitChanged} />

            <div>
              <h2 class="text-2xl font-bold text-zinc-900 dark:text-zinc-100 mb-1">
                {character.name}
              </h2>
              <!-- Rank & Advances -->
              <div class="flex flex-wrap items-center gap-3 text-sm text-zinc-600 dark:text-zinc-400">
                <span>{character.rank.name}</span>
                <span class="text-zinc-300 dark:text-zinc-700">•</span>
                <span>{character.current_advances} advances</span>
              </div>
            </div>
          </div>

          <!-- Derived Stats -->
          <div class="flex gap-4">
            <div class="text-center group relative cursor-help">
              <div class="text-xs text-zinc-500 dark:text-zinc-400 uppercase tracking-wide mb-1">Pace</div>
              <div class="text-2xl font-bold text-zinc-900 dark:text-zinc-100">{character.derived_stats.pace}</div>
              <span class="tooltip">
                How many inches (tabletop) the character moves per round. Running adds a d6.
              </span>
            </div>
            <div class="text-center group relative cursor-help">
              <div class="text-xs text-zinc-500 dark:text-zinc-400 uppercase tracking-wide mb-1">Parry</div>
              <div class="text-2xl font-bold text-zinc-900 dark:text-zinc-100">{character.derived_stats.parry}</div>
              <span class="tooltip">
                Target number to hit in melee combat. Equal to 2 + half Fighting die.
              </span>
            </div>
            <div class="text-center group relative cursor-help">
              <div class="text-xs text-zinc-500 dark:text-zinc-400 uppercase tracking-wide mb-1">Toughness</div>
              <div class="text-2xl font-bold text-zinc-900 dark:text-zinc-100">{character.derived_stats.toughness}</div>
              <span class="tooltip">
                How much damage the character can take. Equal to 2 + half Vigor die (+ armor).
              </span>
            </div>
            <div class="text-center group relative">
              <div class="text-xs text-zinc-500 dark:text-zinc-400 uppercase tracking-wide mb-1">Wealth</div>
              {#if editingWealth}
                <div class="flex items-center justify-center gap-1">
                  <span class="text-2xl font-bold text-zinc-900 dark:text-zinc-100">$</span>
                  <input
                    type="number"
                    bind:value={wealthInput}
                    onkeydown={handleWealthKeydown}
                    onblur={saveWealth}
                    class="w-28 text-2xl font-bold text-center bg-white dark:bg-zinc-800 border border-blue-500 rounded px-1 focus:outline-none focus:ring-2 focus:ring-blue-500"
                    min="0"
                  />
                </div>
              {:else}
                <button
                  onclick={startEditingWealth}
                  class="text-2xl font-bold text-zinc-900 dark:text-zinc-100 hover:text-blue-600 dark:hover:text-blue-400 cursor-pointer transition-colors"
                  title="Click to edit wealth"
                >
                  ${character.wealth}
                </button>
              {/if}
            </div>
          </div>
        </div>

        <!-- Ancestry -->
        {#if character.ancestry}
          <div class="mt-4 pt-4 border-t border-zinc-200 dark:border-zinc-700">
            <button
              class="w-full flex items-center justify-between text-left"
              onclick={() => ancestryExpanded = !ancestryExpanded}
            >
              <div>
                <span class="text-sm font-medium text-zinc-700 dark:text-zinc-300">Ancestry:</span>
                <span class="text-sm text-zinc-600 dark:text-zinc-400 ml-1">{character.ancestry.name}</span>
              </div>
              <svg
                class="w-4 h-4 text-zinc-500 transition-transform {ancestryExpanded ? 'rotate-180' : ''}"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                aria-hidden="true"
              >
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
              </svg>
            </button>

            {#if ancestryExpanded}
              <div class="mt-3 space-y-3">
                <!-- Description -->
                <p class="text-sm text-zinc-600 dark:text-zinc-400">
                  {character.ancestry.description}
                </p>

                <!-- Racial Abilities (Modifiers) -->
                {#if character.ancestry.modifiers.length > 0}
                  <div>
                    <h4 class="text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide mb-2">
                      Racial Abilities
                    </h4>
                    <ul class="space-y-1">
                      {#each character.ancestry.modifiers as mod}
                        <li class="text-sm text-zinc-600 dark:text-zinc-400 flex items-start gap-2">
                          <span class="text-zinc-400 dark:text-zinc-500">•</span>
                          <span>{formatModifierDescription(mod)}</span>
                        </li>
                      {/each}
                    </ul>
                  </div>
                {/if}

                <!-- Ancestry Choices Made -->
                {#if character.ancestry_choices.length > 0}
                  <div>
                    <h4 class="text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide mb-2">
                      Ancestry Choices
                    </h4>
                    <ul class="space-y-1">
                      {#each character.ancestry_choices as choiceValue}
                        <li class="text-sm text-zinc-600 dark:text-zinc-400 flex items-start gap-2">
                          <span class="text-zinc-400 dark:text-zinc-500">•</span>
                          <span>
                            <span class="font-medium">{formatChoiceType(choiceValue.choice.choice_type)}:</span>
                            {#if choiceValue.selected_option}
                              {choiceValue.selected_option.description ?? "Selected"}
                            {:else}
                              <span class="italic text-zinc-500">None selected</span>
                            {/if}
                          </span>
                        </li>
                      {/each}
                    </ul>
                  </div>
                {/if}
              </div>
            {/if}
          </div>
        {/if}

        <!-- Background & Description -->
        {#if character.background || character.description}
          <div class="mt-4 pt-4 border-t border-zinc-200 dark:border-zinc-700">
            {#if character.background}
              <p class="text-sm text-zinc-600 dark:text-zinc-400 mb-2">
                <span class="font-medium text-zinc-700 dark:text-zinc-300">Background:</span> {character.background}
              </p>
            {/if}
            {#if character.description}
              <p class="text-sm text-zinc-600 dark:text-zinc-400">
                <span class="font-medium text-zinc-700 dark:text-zinc-300">Description:</span> {character.description}
              </p>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Main Grid -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Left Column: Attributes & Skills -->
        <div class="space-y-6">
          <!-- Attributes -->
          <div class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5">
            <h3 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 uppercase tracking-wide mb-4">
              Attributes
            </h3>
            <div class="space-y-3">
              {#each character.attributes as attrValue}
                {@const hasModifier = !diceEqual(attrValue.die, attrValue.effective_die)}
                {@const modifiers = hasModifier ? getModifiers("attribute", attrValue.attribute.name) : []}
                <div class="group relative flex items-center justify-between cursor-help">
                  <span class="font-medium text-zinc-700 dark:text-zinc-300">{attrValue.attribute.name}</span>
                  <span class="font-mono font-bold text-lg {hasModifier ? 'text-blue-600 dark:text-blue-400' : 'text-zinc-900 dark:text-zinc-100'}">
                    {formatDie(attrValue.effective_die)}
                  </span>
                  <span class="tooltip">
                    {attrValue.attribute.description}
                    {#if hasModifier}
                      <br/><span class="text-blue-300">Base: {formatDie(attrValue.die)}</span>
                      {#each modifiers as mod}
                        <br/><span class="text-green-300">{mod.value && mod.value > 0 ? '+' : ''}{mod.value} die step from {mod.source}</span>
                      {/each}
                    {/if}
                  </span>
                </div>
              {/each}
            </div>
          </div>

          <!-- Skills -->
          <div class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5">
            <div class="flex items-center justify-between mb-4">
              <h3 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 uppercase tracking-wide">
                Skills
              </h3>
              <label class="flex items-center gap-2 text-xs text-zinc-500 dark:text-zinc-400 cursor-pointer">
                <input
                  type="checkbox"
                  bind:checked={hideUntrainedSkills}
                  class="rounded border-zinc-300 dark:border-zinc-600 text-blue-600 focus:ring-blue-500 dark:bg-zinc-700"
                />
                Hide untrained
              </label>
            </div>
            <div class="space-y-1">
              {#each sortedSkills as skillValue}
                {@const hasSkillModifier = skillValue.die && skillValue.effective_die && !diceEqual(skillValue.die, skillValue.effective_die)}
                {@const attrName = getAttributeName(skillValue.skill.linked_attribute_id)}
                {@const skillModifiers = hasSkillModifier ? getModifiers("skill", skillValue.skill.name) : []}
                <div class="group relative flex items-center justify-between py-1 px-2 rounded cursor-help {skillValue.die ? 'bg-zinc-50 dark:bg-zinc-700/50' : ''}">
                  <span class="text-sm text-zinc-700 dark:text-zinc-300">
                    {skillValue.skill.name}
                    <span class="text-zinc-400 dark:text-zinc-500">({attrName})</span>
                    {#if skillValue.skill.is_core_skill}
                      <span class="text-xs text-blue-500">*</span>
                    {/if}
                  </span>
                  <span class="font-mono text-sm font-medium {hasSkillModifier ? 'text-blue-600 dark:text-blue-400' : skillValue.die ? 'text-zinc-900 dark:text-zinc-100' : 'text-zinc-400 dark:text-zinc-600'}">
                    {formatDie(skillValue.effective_die ?? skillValue.die, true)}
                  </span>
                  <span class="tooltip">
                    {skillValue.skill.description}
                    {#if hasSkillModifier}
                      <br/><span class="text-blue-300">Base: {formatDie(skillValue.die)}</span>
                      {#each skillModifiers as mod}
                        <br/><span class="text-green-300">{mod.value && mod.value > 0 ? '+' : ''}{mod.value} die step from {mod.source}</span>
                      {/each}
                    {/if}
                  </span>
                </div>
              {/each}
            </div>
            <p class="text-xs text-zinc-500 dark:text-zinc-500 mt-4">
              <span class="text-blue-500">*</span> Core skill
            </p>
          </div>
        </div>

        <!-- Center Column: Hindrances, Edges, Powers -->
        <div class="space-y-6">
          <!-- Hindrances -->
          <div class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5">
            <h3 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 uppercase tracking-wide mb-4">
              Hindrances
            </h3>
            {#if character.hindrances.length > 0}
              <div class="space-y-2">
                {#each character.hindrances as h}
                  <div class="group relative flex items-start gap-2 cursor-help">
                    <span class="flex-shrink-0 px-1.5 py-0.5 text-xs rounded {h.hindrance.severity === 'major'
                      ? 'bg-red-100 text-red-700 dark:bg-red-500/20 dark:text-red-400'
                      : 'bg-amber-100 text-amber-700 dark:bg-amber-500/20 dark:text-amber-400'}">
                      {h.hindrance.severity === 'major' ? 'M' : 'm'}
                    </span>
                    <div class="flex-1 min-w-0">
                      <span class="font-medium text-zinc-700 dark:text-zinc-300">{h.hindrance.name}</span>
                      {#if h.source !== 'chosen'}
                        <span class="text-xs text-zinc-500 dark:text-zinc-500 ml-1">({formatSource(h.source)})</span>
                      {/if}
                    </div>
                    <span class="tooltip">
                      {h.hindrance.description}
                    </span>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-sm text-zinc-500 dark:text-zinc-500 italic">None</p>
            {/if}
          </div>

          <!-- Edges -->
          <div class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5">
            <h3 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 uppercase tracking-wide mb-4">
              Edges
            </h3>
            {#if character.edges.length > 0}
              <div class="space-y-2">
                {#each character.edges as e}
                  <div class="group relative flex items-start gap-2 cursor-help">
                    <span class="flex-shrink-0 w-5 h-5 rounded bg-zinc-100 dark:bg-zinc-700 text-xs font-medium text-zinc-600 dark:text-zinc-400 flex items-center justify-center">
                      {getRankAbbrev(character.rank.name)}
                    </span>
                    <div class="flex-1 min-w-0">
                      <span class="font-medium text-zinc-700 dark:text-zinc-300">{e.edge.name}</span>
                      {#if e.source !== 'chosen'}
                        <span class="text-xs text-zinc-500 dark:text-zinc-500 ml-1">({formatSource(e.source)})</span>
                      {/if}
                    </div>
                    <span class="tooltip">
                      <strong class="text-zinc-200">{e.edge.category}</strong><br/>
                      {e.edge.description}
                    </span>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-sm text-zinc-500 dark:text-zinc-500 italic">None</p>
            {/if}
          </div>

          <!-- Powers -->
          <div class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5">
            <h3 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 uppercase tracking-wide mb-4">
              Powers
              {#if character.arcane_backgrounds.length > 0}
                <span class="text-zinc-500 dark:text-zinc-400 font-normal">
                  ({totalPowerPoints} PP)
                </span>
              {/if}
            </h3>

            <!-- Arcane Backgrounds -->
            {#if character.arcane_backgrounds.length > 0}
              <div class="mb-4 pb-4 border-b border-zinc-200 dark:border-zinc-700">
                {#each character.arcane_backgrounds as ab}
                  <div class="group relative cursor-help">
                    <div class="flex items-center gap-2">
                      <span class="px-2 py-0.5 text-xs rounded bg-purple-100 text-purple-700 dark:bg-purple-500/20 dark:text-purple-400">
                        {ab.arcane_background.name}
                      </span>
                      {#if ab.arcane_background.arcane_skill_name}
                        <span class="text-xs text-zinc-500 dark:text-zinc-400">
                          ({ab.arcane_background.arcane_skill_name})
                        </span>
                      {/if}
                    </div>
                    <span class="tooltip">
                      {ab.arcane_background.description}
                    </span>
                  </div>
                {/each}
              </div>
            {/if}

            <!-- Power Points Used -->
            {#if totalPowerPoints > 0}
              <div class="mb-4 pb-4 border-b border-zinc-200 dark:border-zinc-700">
                <div class="flex items-center justify-between mb-2">
                  <div class="text-xs text-zinc-500 dark:text-zinc-400 uppercase tracking-wide">Power Points Used</div>
                  <div class="text-sm font-medium {powerPointsUsed >= totalPowerPoints ? 'text-red-500' : 'text-purple-600 dark:text-purple-400'}">
                    {powerPointsUsed} / {totalPowerPoints}
                  </div>
                </div>
                <ResourcePool
                  label="power point"
                  available={powerPointsUsed}
                  max={totalPowerPoints}
                  color="purple"
                  columns={5}
                  onToggle={togglePowerPoint}
                />
              </div>
            {/if}

            <!-- Powers List -->
            {#if character.powers.length > 0}
              <div class="space-y-3">
                {#each character.powers as p}
                  <div class="group relative border-b border-zinc-100 dark:border-zinc-700 pb-2 last:border-0 last:pb-0 cursor-help">
                    <div class="font-medium text-zinc-700 dark:text-zinc-300">{p.power.name}</div>
                    <div class="text-xs text-zinc-500 dark:text-zinc-400 mt-1">
                      <span>PP: {p.power.power_points}</span>
                      <span class="mx-2">•</span>
                      <span>Range: {p.power.range}</span>
                      <span class="mx-2">•</span>
                      <span>Duration: {p.power.duration}</span>
                    </div>
                    <span class="tooltip">
                      {p.power.description}
                    </span>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-sm text-zinc-500 dark:text-zinc-500 italic">None</p>
            {/if}
          </div>

          <!-- Gear -->
          <div class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5">
            <div class="flex items-center justify-between mb-4">
              <h3 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 uppercase tracking-wide">
                Gear
                <span class="text-zinc-500 dark:text-zinc-400 font-normal">
                  ({character.encumbrance.current_weight.toFixed(1)} / {character.encumbrance.load_limit} lbs)
                </span>
              </h3>
              <button
                onclick={() => gearBrowserOpen = true}
                class="text-sm text-blue-600 dark:text-blue-400 hover:text-blue-500 font-medium flex items-center gap-1"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
                Add Gear
              </button>
            </div>

            <!-- Encumbrance warning -->
            {#if character.encumbrance.is_encumbered}
              <div class="mb-4 p-2 bg-amber-50 dark:bg-amber-500/10 border border-amber-200 dark:border-amber-500/20 rounded text-sm text-amber-700 dark:text-amber-400">
                <strong>Encumbered!</strong> -2 to all physical tasks.
              </div>
            {/if}

            <!-- Gear list -->
            {#if character.gear.length > 0}
              <div class="space-y-2">
                {#each character.gear as item}
                  <GearItem
                    {item}
                    onToggleEquip={handleToggleEquip}
                    onSell={handleSellGear}
                    onRemove={handleRemoveGear}
                  />
                {/each}
              </div>
            {:else}
              <p class="text-sm text-zinc-500 dark:text-zinc-500 italic">No gear</p>
            {/if}
          </div>
        </div>

        <!-- Right Column: Status & Tracking -->
        <div class="space-y-6">
          <!-- Wounds & Fatigue -->
          <div class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5">
            <h3 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 uppercase tracking-wide mb-4">
              Status
            </h3>

            <!-- Wounds -->
            <div class="mb-4">
              <StatusTracker
                label="Wounds"
                value={wounds}
                regularSlots={3}
                color="red"
                showIncapacitated={true}
                onToggle={toggleWound}
              />
            </div>

            <!-- Fatigue -->
            <StatusTracker
              label="Fatigue"
              value={fatigue}
              regularSlots={2}
              color="amber"
              showIncapacitated={true}
              onToggle={toggleFatigue}
            />
          </div>

          <!-- Bennies -->
          <div class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5">
            <h3 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 uppercase tracking-wide mb-4">
              Bennies
              <span class="text-zinc-500 dark:text-zinc-400 font-normal">
                ({benniesAvailable} / 6)
              </span>
            </h3>
            <ResourcePool
              label="benny"
              available={benniesAvailable}
              max={6}
              color="amber"
              columns={6}
              resetValue={3}
              onToggle={toggleBenny}
              onReset={() => benniesAvailable = 3}
            />
          </div>

          <!-- Point Pools -->
          <div class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5">
            <h3 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 uppercase tracking-wide mb-4">
              Point Allocation
            </h3>
            <div class="space-y-2 text-sm">
              <div class="flex justify-between">
                <span class="text-zinc-600 dark:text-zinc-400">Attribute Points</span>
                <span class="font-medium text-zinc-900 dark:text-zinc-100">
                  {character.attribute_points_spent} / {character.attribute_points_earned + character.hindrance_points_to_attributes}
                </span>
              </div>
              <div class="flex justify-between">
                <span class="text-zinc-600 dark:text-zinc-400">Skill Points</span>
                <span class="font-medium text-zinc-900 dark:text-zinc-100">
                  {character.skill_points_spent} / {character.skill_points_earned + character.hindrance_points_to_skills}
                </span>
              </div>
              {#if character.hindrance_points_earned > 0}
                <div class="flex justify-between">
                  <span class="text-zinc-600 dark:text-zinc-400">Hindrance Points</span>
                  <span class="font-medium text-amber-600 dark:text-amber-400">
                    {character.hindrance_points_earned}
                  </span>
                </div>
                {#if character.hindrance_points_to_attributes > 0}
                  <div class="flex justify-between text-xs">
                    <span class="text-zinc-500 dark:text-zinc-500 pl-2">→ Attributes</span>
                    <span class="text-zinc-500 dark:text-zinc-500">+{character.hindrance_points_to_attributes * 2}</span>
                  </div>
                {/if}
                {#if character.hindrance_points_to_skills > 0}
                  <div class="flex justify-between text-xs">
                    <span class="text-zinc-500 dark:text-zinc-500 pl-2">→ Skills</span>
                    <span class="text-zinc-500 dark:text-zinc-500">+{character.hindrance_points_to_skills}</span>
                  </div>
                {/if}
                {#if character.hindrance_points_to_edges > 0}
                  <div class="flex justify-between text-xs">
                    <span class="text-zinc-500 dark:text-zinc-500 pl-2">→ Edges</span>
                    <span class="text-zinc-500 dark:text-zinc-500">+{character.hindrance_points_to_edges}</span>
                  </div>
                {/if}
              {/if}
            </div>
          </div>

          <!-- Modifiers (if any) -->
          {#if character.modifiers.length > 0}
            <div class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5">
              <h3 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 uppercase tracking-wide mb-4">
                Active Modifiers
              </h3>
              <div class="space-y-2">
                {#each character.modifiers as mod}
                  <div class="text-sm text-zinc-600 dark:text-zinc-400">
                    {mod.description}
                  </div>
                {/each}
              </div>
            </div>
          {/if}

          <!-- Advancement History -->
          {#key advancementHistoryKey}
            <AdvancementHistory
              characterId={character.id}
              onUndo={async () => {
                // Reload character after undo
                if (!character) return;
                const result = await commands.getCharacter(character.id);
                if (result.status === "ok" && result.data) {
                  character = result.data;
                  notes = result.data.notes;
                }
              }}
            />
          {/key}
        </div>
      </div>
    {/if}
  </main>
</div>

<!-- Notes Panel -->
<NotesPanel
  {notes}
  isOpen={notesOpen}
  onOpen={() => notesOpen = true}
  onClose={() => notesOpen = false}
  onCreate={handleCreateNote}
  onUpdate={handleUpdateNote}
  onDelete={handleDeleteNote}
/>

<!-- Advancement Modal -->
{#if advancementModalOpen && character}
  <AdvancementModal
    {character}
    onClose={() => advancementModalOpen = false}
    onAdvanceTaken={handleAdvanceTaken}
  />
{/if}

<!-- Gear Browser Modal -->
{#if gearBrowserOpen && character}
  <GearBrowserModal
    {character}
    onClose={() => gearBrowserOpen = false}
    onGearChanged={handleGearChanged}
  />
{/if}

<!-- Delete Confirmation Modal -->
{#if showDeleteConfirm && character}
  <ConfirmDeleteModal
    title="Delete Character"
    message="Are you sure you want to permanently delete this character? This action cannot be undone."
    itemName={character.name}
    onCancel={() => showDeleteConfirm = false}
    onConfirm={handleDelete}
  />
{/if}

<style>
  /* Tooltip styles */
  .tooltip {
    visibility: hidden;
    opacity: 0;
    position: absolute;
    z-index: 50;
    padding: 0.5rem 0.75rem;
    font-size: 0.875rem;
    color: #f4f4f5; /* zinc-100 */
    background-color: #18181b; /* zinc-900 */
    border-radius: 0.5rem;
    box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);
    transition: all 0.2s ease;
    pointer-events: none;
    left: 0;
    top: 100%;
    margin-top: 0.5rem;
    min-width: 200px;
    max-width: 300px;
    white-space: normal;
    line-height: 1.4;
  }

  :global(.dark) .tooltip {
    background-color: #3f3f46; /* zinc-700 */
  }

  .group:hover .tooltip {
    visibility: visible;
    opacity: 1;
  }

  /* Tooltip arrow */
  .tooltip::before {
    content: '';
    position: absolute;
    top: -0.5rem;
    left: 1rem;
    border: 4px solid transparent;
    border-bottom-color: #18181b; /* zinc-900 */
  }

  :global(.dark) .tooltip::before {
    border-bottom-color: #3f3f46; /* zinc-700 */
  }
</style>
