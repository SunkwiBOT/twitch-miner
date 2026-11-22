<script lang="ts">
  import { t } from "../../lib/i18n/i18n.svelte";
  import { BadgeTone } from "../../lib/tones.constants";
  import Badge from "./Badge.svelte";
</script>

<Badge text={t.common_beta()} tone={BadgeTone.NEUTRAL} />
