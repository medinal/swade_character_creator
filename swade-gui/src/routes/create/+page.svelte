<script lang="ts">
  import { commands, type CharacterView } from "$lib/bindings";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import WizardLayout from "$lib/components/WizardLayout.svelte";

  let character = $state<CharacterView | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let error = $state<string | null>(null);

  // Form fields
  let name = $state("");
  let isWildCard = $state(true);
  let background = $state("");
  let description = $state("");

  onMount(async () => {
    // Check if there's an existing draft, otherwise create a new one
    const draftResult = await commands.getDraftCharacter();
    if (draftResult.status === "ok" && draftResult.data) {
      character = draftResult.data;
      syncFormFromCharacter();
    } else {
      // Create a new draft character
      const createResult = await commands.createCharacter("New Character");
      if (createResult.status === "ok") {
        character = createResult.data;
        syncFormFromCharacter();
      } else {
        error = createResult.error.message;
      }
    }
    loading = false;
  });

  function syncFormFromCharacter() {
    if (!character) return;
    name = character.name;
    isWildCard = character.is_wild_card;
    background = character.background ?? "";
    description = character.description ?? "";
  }

  async function saveBasicInfo() {
    saving = true;
    error = null;

    const result = await commands.updateDraftBasicInfo(
      name.trim() || "Unnamed Character",
      isWildCard,
      background.trim() || null,
      description.trim() || null
    );

    if (result.status === "ok") {
      character = result.data;
    } else {
      error = result.error.message;
    }
    saving = false;
  }

  async function handleDiscard() {
    const result = await commands.discardDraft();
    if (result.status === "ok") {
      goto("/");
    } else {
      error = result.error.message;
    }
  }

  async function handleSave() {
    saving = true;
    error = null;

    // First update the basic info
    const updateResult = await commands.updateDraftBasicInfo(
      name.trim() || "Unnamed Character",
      isWildCard,
      background.trim() || null,
      description.trim() || null
    );

    if (updateResult.status !== "ok") {
      error = updateResult.error.message;
      saving = false;
      return;
    }

    // Then save to database
    const saveResult = await commands.saveCharacter();
    if (saveResult.status === "ok") {
      // Update local state with saved character (now has an id)
      character = saveResult.data;
    } else {
      error = saveResult.error.message;
    }
    saving = false;
  }

  async function handleContinue() {
    saving = true;
    error = null;

    // Save basic info before navigating
    const updateResult = await commands.updateDraftBasicInfo(
      name.trim() || "Unnamed Character",
      isWildCard,
      background.trim() || null,
      description.trim() || null
    );

    if (updateResult.status !== "ok") {
      error = updateResult.error.message;
      saving = false;
      return;
    }

    // Navigate to ancestry step
    goto("/create/ancestry");
  }

  async function handleSaveBeforeSheet(): Promise<number | null> {
    saving = true;
    error = null;

    // First update the basic info
    const updateResult = await commands.updateDraftBasicInfo(
      name.trim() || "Unnamed Character",
      isWildCard,
      background.trim() || null,
      description.trim() || null
    );

    if (updateResult.status !== "ok") {
      error = updateResult.error.message;
      saving = false;
      return null;
    }

    // Then save to database
    const saveResult = await commands.saveCharacter();
    saving = false;

    if (saveResult.status === "ok") {
      character = saveResult.data;
      return saveResult.data.id;
    } else {
      error = saveResult.error.message;
      return null;
    }
  }

  // Auto-save on blur
  async function handleBlur() {
    if (character) {
      await saveBasicInfo();
    }
  }
</script>

<WizardLayout
  characterName={character?.name}
  characterId={character?.id ?? 0}
  currentStep={1}
  {saving}
  {loading}
  {error}
  onDiscard={handleDiscard}
  onSave={handleSave}
  onNavigate={saveBasicInfo}
  onSaveBeforeSheet={handleSaveBeforeSheet}
  onBack={handleDiscard}
  onContinue={handleContinue}
  backLabel="Cancel"
  showBackArrow={false}
