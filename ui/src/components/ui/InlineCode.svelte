<script lang="ts">
  type InlineCodePropsType = { text: string };

  let { text }: InlineCodePropsType = $props();
</script>

<code class="rounded-sm bg-secondary px-1 py-0.5 font-mono text-xs text-foreground">{text}</code>
