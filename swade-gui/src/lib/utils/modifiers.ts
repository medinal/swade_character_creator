/**
 * Utility functions for working with character modifiers.
 */

import type { CharacterView, Modifier } from "$lib/bindings";

/**
 * A modifier with its source for display purposes.
 */
export type ModifierWithSource = {
  description: string;
  value: number | null;
  source: string;
};

/**
 * Get all modifiers affecting a specific target (attribute or skill).
 * Collects modifiers from character, ancestry, edges, and hindrances.
 *
 * @param character - The character view to search
 * @param targetType - The type of target (e.g., "attribute", "skill")
 * @param targetIdentifier - The identifier of the target (e.g., "agility", "fighting")
 * @param valueType - The type of modifier value (default: "die_increment")
 */
export function getModifiersFor(
  character: CharacterView | null,
  targetType: string,
  targetIdentifier: string,
  valueType: string = "die_increment"
): ModifierWithSource[] {
  if (!character) return [];

  const results: ModifierWithSource[] = [];

  const matchesTarget = (m: Modifier) =>
    m.value_type === valueType &&
    m.target_type === targetType &&
    m.target_identifier === targetIdentifier;

  // Direct character modifiers
  for (const m of character.modifiers) {
    if (matchesTarget(m)) {
      results.push({ description: m.description, value: m.value, source: "Character" });
    }
  }

  // Ancestry modifiers
  if (character.ancestry) {
    for (const m of character.ancestry.modifiers) {
      if (matchesTarget(m)) {
        results.push({
          description: m.description,
          value: m.value,
          source: character.ancestry.name,
        });
      }
    }
  }

  // Edge modifiers
  for (const edgeValue of character.edges) {
    for (const m of edgeValue.edge.modifiers) {
      if (matchesTarget(m)) {
        results.push({
          description: m.description,
          value: m.value,
          source: edgeValue.edge.name,
        });
      }
    }
  }

  // Hindrance modifiers
  for (const hindranceValue of character.hindrances) {
    for (const m of hindranceValue.hindrance.modifiers) {
      if (matchesTarget(m)) {
        results.push({
          description: m.description,
          value: m.value,
          source: hindranceValue.hindrance.name,
        });
      }
    }
  }

  return results;
}
