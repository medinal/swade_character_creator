/**
 * Utility functions for working with character die values.
 *
 * Note: Effective die values (with modifiers applied) are now pre-computed
 * in Rust and included in CharacterView. Use the effective_die, base_die,
 * and max_die fields directly instead of computing them client-side.
 */

import type { Die } from "./bindings";

const VALID_SIZES = [4, 6, 8, 10, 12] as const;

/**
 * Create a die with the given size and modifier.
 */
export function createDie(size: number, modifier: number = 0): Die {
  return { size, modifier };
}

/**
 * Increment a die to the next value in the progression.
 * d4 → d6 → d8 → d10 → d12 → d12+1 → d12+2 → ...
 */
export function incrementDie(die: Die): Die {
  if (die.size === 12) {
    return { size: 12, modifier: die.modifier + 1 };
  }
  const idx = VALID_SIZES.indexOf(die.size as (typeof VALID_SIZES)[number]);
  if (idx === -1 || idx >= VALID_SIZES.length - 1) {
    return { size: 12, modifier: die.modifier };
  }
  return { size: VALID_SIZES[idx + 1], modifier: 0 };
}

/**
 * Decrement a die to the previous value in the progression.
 * Returns null if already at d4.
 */
export function decrementDie(die: Die): Die | null {
  if (die.modifier > 0) {
    return { size: 12, modifier: die.modifier - 1 };
  }
  if (die.size === 4) {
    return null;
  }
  const idx = VALID_SIZES.indexOf(die.size as (typeof VALID_SIZES)[number]);
  if (idx <= 0) {
    return null;
  }
  return { size: VALID_SIZES[idx - 1], modifier: 0 };
}

/**
 * Format a die for display.
 * @param die The die to format, or null/undefined for untrained
 * @param untrained If true and die is null, returns "-2" instead of "—"
 */
export function formatDie(
  die: Die | null | undefined,
  untrained: boolean = false
): string {
  if (!die) {
    return untrained ? "-2" : "—";
  }
  if (die.modifier > 0) {
    return `d${die.size}+${die.modifier}`;
  } else if (die.modifier < 0) {
    return `d${die.size}${die.modifier}`;
  }
  return `d${die.size}`;
}

/**
 * Compare two dice. Returns:
 * - negative if a < b
 * - 0 if a == b
 * - positive if a > b
 */
export function compareDice(a: Die, b: Die): number {
  if (a.size !== b.size) {
    return a.size - b.size;
  }
  return a.modifier - b.modifier;
}

/**
 * Check if two dice are equal.
 */
export function diceEqual(a: Die, b: Die): boolean {
  return a.size === b.size && a.modifier === b.modifier;
}
