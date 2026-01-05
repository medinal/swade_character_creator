/**
 * Utility functions for hindrance point calculations.
 *
 * Hindrance points can be spent on:
 * - 2 points = 1 Edge
 * - 2 points = 1 Attribute step
 * - 1 point = 1 Skill point
 * - 1 point = Starting funds x2 (wealth)
 */

import type { CharacterView } from "$lib/bindings";

/**
 * Calculate the total hindrance points that have been allocated across all categories.
 * Note: hindrance_points_to_attributes stores attribute points (2 HP each), so multiply by 2.
 */
export function calculateHindrancePointsAllocated(character: CharacterView | null): number {
  if (!character) return 0;

  return (
    (character.hindrance_points_to_edges ?? 0) +
    (character.hindrance_points_to_attributes ?? 0) * 2 +
    (character.hindrance_points_to_skills ?? 0) +
    (character.hindrance_points_to_wealth ?? 0)
  );
}

/**
 * Calculate the hindrance points available for allocation.
 */
export function calculateHindrancePointsAvailable(character: CharacterView | null): number {
  if (!character) return 0;

  const earned = character.hindrance_points_earned ?? 0;
  const allocated = calculateHindrancePointsAllocated(character);
  return earned - allocated;
}
