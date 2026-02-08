// Re-export Sonner toast API with backward compatibility wrapper
// Provides success, error, info, warning, loading, promise, and custom toast methods
import { toast } from 'svelte-sonner';
import type { ExternalToast } from 'svelte-sonner';

// Wrapper to support both {title: string} and string formats for backward compatibility
type ToastOptions = ExternalToast & { title?: string };

function normalizeToast(
  messageOrOptions: string | ToastOptions,
  options?: ExternalToast
): [string, ExternalToast | undefined] {
  if (typeof messageOrOptions === 'string') {
    return [messageOrOptions, options];
  }
  const { title, ...rest } = messageOrOptions;
  return [title || '', rest];
}

export const toaster = {
  success: (messageOrOptions: string | ToastOptions, options?: ExternalToast) => {
    const [message, opts] = normalizeToast(messageOrOptions, options);
    return toast.success(message, opts);
  },
  error: (messageOrOptions: string | ToastOptions, options?: ExternalToast) => {
    const [message, opts] = normalizeToast(messageOrOptions, options);
    return toast.error(message, opts);
  },
  info: (messageOrOptions: string | ToastOptions, options?: ExternalToast) => {
    const [message, opts] = normalizeToast(messageOrOptions, options);
    return toast.info(message, opts);
  },
  warning: (messageOrOptions: string | ToastOptions, options?: ExternalToast) => {
    const [message, opts] = normalizeToast(messageOrOptions, options);
    return toast.warning(message, opts);
  },
  loading: (messageOrOptions: string | ToastOptions, options?: ExternalToast) => {
    const [message, opts] = normalizeToast(messageOrOptions, options);
    return toast.loading(message, opts);
  },
  promise: toast.promise,
  custom: toast.custom,
  message: toast.message,
  dismiss: toast.dismiss
};
