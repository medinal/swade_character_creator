<script lang="ts">
  import { commands, type CharacterView, type CharacterSkillValue } from "$lib/bindings";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import WizardLayout from "$lib/components/WizardLayout.svelte";
  import EdgeImpactWarningModal from "$lib/components/EdgeImpactWarningModal.svelte";
  import HindrancePointsCard from "$lib/components/HindrancePointsCard.svelte";
  import SourceFilter from "$lib/components/SourceFilter.svelte";
  import { formatDie, diceEqual } from "$lib/character-utils";
  import { calculateHindrancePointsAllocated, calculateHindrancePointsAvailable } from "$lib/utils/hindrancePoints";
  import {
    loadDraftOrRedirect,
    discardDraftAndNavigate,
    saveCharacter,
    saveCharacterAndGetId,
    WIZARD_STEPS,
  } from "$lib/utils/wizardOperations";

  let character = $state<CharacterView | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let error = $state<string | null>(null);

  let expandedAttribute = $state<number | null>(null);
  let expandedSkill = $state<number | null>(null);

  // Source filter
  let sourceFilter = $state("all");

  // Edge impact warning modal
  let edgeWarningModal = $state<{ skillId: number; skillName: string; affectedEdges: string[] } | null>(null);

  // Filter character skills by source
  let filteredSkills = $derived(
    sourceFilter === "all"
      ? (character?.skills ?? [])
      : (character?.skills ?? []).filter(s => s.skill.source === sourceFilter)
  );

  onMount(async () => {
    const draft = await loadDraftOrRedirect();
    if (!draft) return;

    character = draft;
    loading = false;
  });

  // Group skills by linked attribute
  let skillsByAttribute = $derived.by(() => {
    if (!character) return new Map<number, CharacterSkillValue[]>();

    const map = new Map<number, CharacterSkillValue[]>();
    for (const skillValue of filteredSkills) {
      const attrId = skillValue.skill.linked_attribute_id;
      const existing = map.get(attrId) ?? [];
      existing.push(skillValue);
      map.set(attrId, existing);
    }
    // Sort skills alphabetically within each group
    for (const [key, skills] of map) {
      map.set(key, skills.sort((a, b) => a.skill.name.localeCompare(b.skill.name)));
    }
    return map;
  });

  // Calculate points
  let baseSkillPoints = $derived(character?.skill_points_earned ?? 12);
  let bonusSkillPoints = $derived(character?.hindrance_points_to_skills ?? 0);
  let totalSkillPoints = $derived(baseSkillPoints + bonusSkillPoints);
  let pointsSpent = $derived(character?.skill_points_spent ?? 0);
  let pointsRemaining = $derived(totalSkillPoints - pointsSpent);

  // Hindrance points available for conversion
  let hindrancePointsEarned = $derived(character?.hindrance_points_earned ?? 0);
  let hindrancePointsAllocated = $derived(calculateHindrancePointsAllocated(character));
  let hindrancePointsAvailable = $derived(calculateHindrancePointsAvailable(character));

  function toggleAttributeExpand(attributeId: number) {
    expandedAttribute = expandedAttribute === attributeId ? null : attributeId;
  }

  function toggleSkillExpand(skillId: number) {
    expandedSkill = expandedSkill === skillId ? null : skillId;
  }

  async function incrementSkill(skillId: number) {
    saving = true;
    error = null;

    const result = await commands.updateDraftSkill(skillId, true);
    if (result.status === "ok") {
      character = result.data;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function decrementSkill(skillId: number, force: boolean = false) {
    // Check if decrementing would invalidate any edges
    if (!force) {
      const impactResult = await commands.checkSkillDecrementImpact(skillId);
      if (impactResult.status === "ok" && impactResult.data.length > 0) {
        // Show warning modal
        const skillValue = character?.skills.find(s => s.skill.id === skillId);
        edgeWarningModal = {
          skillId,
          skillName: skillValue?.skill.name ?? "this skill",
          affectedEdges: impactResult.data,
        };
        return;
      }
    }

    saving = true;
    error = null;

    const result = await commands.updateDraftSkill(skillId, false);
    if (result.status === "ok") {
      character = result.data;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  function confirmSkillDecrement() {
    if (edgeWarningModal) {
      const skillId = edgeWarningModal.skillId;
      edgeWarningModal = null;
      decrementSkill(skillId, true);
    }
  }

  function cancelSkillDecrement() {
    edgeWarningModal = null;
  }

  async function addHindrancePointToSkills() {
    saving = true;
    error = null;

    const result = await commands.allocateHindrancePointsToSkills(1);
    if (result.status === "ok") {
      character = result.data;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function removeHindrancePointFromSkills() {
    saving = true;
    error = null;

    const result = await commands.allocateHindrancePointsToSkills(-1);
    if (result.status === "ok") {
      character = result.data;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function handleBack() {
    goto(WIZARD_STEPS.attributes);
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
    goto(WIZARD_STEPS.edges);
  }
</script>

<WizardLayout
  characterName={character?.name}
  characterId={character?.id ?? 0}
  currentStep={5}
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
    <!-- Skills List grouped by Attribute -->
    <div class="lg:col-span-2">
      <div class="mb-4">
        <div class="flex items-center justify-between mb-1">
          <h2 class="text-lg font-semibold text-zinc-900 dark:text-zinc-100">
            Assign Skills
          </h2>
          <SourceFilter value={sourceFilter} onChange={(v) => sourceFilter = v} />
        </div>
        <p class="text-sm text-zinc-500 dark:text-zinc-400">
          Distribute points among your skills. Skills are grouped by their linked attribute.
          Raising a skill above its linked attribute costs 2 points instead of 1.
        </p>
      </div>

      <div class="space-y-2">
        {#each character?.attributes ?? [] as attrValue}
          {@const attributeSkills = skillsByAttribute.get(attrValue.attribute.id) ?? []}
          {#if attributeSkills.length > 0}
            {@const isExpanded = expandedAttribute === attrValue.attribute.id}
            <div class="rounded-lg border border-zinc-200 dark:border-zinc-700">
              <!-- Attribute Header -->
              <button
                onclick={() => toggleAttributeExpand(attrValue.attribute.id)}
                class="w-full px-4 py-3 flex items-center justify-between bg-zinc-50 dark:bg-zinc-800/50 hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-colors {isExpanded ? 'rounded-t-lg' : 'rounded-lg'}"
              >
                <div class="flex items-center gap-3">
                  <span class="font-medium text-zinc-900 dark:text-zinc-100">
                    {attrValue.attribute.name}
                  </span>
                  <span class="text-sm font-mono text-zinc-600 dark:text-zinc-400">
                    {formatDie(attrValue.effective_die)}
                  </span>
                  <span class="text-xs text-zinc-500 dark:text-zinc-400">
                    ({attributeSkills.length} skill{attributeSkills.length !== 1 ? 's' : ''})
                  </span>
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

              <!-- Skills in this attribute group -->
              {#if isExpanded}
                <div class="divide-y divide-zinc-100 dark:divide-zinc-800">
                  {#each attributeSkills as skillValue}
                    {@const hasModifier = skillValue.die && skillValue.effective_die && !diceEqual(skillValue.die, skillValue.effective_die)}
                    {@const isSkillExpanded = expandedSkill === skillValue.skill.id}
                    <div>
                      <div class="px-4 py-3 flex items-center justify-between">
                        <button
                          onclick={() => toggleSkillExpand(skillValue.skill.id)}
                          class="flex items-center gap-2 text-left hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
                        >
                          <svg
                            class="w-3 h-3 text-zinc-400 transition-transform {isSkillExpanded ? 'rotate-90' : ''}"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                          >
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                          </svg>
                          <span class="text-zinc-900 dark:text-zinc-100">
                            {skillValue.skill.name}
                          </span>
                          {#if skillValue.skill.is_core_skill}
                            <span class="text-xs text-blue-500">*</span>
                          {/if}
                          {#if skillValue.is_above_attribute}
                            <span class="text-xs px-1.5 py-0.5 rounded bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-400">
                              2pt/step
                            </span>
                          {/if}
                        </button>

                        <!-- Die controls -->
                        <div class="flex items-center gap-1">
                          <button
                            onclick={() => decrementSkill(skillValue.skill.id)}
                            disabled={saving || !skillValue.can_decrement}
                            class="w-8 h-8 flex items-center justify-center rounded-md text-zinc-500 hover:bg-zinc-100 dark:hover:bg-zinc-800 disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
                            aria-label="Decrease {skillValue.skill.name}"
                          >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4" />
                            </svg>
                          </button>
                          <span class="w-14 text-center font-mono font-semibold {hasModifier ? 'text-blue-600 dark:text-blue-400' : 'text-zinc-900 dark:text-zinc-100'}">
                            {formatDie(skillValue.effective_die ?? skillValue.die, true)}
                          </span>
                          <button
                            onclick={() => incrementSkill(skillValue.skill.id)}
                            disabled={saving || !skillValue.can_increment}
                            class="w-8 h-8 flex items-center justify-center rounded-md text-zinc-500 hover:bg-zinc-100 dark:hover:bg-zinc-800 disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
                            aria-label="Increase {skillValue.skill.name}"
                          >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                            </svg>
                          </button>
                          {#if skillValue.can_increment}
                            <span class="text-xs text-zinc-400 w-8 text-right">
                              {skillValue.increment_cost}pt
                            </span>
                          {:else}
                            <span class="w-8"></span>
                          {/if}
                        </div>
                      </div>

                      <!-- Skill description -->
                      {#if isSkillExpanded}
                        <div class="px-4 pb-3 pl-9">
                          <p class="text-sm text-zinc-600 dark:text-zinc-400">
                            {skillValue.skill.description}
                          </p>
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

      <p class="text-xs text-zinc-500 dark:text-zinc-500 mt-4">
        <span class="text-blue-500">*</span> Core skill (starts at d4)
      </p>
    </div>

    <!-- Points Summary Sidebar -->
    <div>
      <div class="sticky top-24 space-y-4">
        <!-- Skill Points -->
        <div class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5">
          <h3 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 mb-3">
            Skill Points
          </h3>

          <div class="mb-4">
            <div class="flex items-center justify-between mb-2">
              <span class="text-sm text-zinc-600 dark:text-zinc-400">Points Remaining</span>
              <span class="text-lg font-semibold {pointsRemaining < 0 ? 'text-red-500' : 'text-zinc-900 dark:text-zinc-100'}">
                {pointsRemaining} / {totalSkillPoints}
              </span>
            </div>
            <div class="w-full bg-zinc-200 dark:bg-zinc-700 rounded-full h-2">
              <div
                class="h-2 rounded-full transition-all {pointsRemaining < 0 ? 'bg-red-500' : 'bg-blue-600'}"
                style="width: {Math.min(100, (pointsSpent / totalSkillPoints) * 100)}%"
              ></div>
            </div>
          </div>

          <div class="text-xs text-zinc-500 dark:text-zinc-400 space-y-1">
            <div class="flex justify-between">
              <span>Base points:</span>
              <span>{baseSkillPoints}</span>
            </div>
            {#if bonusSkillPoints > 0}
              <div class="flex justify-between text-amber-600 dark:text-amber-400">
                <span>From hindrances:</span>
                <span>+{bonusSkillPoints}</span>
              </div>
            {/if}
            <div class="flex justify-between font-medium pt-1 border-t border-zinc-200 dark:border-zinc-700">
              <span>Spent:</span>
              <span>{pointsSpent}</span>
            </div>
          </div>
        </div>

        <!-- Hindrance Points Conversion -->
        {#if hindrancePointsEarned > 0}
          <HindrancePointsCard
            {hindrancePointsEarned}
            {hindrancePointsAvailable}
            pointsAllocated={bonusSkillPoints}
            targetLabel="skills"
            description="Trade 1 hindrance point for 1 skill point."
            costPerUnit={1}
            disabled={saving}
            canDecrease={bonusSkillPoints > 0}
            onIncrease={addHindrancePointToSkills}
            onDecrease={removeHindrancePointFromSkills}
          />
        {/if}

        <!-- Quick Reference -->
        <div class="bg-zinc-50 dark:bg-zinc-800/50 rounded-lg border border-zinc-200 dark:border-zinc-700 p-4">
          <h4 class="text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide mb-2">
            Quick Reference
          </h4>
          <ul class="text-xs text-zinc-600 dark:text-zinc-400 space-y-1">
            <li>Train new skill: 1pt</li>
            <li>At or below attribute: 1pt/step</li>
            <li>Above linked attribute: 2pts/step</li>
            <li>Untrained rolls: d4-2</li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</WizardLayout>

{#if edgeWarningModal}
  <EdgeImpactWarningModal
    targetName={edgeWarningModal.skillName}
    affectedEdges={edgeWarningModal.affectedEdges}
    onCancel={cancelSkillDecrement}
    onConfirm={confirmSkillDecrement}
  />
{/if}
