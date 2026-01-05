<script lang="ts">
  interface Props {
    error: string | null;
  }

  let { error }: Props = $props();
</script>

{#if error}
  <div class="bg-red-50 border border-red-200 text-red-600 dark:bg-red-500/10 dark:border-red-500/20 dark:text-red-400 px-4 py-3 rounded-md mb-6 text-sm">
    Error: {error}
  </div>
{/if}
