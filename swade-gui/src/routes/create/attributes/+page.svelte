<script lang="ts">
  import { commands, type CharacterView, type SkillView } from "$lib/bindings";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import WizardLayout from "$lib/components/WizardLayout.svelte";
  import EdgeImpactWarningModal from "$lib/components/EdgeImpactWarningModal.svelte";
  import HindrancePointsCard from "$lib/components/HindrancePointsCard.svelte";
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
  let allSkills = $state<SkillView[]>([]);
  let loading = $state(true);
  let saving = $state(false);
  let error = $state<string | null>(null);

  let expandedAttribute = $state<number | null>(null);

  // Edge impact warning modal
  let edgeWarningModal = $state<{ attributeId: number; attributeName: string; affectedEdges: string[] } | null>(null);

  onMount(async () => {
    const draft = await loadDraftOrRedirect();
    if (!draft) return;

    character = draft;

    // Load all skills to display under attributes
    const skillsResult = await commands.getSkills();
    if (skillsResult.status === "ok") {
      allSkills = skillsResult.data;
    } else {
      error = skillsResult.error.message;
    }

    loading = false;
  });

  // Group skills by linked attribute
  let skillsByAttribute = $derived.by(() => {
    const map = new Map<number, SkillView[]>();
    for (const skill of allSkills) {
      const existing = map.get(skill.linked_attribute_id) ?? [];
      existing.push(skill);
      map.set(skill.linked_attribute_id, existing);
    }
    // Sort skills alphabetically within each group
    for (const [key, skills] of map) {
      map.set(key, skills.sort((a, b) => a.name.localeCompare(b.name)));
    }
    return map;
  });

  // Calculate points
  let baseAttributePoints = $derived(character?.attribute_points_earned ?? 5);
  let bonusAttributePoints = $derived(character?.hindrance_points_to_attributes ?? 0);
  let totalAttributePoints = $derived(baseAttributePoints + bonusAttributePoints);
  let pointsSpent = $derived(character?.attribute_points_spent ?? 0);
  let pointsRemaining = $derived(totalAttributePoints - pointsSpent);

  // Hindrance points available for conversion
  let hindrancePointsEarned = $derived(character?.hindrance_points_earned ?? 0);
  let hindrancePointsAllocated = $derived(calculateHindrancePointsAllocated(character));
  let hindrancePointsAvailable = $derived(calculateHindrancePointsAvailable(character));

  function toggleAttributeExpand(attributeId: number) {
    expandedAttribute = expandedAttribute === attributeId ? null : attributeId;
  }

  async function incrementAttribute(attributeId: number) {
    saving = true;
    error = null;

    const result = await commands.updateDraftAttribute(attributeId, true, null);
    if (result.status === "ok") {
      character = result.data.character;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function decrementAttribute(attributeId: number, force: boolean = false) {
    // Check if decrementing would invalidate any edges
    if (!force) {
      const impactResult = await commands.checkAttributeDecrementImpact(attributeId);
      if (impactResult.status === "ok" && impactResult.data.length > 0) {
        // Show warning modal
        const attrValue = character?.attributes.find(a => a.attribute.id === attributeId);
        edgeWarningModal = {
          attributeId,
          attributeName: attrValue?.attribute.name ?? "this attribute",
          affectedEdges: impactResult.data,
        };
        return;
      }
    }

    saving = true;
    error = null;

    const result = await commands.updateDraftAttribute(attributeId, false, null);
    if (result.status === "ok") {
      character = result.data.character;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  function confirmAttributeDecrement() {
    if (edgeWarningModal) {
      const attributeId = edgeWarningModal.attributeId;
      edgeWarningModal = null;
      decrementAttribute(attributeId, true);
    }
  }

  function cancelAttributeDecrement() {
    edgeWarningModal = null;
  }

  async function addHindrancePointToAttributes() {
    saving = true;
    error = null;

    const result = await commands.allocateHindrancePointsToAttributes(1);
    if (result.status === "ok") {
      character = result.data;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function removeHindrancePointFromAttributes() {
    saving = true;
    error = null;

    const result = await commands.allocateHindrancePointsToAttributes(-1);
    if (result.status === "ok") {
      character = result.data;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function handleBack() {
    goto(WIZARD_STEPS.hindrances);
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
    goto(WIZARD_STEPS.skills);
  }
</script>

<WizardLayout
  characterName={character?.name}
  characterId={character?.id ?? 0}
  currentStep={4}
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
    <!-- Attributes List -->
    <div class="lg:col-span-2">
      <div class="mb-4">
        <h2 class="text-lg font-semibold text-zinc-900 dark:text-zinc-100 mb-1">
          Assign Attributes
        </h2>
        <p class="text-sm text-zinc-500 dark:text-zinc-400">
          Distribute points among your five attributes. Each starts at d4. Click to see related skills.
        </p>
      </div>

      <div class="space-y-1">
        {#each character?.attributes ?? [] as attrValue}
          {@const isExpanded = expandedAttribute === attrValue.attribute.id}
          {@const relatedSkills = skillsByAttribute.get(attrValue.attribute.id) ?? []}
          {@const hasModifier = !diceEqual(attrValue.die, attrValue.effective_die)}
          <div class="rounded-lg border border-zinc-200 dark:border-zinc-700 transition-all">
            <!-- Attribute header -->
            <div class="flex items-center">
              <button
                onclick={() => toggleAttributeExpand(attrValue.attribute.id)}
                class="flex-1 px-3 py-3 flex items-center justify-between gap-2 text-left hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors rounded-l-lg"
              >
                <div class="flex items-center gap-3">
                  <span class="font-medium text-zinc-900 dark:text-zinc-100">{attrValue.attribute.name}</span>
                  <span class="text-xs text-zinc-500 dark:text-zinc-400">
                    {relatedSkills.length} skill{relatedSkills.length !== 1 ? 's' : ''}
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

              <!-- Die controls -->
              <div class="flex items-center gap-1 px-3 border-l border-zinc-200 dark:border-zinc-700">
                <button
                  onclick={() => decrementAttribute(attrValue.attribute.id)}
                  disabled={saving || !attrValue.can_decrement}
                  class="w-8 h-8 flex items-center justify-center rounded-md text-zinc-500 hover:bg-zinc-100 dark:hover:bg-zinc-800 disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
                  aria-label="Decrease {attrValue.attribute.name}"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4" />
                  </svg>
                </button>
                <span class="w-14 text-center font-mono font-semibold {hasModifier ? 'text-blue-600 dark:text-blue-400' : 'text-zinc-900 dark:text-zinc-100'}">
                  {formatDie(attrValue.effective_die)}
                </span>
                <button
                  onclick={() => incrementAttribute(attrValue.attribute.id)}
                  disabled={saving || !attrValue.can_increment}
                  class="w-8 h-8 flex items-center justify-center rounded-md text-zinc-500 hover:bg-zinc-100 dark:hover:bg-zinc-800 disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
                  aria-label="Increase {attrValue.attribute.name}"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                  </svg>
                </button>
              </div>
            </div>

            <!-- Expanded content: Related skills -->
            {#if isExpanded}
              <div class="px-3 pb-3 border-t border-zinc-100 dark:border-zinc-800">
                <p class="text-sm text-zinc-600 dark:text-zinc-400 mt-3 mb-3">
                  {attrValue.attribute.description}
                </p>

                {#if relatedSkills.length > 0}
                  <div class="mt-3">
                    <h4 class="text-xs font-semibold text-zinc-500 dark:text-zinc-400 uppercase tracking-wide mb-2">
                      Related Skills
                    </h4>
                    <div class="grid grid-cols-2 gap-1">
                      {#each relatedSkills as skill}
                        {@const charSkill = character?.skills.find(s => s.skill.id === skill.id)}
                        {@const hasSkillModifier = charSkill?.die && charSkill?.effective_die && !diceEqual(charSkill.die, charSkill.effective_die)}
                        <div class="flex items-center justify-between px-2 py-1.5 rounded bg-zinc-50 dark:bg-zinc-800/50 text-sm">
                          <span class="text-zinc-700 dark:text-zinc-300">
                            {skill.name}
                            {#if skill.is_core_skill}
                              <span class="text-xs text-blue-500">*</span>
                            {/if}
                          </span>
                          <span class="text-xs font-mono {hasSkillModifier ? 'text-blue-500' : 'text-zinc-500 dark:text-zinc-400'}">
                            {formatDie(charSkill?.effective_die ?? charSkill?.die, true)}
                          </span>
                        </div>
                      {/each}
                    </div>
                    <p class="text-xs text-zinc-500 dark:text-zinc-500 mt-2">
                      <span class="text-blue-500">*</span> Core skill
                    </p>
                  </div>
                {/if}
              </div>
            {/if}
          </div>
        {/each}
      </div>
    </div>

    <!-- Points Summary -->
    <div>
      <div class="sticky top-24 space-y-4">
        <!-- Attribute Points -->
        <div class="bg-white dark:bg-zinc-800 rounded-lg border border-zinc-200 dark:border-zinc-700 p-5">
          <h3 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100 mb-3">
            Attribute Points
          </h3>

          <div class="mb-4">
            <div class="flex items-center justify-between mb-2">
              <span class="text-sm text-zinc-600 dark:text-zinc-400">Points Remaining</span>
              <span class="text-lg font-semibold {pointsRemaining < 0 ? 'text-red-500' : 'text-zinc-900 dark:text-zinc-100'}">
                {pointsRemaining} / {totalAttributePoints}
              </span>
            </div>
            <div class="w-full bg-zinc-200 dark:bg-zinc-700 rounded-full h-2">
              <div
                class="h-2 rounded-full transition-all {pointsRemaining < 0 ? 'bg-red-500' : 'bg-blue-600'}"
                style="width: {Math.min(100, (pointsSpent / totalAttributePoints) * 100)}%"
              ></div>
            </div>
          </div>

          <div class="text-xs text-zinc-500 dark:text-zinc-400 space-y-1">
            <div class="flex justify-between">
              <span>Base points:</span>
              <span>{baseAttributePoints}</span>
            </div>
            {#if bonusAttributePoints > 0}
              <div class="flex justify-between text-amber-600 dark:text-amber-400">
                <span>From hindrances:</span>
                <span>+{bonusAttributePoints}</span>
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
            pointsAllocated={bonusAttributePoints * 2}
            targetLabel="attributes"
            description="Trade 2 hindrance points for 1 attribute point."
            costPerUnit={2}
            disabled={saving}
            canDecrease={bonusAttributePoints > 0}
            onIncrease={addHindrancePointToAttributes}
            onDecrease={removeHindrancePointFromAttributes}
          />
        {/if}
      </div>
    </div>
  </div>
</WizardLayout>

{#if edgeWarningModal}
  <EdgeImpactWarningModal
    targetName={edgeWarningModal.attributeName}
    affectedEdges={edgeWarningModal.affectedEdges}
    onCancel={cancelAttributeDecrement}
    onConfirm={confirmAttributeDecrement}
  />
{/if}
