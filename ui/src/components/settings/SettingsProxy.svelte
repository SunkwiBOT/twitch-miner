<script lang="ts">
  import { testProxy } from "../../lib/api/client";
  import type { GlobalSettingsType } from "../../lib/bindings/GlobalSettingsType";
  import { describeError } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ToastKind } from "../../lib/state/state.constants";
  import { toast } from "../../lib/state/toasts";
  import { strong } from "../../lib/text.utils";
  import ActionButton from "../ui/ActionButton.svelte";
  import FieldActions from "../ui/FieldActions.svelte";
  import FormField from "../ui/FormField.svelte";
  import Panel from "../ui/Panel.svelte";
  import SecretInput from "../ui/SecretInput.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";

  type SettingsProxyPropsType = {
    draft: GlobalSettingsType;
  };

  let { draft = $bindable() }: SettingsProxyPropsType = $props();

  let testingProxy = $state(false);

  const checkProxy = async (): Promise<void> => {
    testingProxy = true;

    try {
      toast(ToastKind.SUCCESS, t.settings_proxy_ok_title(), {
        description: t.settings_proxy_ok({
          ms: strong(await testProxy(draft.default_proxy.trim())),
        }),
      });
    } catch (e: any) {
      toast(ToastKind.ERROR, t.action_failed(), { description: describeError(e) });
    } finally {
      testingProxy = false;
    }
  };
</script>

{#snippet input()}
  <SecretInput
    id="default-proxy"
    class="font-mono"
    autocomplete="off"
    bind:value={draft.default_proxy}
    placeholder="socks5://user:pass@host:1080"
  />
{/snippet}

{#snippet test()}
  <ActionButton
    icon={IconName.GLOBE}
    label={t.settings_proxy_test()}
    ariaLabel={`${t.settings_proxy_test()} ${t.settings_proxy()}`}
    busyLabel={t.common_testing()}
    busy={testingProxy}
    disabled={!draft.default_proxy.trim()}
    variant={ButtonVariant.DEFAULT}
    size={ButtonSize.MD}
    type={ButtonType.BUTTON}
    onclick={checkProxy}
  />
{/snippet}

<Panel title={t.settings_proxy()}>
  <FormField id="default-proxy" label={t.settings_default_proxy()} hint={t.settings_proxy_hint()}>
    <FieldActions field={input} actions={test} />
  </FormField>
</Panel>
