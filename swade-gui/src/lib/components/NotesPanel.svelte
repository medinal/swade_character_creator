<script lang="ts">
  import { Editor } from "@tiptap/core";
  import StarterKit from "@tiptap/starter-kit";
  import Placeholder from "@tiptap/extension-placeholder";
  import type { CharacterNoteValue } from "$lib/bindings";

  interface Props {
    notes: CharacterNoteValue[];
    isOpen: boolean;
    onOpen: () => void;
    onClose: () => void;
    onCreate: (title: string, body: string) => Promise<void>;
    onUpdate: (noteId: number, title: string, body: string) => Promise<void>;
    onDelete: (noteId: number) => Promise<void>;
  }

  let { notes, isOpen, onOpen, onClose, onCreate, onUpdate, onDelete }: Props = $props();

  // Track which note is being edited
  let editingNoteId: number | null = $state(null);
  let editTitle = $state("");
  let editBody = $state("");

  // Track if creating a new note
  let isCreating = $state(false);
  let newTitle = $state("");
  let newBody = $state("");

  // Reference for auto-focus
  let newTitleInput = $state<HTMLInputElement | null>(null);
  let editTitleInput = $state<HTMLInputElement | null>(null);

  // Tiptap editor instances
  let newEditor: Editor | null = $state(null);
  let editEditor: Editor | null = $state(null);
  let newEditorElement = $state<HTMLElement | null>(null);
  let editEditorElement = $state<HTMLElement | null>(null);

  function createEditor(element: HTMLElement, content: string, placeholder: string): Editor {
    return new Editor({
      element,
      extensions: [
        StarterKit.configure({
          heading: { levels: [1, 2, 3] },
        }),
        Placeholder.configure({ placeholder }),
      ],
      content,
      editorProps: {
        attributes: {
          class: "outline-none min-h-[100px] px-3 py-2",
        },
      },
    });
  }

  function destroyEditor(editor: Editor | null) {
    if (editor) {
      editor.destroy();
    }
  }

  // Resize state
  const MIN_WIDTH = 280;
  const MAX_WIDTH = 800;
  const DEFAULT_WIDTH = 384; // 24rem = 384px
  let panelWidth = $state(DEFAULT_WIDTH);
  let isResizing = $state(false);

  function startResize(e: MouseEvent) {
    e.preventDefault();
    isResizing = true;
    document.addEventListener("mousemove", handleResize);
    document.addEventListener("mouseup", stopResize);
    document.body.style.cursor = "ew-resize";
    document.body.style.userSelect = "none";
  }

  function handleResize(e: MouseEvent) {
    if (!isResizing) return;
    const newWidth = window.innerWidth - e.clientX;
    panelWidth = Math.max(MIN_WIDTH, Math.min(MAX_WIDTH, newWidth));
  }

  function stopResize() {
    isResizing = false;
    document.removeEventListener("mousemove", handleResize);
    document.removeEventListener("mouseup", stopResize);
    document.body.style.cursor = "";
    document.body.style.userSelect = "";
  }

  $effect(() => {
    if (isCreating && newTitleInput) {
      newTitleInput.focus();
    }
  });

  $effect(() => {
    if (editingNoteId !== null && editTitleInput) {
      editTitleInput.focus();
    }
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      if (isCreating) {
        cancelCreate();
      } else if (editingNoteId !== null) {
        cancelEdit();
      } else {
        onClose();
      }
    }
  }

  function startCreate() {
    // Default title to current datetime
    const now = new Date();
    newTitle = now.toLocaleString();
    newBody = "";
    isCreating = true;
    // Editor will be created when element is mounted
  }

  function cancelCreate() {
    destroyEditor(newEditor);
    newEditor = null;
    isCreating = false;
    newTitle = "";
    newBody = "";
  }

  async function saveNewNote() {
    if (!newTitle.trim()) return;
    const content = newEditor?.getHTML() ?? "";
    await onCreate(newTitle.trim(), content);
    cancelCreate();
  }

  function startEdit(note: CharacterNoteValue) {
    editingNoteId = note.id;
    editTitle = note.title;
    editBody = note.body;
    // Editor will be created when element is mounted
  }

  function cancelEdit() {
    destroyEditor(editEditor);
    editEditor = null;
    editingNoteId = null;
    editTitle = "";
    editBody = "";
  }

  async function saveEdit() {
    if (editingNoteId === null || !editTitle.trim()) return;
    const content = editEditor?.getHTML() ?? "";
    await onUpdate(editingNoteId, editTitle.trim(), content);
    cancelEdit();
  }

  async function handleDelete(noteId: number) {
    // Cancel edit first to clean up editor state
    cancelEdit();
    await onDelete(noteId);
  }

  function formatDate(dateStr: string): string {
    try {
      // Parse as UTC (database stores UTC timestamps)
      const date = new Date(dateStr + "Z");
      return date.toLocaleDateString() + " " + date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
    } catch {
      return dateStr;
    }
  }

  // Svelte actions for initializing editors
  function initNewEditor(node: HTMLElement) {
    newEditor = createEditor(node, "", "Write your note here...");
    newEditor.commands.focus();
    return {
      destroy() {
        destroyEditor(newEditor);
        newEditor = null;
      }
    };
  }

  function initEditEditor(node: HTMLElement) {
    editEditor = createEditor(node, editBody, "Write your note here...");
    editEditor.commands.focus();
    return {
      destroy() {
        destroyEditor(editEditor);
        editEditor = null;
      }
    };
  }
