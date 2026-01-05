/**
 * Svelte context for wizard state and callbacks.
 * Eliminates prop drilling through WizardLayout -> WizardHeader -> StepNav.
 */

import { getContext, setContext } from "svelte";

const WIZARD_CONTEXT_KEY = Symbol("wizard");

export interface WizardCallbacks {
  onDiscard: () => void;
  onSave: () => void;
  onNavigate?: () => Promise<void>;
  onSaveBeforeSheet?: () => Promise<number | null>;
  onBack?: () => void;
  onContinue: () => void;
}

export interface WizardState {
  characterName?: string;
  characterId: number;
  currentStep: number;
  saving: boolean;
  loading: boolean;
  error: string | null;
  backLabel: string;
  continueLabel: string;
  showBackArrow: boolean;
  callbacks: WizardCallbacks;
}

export function setWizardContext(state: WizardState): void {
  setContext(WIZARD_CONTEXT_KEY, state);
}

export function getWizardContext(): WizardState {
  const ctx = getContext<WizardState>(WIZARD_CONTEXT_KEY);
  if (!ctx) {
    throw new Error("Wizard context not found. Make sure this component is used within WizardLayout.");
  }
  return ctx;
}
