/**
 * Utility functions for formatting display strings.
 */

/**
 * Format a source string from snake_case to Title Case.
 * Examples:
 *   "hindrance_points" -> "Hindrance Points"
 *   "ancestry" -> "Ancestry"
 *   "free" -> "Free"
 */
export function formatSource(source: string): string {
  return source
    .split("_")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
    .join(" ");
}
