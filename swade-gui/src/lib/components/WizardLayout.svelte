<script lang="ts">
  import type { Snippet } from "svelte";
  import WizardHeader from "./WizardHeader.svelte";
  import WizardFooter from "./WizardFooter.svelte";
  import ErrorAlert from "./ErrorAlert.svelte";
  import { setWizardContext, type WizardState } from "$lib/contexts/wizardContext";

  interface Props {
    // Header props
    characterName?: string;
    characterId: number;
    currentStep: number;
    saving: boolean;
    loading: boolean;
    error: string | null;
    onDiscard: () => void;
    onSave: () => void;
    onNavigate?: () => Promise<void>;
    onSaveBeforeSheet?: () => Promise<number | null>;
    // Footer props
    onBack?: () => void;
    onContinue: () => void;
    backLabel?: string;
    continueLabel?: string;
    showBackArrow?: boolean;
    // Content
    children: Snippet;
  }

  let {
    characterName,
    characterId,
    currentStep,
    saving,
    loading,
    error,
    onDiscard,
    onSave,
    onNavigate,
    onSaveBeforeSheet,
    onBack,
    onContinue,
    backLabel = "Back",
    continueLabel = "Continue",
    showBackArrow = true,
    children,
  }: Props = $props();

  // Set wizard context for child components (called synchronously during init)
  // Using a getter function allows context consumers to access reactive values
  setWizardContext({
    get characterName() { return characterName; },
    get characterId() { return characterId; },
    get currentStep() { return currentStep; },
    get saving() { return saving; },
    get loading() { return loading; },
    get error() { return error; },
    get backLabel() { return backLabel; },
    get continueLabel() { return continueLabel; },
    get showBackArrow() { return showBackArrow; },
    callbacks: {
      get onDiscard() { return onDiscard; },
      get onSave() { return onSave; },
      get onNavigate() { return onNavigate; },
      get onSaveBeforeSheet() { return onSaveBeforeSheet; },
      get onBack() { return onBack; },
      get onContinue() { return onContinue; },
    },
  });
</script>

<div class="min-h-screen bg-zinc-100 text-zinc-900 dark:bg-zinc-900 dark:text-zinc-100 flex flex-col">
  <WizardHeader />

  <main class="flex-1">
    <div class="max-w-4xl mx-auto px-6 py-8 pb-24">
      {#if loading}
        <div class="text-zinc-500 text-sm">Loading...</div>
      {:else}
        <ErrorAlert {error} />
        {@render children()}
      {/if}
    </div>
  </main>

  <WizardFooter />
</div>
