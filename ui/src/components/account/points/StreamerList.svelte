<script lang="ts">
  import { SvelteMap } from "svelte/reactivity";
  import { Checkbox } from "$lib/components/ui/checkbox";
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import type { ChannelType } from "../../../lib/bindings/ChannelType";
  import type { StreamerSettingsType } from "../../../lib/bindings/StreamerSettingsType";
  import { ensureBanners, knownBanners } from "../../../lib/channels/channels.svelte";
  import { number } from "../../../lib/format.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { strong } from "../../../lib/text.utils";
  import IconButton from "../../ui/IconButton.svelte";
  import RichText from "../../ui/RichText.svelte";
  import SortableList from "../../ui/SortableList.svelte";
  import TextButton from "../../ui/TextButton.svelte";
  import TiledArt from "../../ui/TiledArt.svelte";
  import Toggle from "../../ui/Toggle.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../../ui/button.constants";
  import { IconName } from "../../../lib/icons.constants";
  import { ThumbShape } from "../../ui/picture.constants";
  import StreamerRow from "./StreamerRow.svelte";
  import { STREAMER_BOX_CLASS } from "../account.constants";

  type StreamerListPropsType = {
    account: AccountSnapshotType;
    channelOf: (login: string) => ChannelType | undefined;
    selected: string[];
    onselect: (logins: string[]) => void;
    oneditselected: () => void;
    onreorder: (list: StreamerSettingsType[], moved: StreamerSettingsType) => void;
    ontoggle: (login: string, enabled: boolean) => void;
    onedit: (login: string) => void;
    onremove: (login: string) => void;
  };

  let {
    account,
    channelOf,
    selected,
    onselect,
    oneditselected,
    onreorder,
    ontoggle,
    onedit,
    onremove,
  }: StreamerListPropsType = $props();

  const streamers = $derived(account.settings.points.streamers);
  const chosen = $derived(new Set(selected));
  const all = $derived(streamers.length > 0 && streamers.every((s) => chosen.has(s.login)));
  const fits = new SvelteMap<string, boolean>();
  const beside = $derived([...fits.values()].every(Boolean));

  const selectLabel = $derived.by(() => {
    if (selected.length === 0) {
      return t.common_select_all();
    }

    const count = strong(number(selected.length));

    return all
      ? t.common_select_none_count({ itemCount: count })
      : t.common_select_all_count({ itemCount: count });
  });

  $effect(() => {
    ensureBanners(streamers.map((s) => s.login));
  });

  const tick = (login: string, checked: boolean): void =>
    onselect(checked ? [...selected, login] : selected.filter((item) => item !== login));

  const measure = (login: string, fit: boolean | null): void => {
    if (fit === null) {
      fits.delete(login);
    } else {
      fits.set(login, fit);
    }
  };
</script>

<SortableList
  items={streamers}
  key={(streamer) => streamer.login}
  empty={t.account_points_empty()}
  upLabel={t.common_up()}
  downLabel={t.common_down()}
  {onreorder}
>
  {#snippet header()}
    <label class="flex cursor-pointer items-center gap-3 text-hint font-medium">
      <Checkbox
        class={STREAMER_BOX_CLASS}
        checked={all}
        indeterminate={selected.length > 0 && !all}
        onCheckedChange={() => onselect(all ? [] : streamers.map((s) => s.login))}
      />
      <RichText text={selectLabel} />
    </label>
    <span class="sr-only" role="status">
      <RichText
        text={t.common_selected_count({
          selectedCount: strong(selected.length),
          total: strong(streamers.length),
        })}
      />
    </span>
    {#if selected.length > 0}
      <TextButton
        label={t.points_edit_selected({ selectedCount: strong(selected.length) })}
        icon={IconName.EDIT}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.SM}
        type={ButtonType.BUTTON}
        disabled={false}
        class="ml-auto"
        onclick={oneditselected}
      />
    {/if}
  {/snippet}
  {#snippet lead(s)}
    <Checkbox
      class={STREAMER_BOX_CLASS}
      checked={chosen.has(s.login)}
      aria-label={channelOf(s.login)?.display_name ?? s.login}
      onCheckedChange={(checked) => tick(s.login, checked === true)}
    />
  {/snippet}
  {#snippet background(s)}
    {@const banner = knownBanners[s.login]}
    {@const picture = channelOf(s.login)?.avatar_url}
    {#if banner}
      <span
        class={[
          "absolute inset-0 -z-10 bg-cover bg-center opacity-20 [mask-image:linear-gradient(to_right,black,transparent_75%)] dark:opacity-30",
          !s.enabled && "grayscale",
        ]}
        style:background-image="url({banner})"
        aria-hidden="true"
      ></span>
    {:else if picture}
      <TiledArt url={picture} shape={ThumbShape.SQUARE} grey={!s.enabled} />
    {/if}
  {/snippet}
  {#snippet row(s)}
    <StreamerRow
      streamer={s}
      channel={channelOf(s.login)}
      view={account.streamers.find((view) => view.login === s.login)}
      {beside}
      onmeasure={measure}
    />
  {/snippet}
  {#snippet actions(s)}
    <Toggle
      checked={s.enabled}
      disabled={false}
      label={t.common_enabled()}
      onchange={(enabled) => ontoggle(s.login, enabled)}
    />
    <IconButton
      icon={IconName.EDIT}
      label={t.common_edit()}
      busy={false}
      disabled={false}
      variant={ButtonVariant.GHOST}
      size={ButtonSize.SM}
      onclick={() => onedit(s.login)}
    />
    <IconButton
      icon={IconName.TRASH}
      label={t.confirm_remove()}
      busy={false}
      disabled={false}
      variant={ButtonVariant.DANGER_GHOST}
      size={ButtonSize.SM}
      onclick={() => onremove(s.login)}
    />
  {/snippet}
</SortableList>
