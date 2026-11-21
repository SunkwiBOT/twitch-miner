<script lang="ts">
  import type { DropsByCategoryType } from "../../lib/bindings/DropsByCategoryType";
  import { gameArt, gameUrl } from "../../lib/catalogue/games.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import GameName from "../ui/GameName.svelte";
  import { ThumbSize } from "../ui/picture.constants";

  type StatsTopCategoryPropsType = { topCategory: DropsByCategoryType | undefined };

  let { topCategory }: StatsTopCategoryPropsType = $props();
</script>

{#if !topCategory}
  <span class="text-muted-foreground">{t.stats_tile_top_empty()}</span>
{:else if topCategory.category}
  <GameName
    name={topCategory.category}
    url={gameUrl(topCategory.category, undefined)}
    art={gameArt(topCategory.category, undefined)}
    size={ThumbSize.XS}
    inSentence={false}
  />
{:else}
  <span class="text-muted-foreground">{t.stats_category_unknown()}</span>
{/if}