>
  <!-- Basic Info Form -->
  <div class="space-y-6">
    <div>
      <h2 class="text-lg font-semibold text-zinc-900 dark:text-zinc-100 mb-1">Basic Info</h2>
      <p class="text-sm text-zinc-500 dark:text-zinc-400">Enter your character's name and basic details.</p>
    </div>

    <!-- Name -->
    <div>
      <label for="name" class="block text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-1.5">
        Character Name
      </label>
      <input
        id="name"
        type="text"
        bind:value={name}
        onblur={handleBlur}
        placeholder="Enter character name"
        class="w-full px-3 py-2 bg-white dark:bg-zinc-800 border border-zinc-300 dark:border-zinc-700 rounded-md text-zinc-900 dark:text-zinc-100 placeholder-zinc-400 dark:placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors"
      />
    </div>

    <!-- Character Type -->
    <div>
      <span id="character-type-label" class="block text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-1.5">
        Character Type
      </span>
      <div class="flex gap-3" role="group" aria-labelledby="character-type-label">
        <button
          onclick={() => { isWildCard = true; handleBlur(); }}
          class="flex-1 px-4 py-3 rounded-lg border-2 transition-all {isWildCard
            ? 'border-violet-500 bg-violet-50 dark:bg-violet-500/10'
            : 'border-zinc-200 dark:border-zinc-700 hover:border-zinc-300 dark:hover:border-zinc-600'}"
        >
          <div class="flex items-center gap-2 mb-1">
            <span class="inline-block px-1.5 py-0.5 bg-violet-100 text-violet-600 dark:bg-violet-500/20 dark:text-violet-400 text-xs rounded font-medium">
              Wild Card
            </span>
          </div>
          <p class="text-xs text-zinc-500 dark:text-zinc-400 text-left">
            Player characters and major NPCs. Rolls a Wild Die on trait tests.
          </p>
        </button>
        <button
          onclick={() => { isWildCard = false; handleBlur(); }}
          class="flex-1 px-4 py-3 rounded-lg border-2 transition-all {!isWildCard
            ? 'border-zinc-500 bg-zinc-50 dark:bg-zinc-500/10'
            : 'border-zinc-200 dark:border-zinc-700 hover:border-zinc-300 dark:hover:border-zinc-600'}"
        >
          <div class="flex items-center gap-2 mb-1">
            <span class="inline-block px-1.5 py-0.5 bg-zinc-100 text-zinc-600 dark:bg-zinc-500/20 dark:text-zinc-400 text-xs rounded font-medium">
              Extra
            </span>
          </div>
          <p class="text-xs text-zinc-500 dark:text-zinc-400 text-left">
            Minor NPCs and henchmen. No Wild Die, fewer wounds.
          </p>
        </button>
      </div>
    </div>

    <!-- Background -->
    <div>
      <label for="background" class="block text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-1.5">
        Background
        <span class="text-zinc-400 dark:text-zinc-500 font-normal">(optional)</span>
      </label>
      <textarea
        id="background"
        bind:value={background}
        onblur={handleBlur}
        placeholder="Where does your character come from? What shaped them?"
        rows="3"
        class="w-full px-3 py-2 bg-white dark:bg-zinc-800 border border-zinc-300 dark:border-zinc-700 rounded-md text-zinc-900 dark:text-zinc-100 placeholder-zinc-400 dark:placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors resize-none"
      ></textarea>
    </div>

    <!-- Description -->
    <div>
      <label for="description" class="block text-sm font-medium text-zinc-700 dark:text-zinc-300 mb-1.5">
        Description
        <span class="text-zinc-400 dark:text-zinc-500 font-normal">(optional)</span>
      </label>
      <textarea
        id="description"
        bind:value={description}
        onblur={handleBlur}
        placeholder="What does your character look like? Any distinguishing features?"
        rows="3"
        class="w-full px-3 py-2 bg-white dark:bg-zinc-800 border border-zinc-300 dark:border-zinc-700 rounded-md text-zinc-900 dark:text-zinc-100 placeholder-zinc-400 dark:placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors resize-none"
      ></textarea>
    </div>
  </div>
</WizardLayout>
