import { Flame, Fuel, Heart, PackageOpen, Tag as TagIcon, TrainFront, Zap } from 'lucide-svelte';

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
