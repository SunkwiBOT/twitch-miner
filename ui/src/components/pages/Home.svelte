<script lang="ts">
  import { PauseScope } from "../../lib/accounts/account-state.constants";
  import { MODE_LABELS } from "../../lib/accounts/accounts.utils";
  import { maxWatchedChannelsParams } from "../../lib/accounts/slots.utils";
  import { AccountStatus } from "../../lib/bindings/AccountStatus";
  import { EventKind } from "../../lib/bindings/EventKind";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { recentEvents } from "../../lib/events/events.utils";
  import { number } from "../../lib/format.utils";
  import { TWITCH_ACTIVATE_ADDRESS } from "../../lib/partners.constants";
  import { code, strong } from "../../lib/text.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { PAGE_ICONS, Page } from "../../lib/state/state.constants";
  import { app } from "../../lib/state/app.svelte";
  import { navigate } from "../../lib/state/router";
  import AccountActivity from "../AccountActivity.svelte";
  import ActivityFeed from "../ActivityFeed.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { Card } from "$lib/components/ui/card";
  import AccountHeading from "../ui/AccountHeading.svelte";
  import EmptyState from "../ui/EmptyState.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import Notice from "../ui/Notice.svelte";
  import PageHeader from "../ui/PageHeader.svelte";
  import Panel from "../ui/Panel.svelte";
  import PauseControl from "../ui/PauseControl.svelte";
  import QuietWindowBadge from "../ui/QuietWindowBadge.svelte";
  import RichText from "../ui/RichText.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";
  import { MenuAlign } from "../ui/menu.constants";
  import { NoticeTone } from "../ui/ui.constants";
  import { RECENT_MAX } from "./home.constants";
  import UpdateNotificationsBanner from "../updates/UpdateNotificationsBanner.svelte";

  type HomePropsType = { snapshot: SnapshotType };

  let { snapshot }: HomePropsType = $props();

  const events = $derived(
    recentEvents(
      app.liveEvents,
      snapshot.recent_events,
      (e) => e.kind !== EventKind.DROP_PROGRESS,
      RECENT_MAX,
    ),
  );

  const stats = (a: (typeof snapshot.accounts)[number]): string[] =>
    [
      t.accounts_stats_drops({ dropCount: strong(number(a.stats.drops_claimed)) }),
      t.accounts_stats_points({ pointCount: strong(number(a.stats.points_earned)) }),
      t.accounts_stats_minutes({ watchedMinutes: strong(number(a.stats.minutes_watched)) }),
      a.stats.bonuses_claimed > 0
        ? t.accounts_stats_bonuses({ bonusCount: strong(number(a.stats.bonuses_claimed)) })
        : "",
      a.stats.predictions_won + a.stats.predictions_lost > 0
        ? t.accounts_stats_bets({
            won: strong(number(a.stats.predictions_won)),
            lost: strong(number(a.stats.predictions_lost)),
          })
        : "",
    ].filter(Boolean);
</script>

<PageBody>
  {#if snapshot.accounts.length === 0}
    <EmptyState
      title={t.dashboard_no_accounts_title()}
      text={t.dashboard_no_accounts_text({ activateAddress: code(TWITCH_ACTIVATE_ADDRESS) })}
      icon={IconName.USERS}
    >
      {#snippet action()}
        <TextButton
          label={t.accounts_add()}
          icon={IconName.PLUS}
          variant={ButtonVariant.PRIMARY}
          size={ButtonSize.MD}
          type={ButtonType.BUTTON}
          disabled={false}
          onclick={() => (app.loginDialog = true)}
        />
      {/snippet}
    </EmptyState>
  {:else}
    <PageHeader intro={t.home_rules(maxWatchedChannelsParams())}>
      {#snippet actions()}
        <QuietWindowBadge />
        <PauseControl accounts={snapshot.accounts} scope={PauseScope.ALL} align={MenuAlign.RIGHT} />
      {/snippet}
    </PageHeader>
    <UpdateNotificationsBanner />
    <!-- ! Columns, not a grid: each card keeps its own height instead of the tallest of its row.
         The gap is the wrapper's padding, not a margin on the card: WebKitGTK carries a card's
         margin and bottom edge into the next column despite break-inside: avoid. -->
    <div class="columns-[460px] gap-5">
      {#each snapshot.accounts as a (a.id)}
        <div class="break-inside-avoid pb-5 last:pb-0">
          <Card class="min-w-0 gap-0 py-0">
            <header class="border-b px-5 py-4">
              <AccountHeading account={a} subtitle={MODE_LABELS[a.settings.mode]()}>
                {#snippet actions()}
                  {#if a.status === AccountStatus.LOGIN_REQUIRED}
                    <TextButton
                      label={t.accounts_login_again()}
                      icon={IconName.LOG_IN}
                      variant={ButtonVariant.PRIMARY}
                      size={ButtonSize.MD}
                      type={ButtonType.BUTTON}
                      disabled={false}
                      onclick={() => (app.loginDialog = true)}
                    />
                  {:else}
                    <PauseControl
                      accounts={[a]}
                      scope={PauseScope.ACCOUNT}
                      align={MenuAlign.RIGHT}
                    />
                  {/if}
                  <TextButton
                    label={t.home_open()}
                    icon={PAGE_ICONS[Page.ACCOUNT]}
                    variant={ButtonVariant.OUTLINE}
                    size={ButtonSize.MD}
                    type={ButtonType.BUTTON}
                    disabled={false}
                    onclick={() => navigate(Page.ACCOUNT, a.id)}
                  />
                {/snippet}
              </AccountHeading>
            </header>
            {#if a.drops_held}
              <div class="px-5 pt-4">
                <Notice
                  tone={NoticeTone.DANGER}
                  text={t.account_drops_hold({ count: strong(a.drops_held) })}
                />
              </div>
            {/if}
            <div class="flex flex-col"><AccountActivity account={a} /></div>
            <footer
              class="flex flex-wrap gap-x-4 gap-y-1 border-t bg-muted/60 px-5 py-3 text-hint text-muted-foreground tabular-nums"
            >
              {#each stats(a) as line (line)}<RichText text={line} />{/each}
            </footer>
          </Card>
        </div>
      {/each}
    </div>

    <Panel title={t.dashboard_recent()}>
      {#snippet actions()}
        <TextButton
          label={t.nav_history()}
          icon={PAGE_ICONS[Page.HISTORY]}
          variant={ButtonVariant.OUTLINE}
          size={ButtonSize.SM}
          type={ButtonType.BUTTON}
          disabled={false}
          onclick={() => navigate(Page.HISTORY)}
        />
      {/snippet}
      <ActivityFeed {events} showAccount={true} wide={false} empty={t.history_empty()} />
    </Panel>
  {/if}
</PageBody>