</script>

<!-- Pull Tab - Always visible on right edge -->
<button
  onclick={() => isOpen ? onClose() : onOpen()}
  class="fixed right-0 top-1/2 -translate-y-1/2 z-40 bg-zinc-700 dark:bg-zinc-600 hover:bg-zinc-600 dark:hover:bg-zinc-500 text-white px-1.5 py-4 rounded-l-lg shadow-lg transition-all {isOpen ? 'opacity-0 pointer-events-none' : 'opacity-100'}"
  aria-label="Open notes"
  title="Notes{notes.length > 0 ? ` (${notes.length})` : ''}"
>
  <div class="flex flex-col items-center gap-1">
    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
    </svg>
    {#if notes.length > 0}
      <span class="text-xs font-medium">{notes.length}</span>
    {/if}
  </div>
</button>

<!-- Backdrop -->
<div
  class="fixed inset-0 bg-black/30 z-40 transition-opacity duration-300 {isOpen ? 'opacity-100' : 'opacity-0 pointer-events-none'}"
  onclick={onClose}
  onkeydown={handleKeydown}
  role="button"
  tabindex="-1"
  aria-label="Close notes panel"
></div>

<!-- Panel -->
<div
  class="fixed top-0 right-0 h-full bg-white dark:bg-zinc-800 shadow-xl z-50 flex flex-col transition-transform duration-300 ease-out {isOpen ? 'translate-x-0' : 'translate-x-full'}"
  style="width: {Math.min(panelWidth, window.innerWidth * 0.9)}px;"
  role="dialog"
  aria-modal="true"
  aria-labelledby="notes-panel-title"
  onkeydown={handleKeydown}
  tabindex="-1"
>
    <!-- Resize Handle - Using focusable separator per WAI-ARIA 1.2 spec -->
    <button
      type="button"
      class="absolute left-0 top-0 bottom-0 w-1 cursor-ew-resize hover:bg-blue-500/50 transition-colors border-0 bg-transparent p-0 {isResizing ? 'bg-blue-500' : ''}"
      onmousedown={startResize}
      onkeydown={(e) => { if (e.key === "ArrowLeft") { panelWidth = Math.min(panelWidth + 10, 800); } else if (e.key === "ArrowRight") { panelWidth = Math.max(panelWidth - 10, 280); } }}
      aria-label="Resize panel"
    ></button>

    <!-- Header -->
    <div class="flex items-center justify-between p-4 border-b border-zinc-200 dark:border-zinc-700">
      <h2 id="notes-panel-title" class="text-lg font-semibold text-zinc-900 dark:text-zinc-100">
        Notes
      </h2>
      <button
        onclick={onClose}
        class="p-1 rounded-md hover:bg-zinc-100 dark:hover:bg-zinc-700 transition-colors"
        aria-label="Close"
      >
        <svg class="w-5 h-5 text-zinc-500" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    <!-- Add Note Button -->
    {#if !isCreating}
      <div class="p-4 border-b border-zinc-200 dark:border-zinc-700">
        <button
          onclick={startCreate}
          class="w-full px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-500 rounded-md transition-colors flex items-center justify-center gap-2"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          Add Note
        </button>
      </div>
    {/if}

    <!-- New Note Form -->
    {#if isCreating}
      <div class="p-4 border-b border-zinc-200 dark:border-zinc-700 bg-blue-50 dark:bg-zinc-700/50">
        <input
          bind:this={newTitleInput}
          type="text"
          bind:value={newTitle}
          placeholder="Note title"
          maxlength="100"
          class="w-full px-3 py-2 mb-2 text-sm rounded-md border border-zinc-300 dark:border-zinc-600 bg-white dark:bg-zinc-700 text-zinc-900 dark:text-zinc-100 placeholder-zinc-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <div
          bind:this={newEditorElement}
          class="w-full mb-2 text-sm rounded-md border border-zinc-300 dark:border-zinc-600 bg-white dark:bg-zinc-700 text-zinc-900 dark:text-zinc-100 focus-within:ring-2 focus-within:ring-blue-500 tiptap-editor"
          use:initNewEditor
        ></div>
        <div class="flex justify-end gap-2">
          <button
            onclick={cancelCreate}
            class="px-3 py-1 text-sm text-zinc-600 dark:text-zinc-400 hover:bg-zinc-100 dark:hover:bg-zinc-600 rounded-md transition-colors"
          >
            Cancel
          </button>
          <button
            onclick={saveNewNote}
            disabled={!newTitle.trim()}
            class="px-3 py-1 text-sm font-medium text-white bg-blue-600 hover:bg-blue-500 disabled:bg-zinc-400 disabled:cursor-not-allowed rounded-md transition-colors"
          >
            Save
          </button>
        </div>
      </div>
    {/if}

    <!-- Notes List -->
    <div class="flex-1 overflow-y-auto">
      {#if notes.length === 0 && !isCreating}
        <div class="p-8 text-center text-zinc-500 dark:text-zinc-400">
          <svg class="w-12 h-12 mx-auto mb-3 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
          <p class="text-sm">No notes yet</p>
          <p class="text-xs mt-1">Click "Add Note" to create one</p>
        </div>
      {:else}
        {#each notes as note (note.id)}
          <div class="border-b border-zinc-200 dark:border-zinc-700">
            {#if editingNoteId === note.id}
              <!-- Edit Mode -->
              <div class="p-4 bg-yellow-50 dark:bg-zinc-700/50">
                <input
                  bind:this={editTitleInput}
                  type="text"
                  bind:value={editTitle}
                  placeholder="Note title"
                  maxlength="100"
                  class="w-full px-3 py-2 mb-2 text-sm rounded-md border border-zinc-300 dark:border-zinc-600 bg-white dark:bg-zinc-700 text-zinc-900 dark:text-zinc-100 placeholder-zinc-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
                <div
                  bind:this={editEditorElement}
                  class="w-full mb-2 text-sm rounded-md border border-zinc-300 dark:border-zinc-600 bg-white dark:bg-zinc-700 text-zinc-900 dark:text-zinc-100 focus-within:ring-2 focus-within:ring-blue-500 tiptap-editor"
                  use:initEditEditor
                ></div>
                <div class="flex justify-between gap-2">
                  <button
                    onclick={() => handleDelete(note.id)}
                    class="px-3 py-1 text-sm text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-md transition-colors"
                  >
                    Delete
                  </button>
                  <div class="flex gap-2">
                    <button
                      onclick={cancelEdit}
                      class="px-3 py-1 text-sm text-zinc-600 dark:text-zinc-400 hover:bg-zinc-100 dark:hover:bg-zinc-600 rounded-md transition-colors"
                    >
                      Cancel
                    </button>
                    <button
                      onclick={saveEdit}
                      disabled={!editTitle.trim()}
                      class="px-3 py-1 text-sm font-medium text-white bg-blue-600 hover:bg-blue-500 disabled:bg-zinc-400 disabled:cursor-not-allowed rounded-md transition-colors"
                    >
                      Save
                    </button>
                  </div>
                </div>
              </div>
            {:else}
              <!-- Collapsed View - Click to expand/edit -->
              <button
                onclick={() => startEdit(note)}
                class="w-full p-4 text-left hover:bg-zinc-50 dark:hover:bg-zinc-700/50 transition-colors"
              >
                <div class="flex items-center justify-between gap-2">
                  <h3 class="font-medium text-zinc-900 dark:text-zinc-100 text-sm truncate">
                    {note.title}
                  </h3>
                  <span class="text-xs text-zinc-500 dark:text-zinc-400 flex-shrink-0">
                    {formatDate(note.created_at)}
                  </span>
                </div>
              </button>
            {/if}
          </div>
        {/each}
      {/if}
    </div>
  </div>

<style>
  /* Tiptap editor styles */
  .tiptap-editor :global(.tiptap) {
    outline: none;
    min-height: 100px;
    padding: 0.5rem 0.75rem;
  }
  .tiptap-editor :global(.tiptap p.is-editor-empty:first-child::before) {
    color: #a1a1aa;
    content: attr(data-placeholder);
    float: left;
    height: 0;
    pointer-events: none;
  }
  .tiptap-editor :global(.tiptap p) {
    margin-bottom: 0.5rem;
  }
  .tiptap-editor :global(.tiptap p:last-child) {
    margin-bottom: 0;
  }
  .tiptap-editor :global(.tiptap h1) {
    font-size: 1.5rem;
    font-weight: 700;
    margin-top: 0.75rem;
    margin-bottom: 0.5rem;
  }
  .tiptap-editor :global(.tiptap h2) {
    font-size: 1.25rem;
    font-weight: 600;
    margin-top: 0.75rem;
    margin-bottom: 0.5rem;
  }
  .tiptap-editor :global(.tiptap h3) {
    font-size: 1.125rem;
    font-weight: 600;
    margin-top: 0.5rem;
    margin-bottom: 0.25rem;
  }
  .tiptap-editor :global(.tiptap ul) {
    list-style-type: disc;
    margin-left: 1.25rem;
    margin-bottom: 0.5rem;
  }
  .tiptap-editor :global(.tiptap ol) {
    list-style-type: decimal;
    margin-left: 1.25rem;
    margin-bottom: 0.5rem;
  }
  .tiptap-editor :global(.tiptap li) {
    margin-bottom: 0.125rem;
  }
  .tiptap-editor :global(.tiptap strong) {
    font-weight: 600;
  }
  .tiptap-editor :global(.tiptap em) {
    font-style: italic;
  }
  .tiptap-editor :global(.tiptap code) {
    background-color: rgba(0, 0, 0, 0.05);
    padding: 0.125rem 0.25rem;
    border-radius: 0.25rem;
    font-family: ui-monospace, monospace;
    font-size: 0.875em;
  }
  :global(.dark) .tiptap-editor :global(.tiptap code) {
    background-color: rgba(255, 255, 255, 0.1);
  }
  .tiptap-editor :global(.tiptap pre) {
    background-color: rgba(0, 0, 0, 0.05);
    padding: 0.5rem;
    border-radius: 0.375rem;
    overflow-x: auto;
    margin-bottom: 0.5rem;
  }
  :global(.dark) .tiptap-editor :global(.tiptap pre) {
    background-color: rgba(255, 255, 255, 0.1);
  }
  .tiptap-editor :global(.tiptap blockquote) {
    border-left: 3px solid #d1d5db;
    padding-left: 0.75rem;
    margin-left: 0;
    margin-bottom: 0.5rem;
    color: #6b7280;
  }
  :global(.dark) .tiptap-editor :global(.tiptap blockquote) {
    border-left-color: #4b5563;
    color: #9ca3af;
  }
  .tiptap-editor :global(.tiptap hr) {
    border: none;
    border-top: 1px solid #e5e7eb;
    margin: 0.75rem 0;
  }
  :global(.dark) .tiptap-editor :global(.tiptap hr) {
    border-top-color: #374151;
  }
</style>
