/**
 * Shared utility functions for wizard page operations.
 *
 * These functions wrap common Tauri command patterns used across
 * multiple wizard pages, simplifying error handling and navigation.
 */

import { goto } from "$app/navigation";
import { commands, type CharacterView } from "$lib/bindings";

/**
 * Result type for wizard operations
 */
export type WizardResult<T> =
  | { ok: true; data: T }
  | { ok: false; error: string };

/**
 * Wizard step paths in order
 */
export const WIZARD_STEPS = {
  basics: "/create",
  ancestry: "/create/ancestry",
  hindrances: "/create/hindrances",
  attributes: "/create/attributes",
  skills: "/create/skills",
  edges: "/create/edges",
  powers: "/create/powers",
} as const;

/**
 * Load the draft character, redirecting to /create if not found.
 * Returns the character or null if redirected.
 */
export async function loadDraftOrRedirect(): Promise<CharacterView | null> {
  const result = await commands.getDraftCharacter();
  if (result.status === "ok" && result.data) {
    return result.data;
  }
  await goto("/create");
  return null;
}

/**
 * Discard the draft character and navigate to home.
 * Returns an error message if the operation failed, or null on success.
 */
export async function discardDraftAndNavigate(): Promise<string | null> {
  const result = await commands.discardDraft();
  if (result.status === "ok") {
    await goto("/");
    return null;
  }
  return result.error.message;
}

/**
 * Save the current character.
 * Returns a WizardResult with the updated character or an error.
 */
export async function saveCharacter(): Promise<WizardResult<CharacterView>> {
  const result = await commands.saveCharacter();
  if (result.status === "ok") {
    return { ok: true, data: result.data };
  }
  return { ok: false, error: result.error.message };
}

/**
 * Save the current character and return its ID for navigation.
 * Useful for "Save & View Sheet" functionality.
 * Returns the character ID on success, or null with error message on failure.
 */
export async function saveCharacterAndGetId(): Promise<{ id: number | null; error: string | null }> {
  const result = await commands.saveCharacter();
  if (result.status === "ok") {
    return { id: result.data.id, error: null };
  }
  return { id: null, error: result.error.message };
}

/**
 * Navigate to the character sheet for a given ID.
 */
export function navigateToSheet(characterId: number): void {
  goto(`/character/${characterId}`);
}
