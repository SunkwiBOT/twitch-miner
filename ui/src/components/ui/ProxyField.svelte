<script lang="ts">
  import { testProxy } from "../../lib/api/client";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { reportFailure } from "../../lib/state/session";
  import { toast } from "../../lib/state/toasts";
  import { ToastKind } from "../../lib/state/state.constants";
  import { strong } from "../../lib/text.utils";
  import ActionButton from "./ActionButton.svelte";
  import FormField from "./FormField.svelte";
  import SecretInput from "./SecretInput.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "./button.constants";
  import { IconName } from "../../lib/icons.constants";

  type ProxyFieldPropsType = {
    id: string;
    label: string;
    hint: string;
    proxy: string;
    onProxyChange: (proxy: string) => void;
    proxyPlaceholder: string;
  };

  let { id, label, hint, proxy, onProxyChange, proxyPlaceholder }: ProxyFieldPropsType = $props();

  let testing = $state(false);

  const checkProxy = async (): Promise<void> => {
    testing = true;

    try {
      toast(ToastKind.SUCCESS, t.settings_proxy_ok_title(), {
        description: t.settings_proxy_ok({ ms: strong(await testProxy(proxy)) }),
      });
    } catch (e: unknown) {
      reportFailure(e);
    } finally {
      testing = false;
    }
  };
</script>

<FormField {id} {label} {hint}>
  <SecretInput
    {id}
    class="font-mono"
    autocomplete="off"
    value={proxy}
    oninput={(event) => onProxyChange(event.currentTarget.value.trim())}
    placeholder={proxyPlaceholder}
  />
</FormField>
<div class="flex flex-wrap justify-end gap-2">
  <ActionButton
    icon={IconName.GLOBE}
    label={t.settings_proxy_test()}
    busyLabel={t.common_testing()}
    busy={testing}
    disabled={!proxy}
    variant={ButtonVariant.DEFAULT}
    size={ButtonSize.MD}
    type={ButtonType.BUTTON}
    onclick={() => void checkProxy()}
  />
</div>
