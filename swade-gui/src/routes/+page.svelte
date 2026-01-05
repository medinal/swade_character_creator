<script lang="ts">
  import { commands, type CharacterView } from "$lib/bindings";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import ConfirmDeleteModal from "$lib/components/ConfirmDeleteModal.svelte";

  let characters = $state<CharacterView[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let sortBy = $state<"name" | "rank" | "ancestry">("name");
  let sortAsc = $state(true);
  let characterToDelete = $state<CharacterView | null>(null);
  let deleting = $state(false);

  onMount(() => {
    loadCharacters();
  });

  async function loadCharacters() {
    loading = true;
    error = null;
    const result = await commands.getCharacters();
    if (result.status === "ok") {
      characters = result.data;
    } else {
      error = result.error.message;
    }
    loading = false;
  }

  function toggleSort(column: "name" | "rank" | "ancestry") {
    if (sortBy === column) {
      sortAsc = !sortAsc;
    } else {
      sortBy = column;
      sortAsc = true;
    }
  }

  let sortedCharacters = $derived(() => {
    const sorted = [...characters].sort((a, b) => {
      let aVal: string, bVal: string;
      switch (sortBy) {
        case "name":
          aVal = a.name.toLowerCase();
          bVal = b.name.toLowerCase();
          break;
        case "rank":
          aVal = a.rank.name.toLowerCase();
          bVal = b.rank.name.toLowerCase();
          break;
        case "ancestry":
          aVal = a.ancestry?.name.toLowerCase() ?? "";
          bVal = b.ancestry?.name.toLowerCase() ?? "";
          break;
        default:
          return 0;
      }
      if (aVal < bVal) return sortAsc ? -1 : 1;
      if (aVal > bVal) return sortAsc ? 1 : -1;
      return 0;
    });
    return sorted;
  });

  function getSortIcon(column: string) {
    if (sortBy !== column) return "↕";
    return sortAsc ? "↑" : "↓";
  }

  function navigateToCreate() {
    // Discard any existing draft and navigate to create page
    commands.discardDraft().then(() => {
      goto("/create");
    });
  }

  function navigateToCharacter(id: number) {
    goto(`/character/${id}`);
  }

  function handleDeleteClick(e: MouseEvent, character: CharacterView) {
    e.stopPropagation();
    characterToDelete = character;
  }

  async function confirmDelete() {
    if (!characterToDelete) return;

    deleting = true;
    const result = await commands.deleteCharacter(characterToDelete.id);
    if (result.status === "ok") {
      characters = characters.filter((c) => c.id !== characterToDelete!.id);
      characterToDelete = null;
    } else {
      error = typeof result.error === "string" ? result.error : result.error.message;
    }
    deleting = false;
  }
</script>

<div class="min-h-screen bg-zinc-100 text-zinc-900 dark:bg-zinc-900 dark:text-zinc-100">
  <!-- Header -->
  <header class="bg-white border-b border-zinc-200 dark:bg-zinc-900 dark:border-zinc-800">
    <div class="max-w-5xl mx-auto px-6 py-3 flex items-center justify-between">
      <h1 class="text-sm font-medium text-zinc-500 dark:text-zinc-400">SWADE Character Builder</h1>
      <button
        onclick={navigateToCreate}
        class="bg-blue-600 hover:bg-blue-500 text-white text-sm font-medium px-3 py-1.5 rounded-md transition-colors"
      >
        New Character
      </button>
    </div>
  </header>

  <main class="max-w-5xl mx-auto px-6 py-6">
    <!-- Page Title -->
    <div class="mb-5">
      <h2 class="text-lg font-medium text-zinc-900 dark:text-zinc-100">Characters</h2>
      <p class="text-zinc-500 text-sm">
        {#if loading}
          Loading...
        {:else}
          {characters.length} character{characters.length !== 1 ? "s" : ""}
        {/if}
      </p>
    </div>

    {#if error}
      <div class="bg-red-50 border border-red-200 text-red-600 dark:bg-red-500/10 dark:border-red-500/20 dark:text-red-400 px-4 py-3 rounded-md mb-6 text-sm">
        Error: {error}
      </div>
    {/if}

    {#if loading}
      <div class="text-zinc-500 text-sm">Loading characters...</div>
    {:else if characters.length === 0}
      <div class="text-center py-16">
        <p class="text-zinc-500 mb-4">No characters yet</p>
        <button
          onclick={navigateToCreate}
          class="bg-blue-600 hover:bg-blue-500 text-white text-sm font-medium px-4 py-2 rounded-md transition-colors"
        >
          Create Your First Character
        </button>
      </div>
    {:else}
      <!-- Character Table -->
      <div class="rounded-lg border border-zinc-200 dark:border-zinc-800 overflow-hidden">
        <table class="w-full">
          <thead>
            <tr class="border-b border-zinc-200 dark:border-zinc-800 text-left text-xs text-zinc-500 bg-zinc-50 dark:bg-zinc-800/50">
              <th class="px-4 py-2.5 font-medium w-12"></th>
              <th class="px-4 py-2.5 font-medium">
                <button
                  onclick={() => toggleSort("name")}
                  class="hover:text-zinc-700 dark:hover:text-zinc-300 transition-colors flex items-center gap-1"
                >
                  Name
                  <span class="text-zinc-400 dark:text-zinc-600">{getSortIcon("name")}</span>
                </button>
              </th>
              <th class="px-4 py-2.5 font-medium">
                <button
                  onclick={() => toggleSort("rank")}
                  class="hover:text-zinc-700 dark:hover:text-zinc-300 transition-colors flex items-center gap-1"
                >
                  Rank
                  <span class="text-zinc-400 dark:text-zinc-600">{getSortIcon("rank")}</span>
                </button>
              </th>
              <th class="px-4 py-2.5 font-medium">
                <button
                  onclick={() => toggleSort("ancestry")}
                  class="hover:text-zinc-700 dark:hover:text-zinc-300 transition-colors flex items-center gap-1"
                >
                  Ancestry
                  <span class="text-zinc-400 dark:text-zinc-600">{getSortIcon("ancestry")}</span>
                </button>
              </th>
              <th class="px-4 py-2.5 font-medium">Type</th>
              <th class="px-4 py-2.5 font-medium w-12"></th>
            </tr>
          </thead>
          <tbody class="text-sm bg-white dark:bg-transparent">
            {#each sortedCharacters() as character}
              <tr
                onclick={() => navigateToCharacter(character.id)}
                onkeydown={(e) => e.key === 'Enter' && navigateToCharacter(character.id)}
                role="button"
                tabindex="0"
                aria-label="View {character.name}"
                class="border-b border-zinc-100 dark:border-zinc-800/50 hover:bg-zinc-50 dark:hover:bg-zinc-800/30 focus:bg-zinc-50 dark:focus:bg-zinc-800/30 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-inset transition-colors cursor-pointer"
              >
                <td class="px-4 py-2.5">
                  {#if character.portrait_data_url}
                    <img
                      src={character.portrait_data_url}
                      alt=""
                      class="w-8 h-8 rounded-full object-cover"
                    />
                  {:else}
                    <div class="w-8 h-8 rounded-full bg-zinc-200 dark:bg-zinc-700 flex items-center justify-center">
                      <svg class="w-4 h-4 text-zinc-400 dark:text-zinc-500" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                      </svg>
                    </div>
                  {/if}
                </td>
                <td class="px-4 py-2.5">
                  <span class="font-medium text-zinc-900 dark:text-zinc-100">{character.name}</span>
                </td>
                <td class="px-4 py-2.5 text-zinc-600 dark:text-zinc-400">
                  {character.rank.name}
                </td>
                <td class="px-4 py-2.5 text-zinc-600 dark:text-zinc-400">
                  {character.ancestry?.name ?? "—"}
                </td>
                <td class="px-4 py-2.5">
                  {#if character.is_wild_card}
                    <span class="inline-block px-1.5 py-0.5 bg-violet-100 text-violet-600 dark:bg-violet-500/15 dark:text-violet-400 text-xs rounded font-medium">
                      Wild Card
                    </span>
                  {:else}
                    <span class="text-zinc-400 dark:text-zinc-600 text-xs">Extra</span>
                  {/if}
                </td>
                <td class="px-4 py-2.5 text-right">
                  <button
                    onclick={(e) => handleDeleteClick(e, character)}
                    class="p-1.5 bg-red-600 hover:bg-red-500 text-white rounded transition-colors"
                    title="Delete character"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                  </button>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </main>
</div>

{#if characterToDelete}
  <ConfirmDeleteModal
    title="Delete Character"
    message="Are you sure you want to permanently delete this character? This action cannot be undone."
    itemName={characterToDelete.name}
    onCancel={() => characterToDelete = null}
    onConfirm={confirmDelete}
  />
{/if}
