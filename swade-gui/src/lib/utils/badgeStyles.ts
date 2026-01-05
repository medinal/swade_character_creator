/**
 * Badge style constants for consistent styling across the application.
 *
 * These classes follow the pattern:
 * - Background: bg-{color}-100 dark:bg-{color}-500/20
 * - Text: text-{color}-700 dark:text-{color}-400
 */

// Severity badges for hindrances
export const SEVERITY_STYLES = {
  minor: 'bg-amber-100 text-amber-700 dark:bg-amber-500/20 dark:text-amber-400',
  major: 'bg-red-100 text-red-700 dark:bg-red-500/20 dark:text-red-400',
} as const;

// Status badges for requirements and availability
export const STATUS_STYLES = {
  met: 'bg-green-100 text-green-700 dark:bg-green-500/20 dark:text-green-400',
  unmet: 'bg-red-100 text-red-700 dark:bg-red-500/20 dark:text-red-400',
  info: 'bg-blue-100 text-blue-700 dark:bg-blue-500/20 dark:text-blue-400',
  special: 'bg-purple-100 text-purple-700 dark:bg-purple-500/20 dark:text-purple-400',
  neutral: 'bg-zinc-200 text-zinc-600 dark:bg-zinc-700 dark:text-zinc-400',
} as const;

// Base badge styling (add to specific badge styles)
export const BADGE_BASE = 'px-1.5 py-0.5 text-xs rounded';

/**
 * Helper to combine base badge styles with a specific variant.
 */
export function badge(variant: keyof typeof STATUS_STYLES | keyof typeof SEVERITY_STYLES): string {
  if (variant in SEVERITY_STYLES) {
    return `${BADGE_BASE} ${SEVERITY_STYLES[variant as keyof typeof SEVERITY_STYLES]}`;
  }
  return `${BADGE_BASE} ${STATUS_STYLES[variant as keyof typeof STATUS_STYLES]}`;
}
