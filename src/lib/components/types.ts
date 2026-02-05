/**
 * Central type definitions for shadcn-svelte components
 * Feature: 012-shadcn-migration
 *
 * This file re-exports all component types for convenient importing.
 * Usage: import type { ButtonVariant, BadgeVariant, ... } from '$lib/components/types';
 */

// Button types
export type { ButtonVariant, ButtonSize } from './shadcn/button/types.js';

// Badge types
export type { BadgeVariant } from './shadcn/badge/types.js';

// Alert types
export type {
  AlertVariant,
  AlertProps,
  AlertTitleProps,
  AlertDescriptionProps
} from './shadcn/alert/types.js';

// Card types
export type {
  CardProps,
  CardHeaderProps,
  CardTitleProps,
  CardDescriptionProps,
  CardContentProps,
  CardFooterProps
} from './shadcn/card/types.js';

// Sheet types
export type { SheetSide, SheetProps } from './shadcn/sheet/types.js';

// Table types
export type {
  TableProps,
  TableHeaderProps,
  TableBodyProps,
  TableRowProps,
  TableHeadProps,
  TableCellProps
} from './shadcn/table/types.js';

// Input types (HTML native types can be imported as needed)
export type AutoFillHint = HTMLInputElement['autocomplete'];

// Textarea types (native HTML)
export type TextareaRows = number;

// Checkbox types (native HTML)
export type CheckboxState = boolean;

// Dialog types
export interface DialogProps {
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  class?: string;
  'aria-labelledby'?: string;
  'aria-describedby'?: string;
}

// Toast types
export interface ToastOptions {
  id?: string;
  title?: string;
  description?: string;
  variant?: 'default' | 'destructive' | 'success';
  duration?: number;
}

export interface Toast extends Required<Omit<ToastOptions, 'duration'>> {
  duration: number;
}

// Theme types
export type ThemeMode = 'light' | 'dark';
