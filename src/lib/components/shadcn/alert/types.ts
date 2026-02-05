export type AlertVariant = 'default' | 'destructive' | 'success' | 'warning';

export interface AlertProps {
	variant?: AlertVariant;
	class?: string;
}

export interface AlertTitleProps {
	class?: string;
}

export interface AlertDescriptionProps {
	class?: string;
}
