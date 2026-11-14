<script lang="ts">
  import { BadgesList } from "../../../lib/accounts/accounts.constants";
  import { editBadgesList, setIgnored } from "../../../lib/accounts/drops.utils";
  import type { AccountBadgesType } from "../../../lib/badges/badges.utils";
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../../lib/bindings/CampaignType";
  import type { SnapshotType } from "../../../lib/bindings/SnapshotType";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { escapeEmphasis, strong } from "../../../lib/text.utils";
  import EmptyState from "../../ui/EmptyState.svelte";
  import FormField from "../../ui/FormField.svelte";
  import List from "../../ui/List.svelte";
  import ListRow from "../../ui/ListRow.svelte";
  import Panel from "../../ui/Panel.svelte";
  import PanelColumns from "../../ui/PanelColumns.svelte";
  import TagInput from "../../ui/TagInput.svelte";
  import TextButton from "../../ui/TextButton.svelte";
  import Thumb from "../../ui/Thumb.svelte";
  import {
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
    PanelWidth,
    ThumbShape,
    ThumbSize,
  } from "../../ui/ui.constants";

  type BadgesExclusionsTabPropsType = {
    account: AccountSnapshotType;
    snapshot: SnapshotType;
    sections: AccountBadgesType;
    onopen: (campaign: CampaignType) => void;
  };

  let { account, snapshot, sections, onopen }: BadgesExclusionsTabPropsType = $props();

  const games = $derived([...new Set(snapshot.catalog.campaigns.map((c) => c.game.name))].sort());
</script>

<PanelColumns width={PanelWidth.LG}>
  <Panel
    title={t.ignored_title()}
    count={sections.ignored.length}
    description={t.account_badges_ignored_hint()}
  >
    {#if sections.ignored.length === 0}
      <EmptyState text={t.ignored_empty()} />
    {:else}
      <List>
        {#each sections.ignored as item (item.key)}
          <ListRow
            title={item.title}
            details={[escapeEmphasis(item.source)]}
            onopen={item.campaign ? () => item.campaign && onopen(item.campaign) : undefined}
            muted={true}
          >
            {#snippet lead()}
              <Thumb
                src={item.image}
                round={false}
                contain={true}
                size={ThumbSize.MD}
                shape={ThumbShape.SQUARE}
                fallback={IconName.AWARD}
              />
            {/snippet}
            {#snippet trailing()}
              <TextButton
                label={t.ignored_restore()}
                icon={IconName.EYE}
                variant={ButtonVariant.OUTLINE}
                size={ButtonSize.SM}
                type={ButtonType.BUTTON}
                disabled={false}
                onclick={() => item.ignored && setIgnored(account, item.ignored, false)}
              />
            {/snippet}
          </ListRow>
        {/each}
      </List>
    {/if}
  </Panel>
  <Panel title={t.drops_exclusions_title()}>
    <FormField id="badges-exclude-games" label={t.account_drops_exclude_games()}>
      <TagInput
        id="badges-exclude-games"
        items={account.settings.badges.exclude_games}
        suggestions={games}
        placeholder={t.account_drops_priority_placeholder()}
        addLabel={t.common_add()}
        removeLabel={t.common_remove()}
        confirmTitle={t.confirm_remove_excluded_game_title()}
        confirmText={(game) => t.confirm_remove_excluded_game_text({ name: strong(game) })}
        onchange={(list) => editBadgesList(account, BadgesList.EXCLUDE_GAMES, list)}
        remote={true}
      />
    </FormField>
    <FormField
      id="badges-exclude-names"
      label={t.account_drops_exclude_names()}
      hint={t.account_drops_exclude_names_hint()}
    >
      <TagInput
        id="badges-exclude-names"
        items={account.settings.badges.exclude_names}
        suggestions={[]}
        placeholder={t.drops_exclude_names_placeholder()}
        addLabel={t.common_add()}
        removeLabel={t.common_remove()}
        confirmTitle={t.confirm_remove_excluded_word_title()}
        confirmText={(word) => t.confirm_remove_excluded_word_text({ name: strong(word) })}
        onchange={(list) => editBadgesList(account, BadgesList.EXCLUDE_NAMES, list)}
        remote={false}
      />
    </FormField>
  </Panel>
</PanelColumns>
