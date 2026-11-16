<script lang="ts">
  import type { RewardType } from "../../lib/catalogue/campaign.utils";
  import Thumb from "./Thumb.svelte";
  import { IconName } from "../../lib/icons.constants";
  import { ThumbShape, type ThumbSize } from "./picture.constants";

  type RewardThumbPropsType = { item: RewardType; size: ThumbSize };

  let { item, size }: RewardThumbPropsType = $props();
</script>

<Thumb
  src={item.drop.benefits.at(0)?.image_url}
  round={false}
  contain={true}
  {size}
  shape={ThumbShape.SQUARE}
  fallback={IconName.GIFT}
/>
