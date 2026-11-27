import Flame from '@lucide/svelte/icons/flame';
import Fuel from '@lucide/svelte/icons/fuel';
import Heart from '@lucide/svelte/icons/heart';
import PackageOpen from '@lucide/svelte/icons/package-open';
import TagIcon from '@lucide/svelte/icons/tag';
import TrainFront from '@lucide/svelte/icons/train-front';
import Zap from '@lucide/svelte/icons/zap';

// Lucide exports Svelte components; use a representative type alias.
export type IconComponent = typeof Flame;

export const iconMap: Record<string, IconComponent> = {
  steam: Flame,
  diesel: Fuel,
  electric: Zap,
  passenger: TrainFront,
  freight: PackageOpen,
  heart: Heart,
  default: TagIcon
};

export function resolveTagIcon(tag: string): IconComponent {
  const key = tag.toLowerCase();
  return iconMap[key] ?? iconMap.default;
}
