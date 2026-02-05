export type SheetSide = 'left' | 'right' | 'top' | 'bottom';

export interface SheetProps {
  open: boolean;
  onOpenChange?: (open: boolean) => void;
  side?: SheetSide;
  class?: string;
}
