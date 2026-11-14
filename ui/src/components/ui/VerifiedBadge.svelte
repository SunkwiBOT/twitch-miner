<script lang="ts">
  import { t } from "../../lib/i18n/i18n.svelte";
  import Icon from "./Icon.svelte";
  import { IconName } from "../../lib/icons.constants";
</script>

<span
  class="inline-flex shrink-0 text-brand-twitch"
  role="img"
  aria-label={t.search_verified()}
  title={t.search_verified()}><Icon name={IconName.VERIFIED} size={16} /></span
>
