<script lang="ts">
  import type { LockWarningToastPropsType } from "../../lib/state/toasts";
  import { mmss } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { strong } from "../../lib/text.utils";
  import { IconName } from "../../lib/icons.constants";
  import Icon from "../ui/Icon.svelte";
  import IconButton from "../ui/IconButton.svelte";
  import RichText from "../ui/RichText.svelte";
  import { ButtonSize, ButtonVariant } from "../ui/button.constants";

  let { deadline, onclose }: LockWarningToastPropsType = $props();

  let now = $state(performance.now());

  $effect(() => {
    const timer = setInterval(() => {
      now = performance.now();
    }, 1000);

    return () => clearInterval(timer);
  });

  const remaining = $derived(Math.max(0, deadline - now));
</script>

<div
  role="status"
  class="relative flex w-toast max-w-[calc(100vw-2rem)] items-start gap-3 rounded-lg border bg-popover p-3 pr-10 text-sm text-popover-foreground shadow-lg"
>
  <Icon name={IconName.CLOCK} size={16} class="mt-0.5 shrink-0 text-muted-foreground" />
  <p class="m-0 flex-1 leading-snug">
    <span aria-hidden="true">
      <RichText text={t.lock_warning_message({ time: strong(mmss(remaining)) })} />
    </span>
    <span class="sr-only">{t.lock_warning_message_a11y()}</span>
  </p>
  <div class="absolute top-1.5 right-1.5">
    <IconButton
      icon={IconName.X}
      label={t.common_close()}
      busy={false}
      disabled={false}
      variant={ButtonVariant.GHOST}
      size={ButtonSize.SM}
      onclick={onclose}
    />
  </div>
</div>
