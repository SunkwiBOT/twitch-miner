<script lang="ts">
  import { untrack } from "svelte";

  import { t } from "../../lib/i18n/i18n.svelte";
  import { applyToAccounts, bulk, closeBulk } from "../../lib/state/bulk.svelte";
  import { wouldChange } from "../../lib/state/bulk.utils";
  import { account, app } from "../../lib/state/app.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { personMarker, strong } from "../../lib/text.utils";
  import ActionButton from "../ui/ActionButton.svelte";
  import CheckList from "../ui/CheckList.svelte";
  import Dialog from "../ui/Dialog.svelte";
  import RichText from "../ui/RichText.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";

  let sourceIds = $state<string[]>([]);
  let count = $state(0);
  let selected = $state<string[]>([]);
  let applying = $state(false);
  let applicableAtOpen = $state<Map<string, boolean>>(new Map());

  const source = $derived(sourceIds.length === 1 ? account(sourceIds[0]) : null);
  const others = $derived((app.snapshot?.accounts ?? []).filter((a) => !sourceIds.includes(a.id)));
  const applicable = $derived(new Map(others.map((a) => [a.id, wouldChange(a, bulk.changes)])));

  const items = $derived(
    [...others]
      .sort((a, b) => Number(applicableAtOpen.get(b.id)) - Number(applicableAtOpen.get(a.id)))
      .map((a) => ({
        value: a.id,
        label: a.display_name,
        image: a.avatar_url,
        hint: applicableAtOpen.get(a.id) ? "" : t.bulk_nothing_to_change(),
        channel: null,
        disabled: !applicableAtOpen.get(a.id),
      })),
  );

  const applicableCount = $derived(items.filter((item) => !item.disabled).length);

  // ! Kept until the dialog is gone: its closing animation must not show the emptied offer.
  $effect(() => {
    if (bulk.open) {
      untrack(() => {
        sourceIds = bulk.sourceIds;
        count = bulk.changes.length;
        applicableAtOpen = new Map(others.map((a) => [a.id, applicable.get(a.id) ?? false]));
        selected = others.filter((a) => applicableAtOpen.get(a.id)).map((a) => a.id);
      });
    }
  });

  const apply = async (): Promise<void> => {
    applying = true;
    await applyToAccounts(selected);
    applying = false;
    closeBulk();
  };
</script>

<Dialog
  open={bulk.open}
  title={t.bulk_title()}
  wide={false}
  closable={!applying}
  closeLabel={t.common_close()}
  onclose={closeBulk}
>
  <div class="flex flex-col gap-3">
    <p class="text-hint text-muted-foreground">
      {#if source}
        <RichText
          text={t.bulk_hint({
            count: strong(count),
            account: personMarker(0),
          })}
          people={[{ name: source.display_name, avatar: source.avatar_url, channel: null }]}
        />
      {:else}
        <RichText text={t.bulk_hint_chosen({ count: strong(count) })} />
      {/if}
    </p>
    <CheckList
      {items}
      {selected}
      label={t.bulk_accounts()}
      allLabel={t.common_select_all()}
      noneLabel={t.common_select_none()}
      countLabel={t.common_selected_count({ count: selected.length, total: applicableCount })}
      empty={t.bulk_no_accounts()}
      onchange={(value) => (selected = value)}
    />
  </div>
  {#snippet footer()}
    <TextButton
      label={t.common_cancel()}
      icon={IconName.X}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={applying}
      onclick={closeBulk}
    />
    <ActionButton
      icon={IconName.CHECK}
      label={t.bulk_apply({ count: selected.length })}
      busyLabel={t.bulk_applying()}
      busy={applying}
      disabled={selected.length === 0}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      onclick={apply}
    />
  {/snippet}
</Dialog>
