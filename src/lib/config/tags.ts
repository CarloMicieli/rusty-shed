import * as m from '$lib/paraglide/messages.js';
import { resolveTagIcon } from './icons';

export type FixedTagKey = 'steam' | 'diesel' | 'electric' | 'passenger' | 'freight';

export type TagMeta = {
  key: string;
  label: () => string;
  variant: string;
  gradient: string;
  iconKey: string;
};

export const FIXED_TAG_META: Record<FixedTagKey, TagMeta> = {
  steam: {
    key: 'steam',
    label: () => m.collection_tag_steam(),
    variant: 'default',
    gradient: 'bg-gradient-to-br from-primary-500/20 via-muted to-background',
    iconKey: 'steam'
  },
  diesel: {
    key: 'diesel',
    label: () => m.collection_tag_diesel(),
    variant: 'default',
    gradient: 'bg-gradient-to-br from-secondary-500/20 via-muted to-background',
    iconKey: 'diesel'
  },
  electric: {
    key: 'electric',
    label: () => m.collection_tag_electric(),
    variant: 'default',
    gradient: 'bg-gradient-to-br from-tertiary-500/20 via-muted to-background',
    iconKey: 'electric'
  },
  passenger: {
    key: 'passenger',
    label: () => m.collection_tag_passenger(),
    variant: 'secondary',
    gradient: 'bg-gradient-to-br from-accent-500/15 via-muted to-background',
    iconKey: 'passenger'
  },
  freight: {
    key: 'freight',
    label: () => m.collection_tag_freight(),
    variant: 'secondary',
    gradient: 'bg-gradient-to-br from-warning-500/15 via-muted to-background',
    iconKey: 'freight'
  }
};

export const DEFAULT_TAG_META: TagMeta = {
  key: 'default',
  label: () => 'Tag',
  variant: 'secondary',
  gradient: 'bg-gradient-to-br from-muted to-background',
  iconKey: 'default'
};

export function resolveTagMeta(tag: string): TagMeta {
  const key = tag.toLowerCase() as FixedTagKey;
  return FIXED_TAG_META[key] ?? { ...DEFAULT_TAG_META, key: tag };
}

export function sortAvailableTags(tags: string[]): string[] {
  const fixedOrder: FixedTagKey[] = ['steam', 'diesel', 'electric', 'passenger', 'freight'];
  const fixed = fixedOrder.filter((key) => tags.map((t) => t.toLowerCase()).includes(key));
  const dynamic = tags
    .filter((t) => !fixed.includes(t.toLowerCase() as FixedTagKey))
    .sort((a, b) => a.localeCompare(b));
  return [...fixed, ...dynamic];
}

export function tagIcon(tag: string) {
  const meta = resolveTagMeta(tag);
  return resolveTagIcon(meta.iconKey);
}
