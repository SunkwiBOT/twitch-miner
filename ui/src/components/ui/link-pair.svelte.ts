import { getContext, setContext } from "svelte";

const LINK_PAIR = Symbol("link-pair");

export class LinkPairState {
  hovered = $state(false);
}

export const provideLinkPair = (): LinkPairState => setContext(LINK_PAIR, new LinkPairState());

export const linkPair = (): LinkPairState | undefined => getContext(LINK_PAIR);
