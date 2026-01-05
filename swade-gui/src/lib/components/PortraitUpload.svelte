<script lang="ts">
  import { commands, type CharacterView } from "$lib/bindings";

  interface Props {
    character: CharacterView;
    onPortraitChanged: (character: CharacterView) => void;
  }

  let { character, onPortraitChanged }: Props = $props();

  let uploading = $state(false);
  let error = $state<string | null>(null);
  let fileInput = $state<HTMLInputElement | null>(null);
  let showModal = $state(false);

  const MAX_SIZE = 1024; // Maximum dimension in pixels

  async function resizeImage(file: File): Promise<{ base64: string; mimeType: string }> {
    return new Promise((resolve, reject) => {
      const img = new Image();
      const reader = new FileReader();

      reader.onload = (e) => {
        img.src = e.target?.result as string;
      };

      img.onload = () => {
        const canvas = document.createElement("canvas");
        const ctx = canvas.getContext("2d");
        if (!ctx) {
          reject(new Error("Could not get canvas context"));
          return;
        }

        // Calculate new dimensions maintaining aspect ratio
        let width = img.width;
        let height = img.height;

        if (width > MAX_SIZE || height > MAX_SIZE) {
          if (width > height) {
            height = Math.round((height * MAX_SIZE) / width);
            width = MAX_SIZE;
          } else {
            width = Math.round((width * MAX_SIZE) / height);
            height = MAX_SIZE;
          }
        }

        canvas.width = width;
        canvas.height = height;

        // Draw with high quality
        ctx.imageSmoothingEnabled = true;
        ctx.imageSmoothingQuality = "high";
        ctx.drawImage(img, 0, 0, width, height);

        // Get the data URL - use JPEG for photos (smaller), PNG for transparency
        const mimeType = file.type === "image/png" ? "image/png" : "image/jpeg";
        const quality = mimeType === "image/jpeg" ? 0.9 : undefined;
        const dataUrl = canvas.toDataURL(mimeType, quality);

        // Extract base64 data (remove "data:image/xxx;base64," prefix)
        const base64 = dataUrl.split(",")[1];

        resolve({ base64, mimeType });
      };

      img.onerror = () => reject(new Error("Failed to load image"));
      reader.onerror = () => reject(new Error("Failed to read file"));

      reader.readAsDataURL(file);
    });
  }

  async function handleFileSelect(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;

    // Validate file type
    if (!file.type.startsWith("image/")) {
      error = "Please select an image file";
      return;
    }

    uploading = true;
    error = null;

    try {
      // Resize the image
      const { base64, mimeType } = await resizeImage(file);

      // Upload to backend
      const result = await commands.updateCharacterPortrait(character.id, base64, mimeType);

      if (result.status === "ok") {
        onPortraitChanged(result.data);
        showModal = false;
      } else {
        error = typeof result.error === "string" ? result.error : result.error.message;
      }
    } catch (e) {
      error = e instanceof Error ? e.message : "Failed to upload portrait";
    } finally {
      uploading = false;
      // Reset the input so the same file can be selected again
      if (input) input.value = "";
    }
  }

  async function handleClear() {
    uploading = true;
    error = null;

    try {
      const result = await commands.clearCharacterPortrait(character.id);

      if (result.status === "ok") {
        onPortraitChanged(result.data);
        showModal = false;
      } else {
        error = typeof result.error === "string" ? result.error : result.error.message;
      }
    } catch (e) {
      error = e instanceof Error ? e.message : "Failed to clear portrait";
    } finally {
      uploading = false;
    }
  }

  function triggerFileInput() {
    fileInput?.click();
  }

  function handlePortraitClick() {
    if (character.portrait_data_url) {
      showModal = true;
    } else {
      triggerFileInput();
    }
  }

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      showModal = false;
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      showModal = false;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="portrait-upload">
  <input
    bind:this={fileInput}
    type="file"
    accept="image/png,image/jpeg,image/jpg"
    class="hidden"
    onchange={handleFileSelect}
  />

  <div class="relative group">
    {#if character.portrait_data_url}
      <!-- Has portrait -->
      <button
        onclick={handlePortraitClick}
        disabled={uploading}
        class="w-24 h-24 rounded-full overflow-hidden border-2 border-zinc-300 dark:border-zinc-600 hover:border-blue-500 dark:hover:border-blue-400 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
      >
        <img
          src={character.portrait_data_url}
          alt={`${character.name}'s portrait`}
          class="w-full h-full object-cover"
        />
      </button>

      <!-- Overlay on hover -->
      <div class="absolute inset-0 w-24 h-24 rounded-full bg-black/50 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center pointer-events-none">
        <span class="text-white text-xs font-medium">View</span>
      </div>
    {:else}
      <!-- No portrait - show placeholder -->
      <button
        onclick={handlePortraitClick}
        disabled={uploading}
        class="w-24 h-24 rounded-full border-2 border-dashed border-zinc-300 dark:border-zinc-600 hover:border-blue-500 dark:hover:border-blue-400 bg-zinc-100 dark:bg-zinc-800 flex flex-col items-center justify-center text-zinc-400 dark:text-zinc-500 hover:text-blue-500 dark:hover:text-blue-400 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
      >
        {#if uploading}
          <svg class="w-6 h-6 animate-spin" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
        {:else}
          <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
          </svg>
          <span class="text-xs mt-1">Add Photo</span>
        {/if}
      </button>
    {/if}
  </div>

  {#if error}
    <div class="mt-2 text-xs text-red-500 max-w-24 text-center">{error}</div>
  {/if}
</div>

<!-- Full-size portrait modal -->
{#if showModal && character.portrait_data_url}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/70"
    onclick={handleBackdropClick}
    onkeydown={(e) => e.key === "Escape" && (showModal = false)}
    role="dialog"
    aria-modal="true"
    aria-label="Portrait preview"
    tabindex="-1"
  >
    <div class="relative max-w-lg max-h-[80vh] mx-4">
      <!-- Close button -->
      <button
        onclick={() => showModal = false}
        class="absolute -top-10 right-0 text-white/70 hover:text-white transition-colors"
        aria-label="Close"
      >
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>

      <!-- Portrait image -->
      <img
        src={character.portrait_data_url}
        alt={`${character.name}'s portrait`}
        class="max-w-full max-h-[70vh] rounded-lg shadow-2xl object-contain"
      />

      <!-- Action buttons -->
      <div class="flex justify-center gap-3 mt-4">
        <button
          onclick={triggerFileInput}
          disabled={uploading}
          class="px-4 py-2 bg-blue-600 hover:bg-blue-500 text-white text-sm font-medium rounded-md transition-colors disabled:opacity-50"
        >
          {#if uploading}
            Uploading...
          {:else}
            Change Photo
          {/if}
        </button>
        <button
          onclick={handleClear}
          disabled={uploading}
          class="px-4 py-2 bg-zinc-600 hover:bg-zinc-500 text-white text-sm font-medium rounded-md transition-colors disabled:opacity-50"
        >
          Remove
        </button>
      </div>
    </div>
  </div>
{/if}
