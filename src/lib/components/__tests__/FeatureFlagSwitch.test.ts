import { describe, it, expect, vi } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import FeatureFlagSwitch from '../FeatureFlagSwitch.svelte';

describe('FeatureFlagSwitch', () => {
  const label = 'Test Flag';

  describe('Rendering', () => {
    it('renders "Yes" when value is YES', () => {
      const { getByRole, getByText } = render(FeatureFlagSwitch, {
        props: { label, value: 'YES' }
      });
      const button = getByRole('button');
      expect(button.getAttribute('aria-label')).toBe('Test Flag: Yes');
      expect(getByText('Yes')).not.toBeNull();
    });

    it('renders "No" when value is NO', () => {
      const { getByRole, getByText } = render(FeatureFlagSwitch, {
        props: { label, value: 'NO' }
      });
      const button = getByRole('button');
      expect(button.getAttribute('aria-label')).toBe('Test Flag: No');
      expect(getByText('No')).not.toBeNull();
    });

    it('renders "N/A" when value is NOT_APPLICABLE', () => {
      const { getByRole, getByText } = render(FeatureFlagSwitch, {
        props: { label, value: 'NOT_APPLICABLE' }
      });
      const button = getByRole('button');
      expect(button.getAttribute('aria-label')).toBe('Test Flag: Not Applicable');
      expect(getByText('N/A')).not.toBeNull();
    });

    it('maps null value to NOT_APPLICABLE', () => {
      const { getByRole, getByText } = render(FeatureFlagSwitch, {
        props: { label, value: null }
      });
      const button = getByRole('button');
      expect(button.getAttribute('aria-label')).toBe('Test Flag: Not Applicable');
      expect(getByText('N/A')).not.toBeNull();
    });

    it('maps undefined value to NOT_APPLICABLE', () => {
      const { getByRole, getByText } = render(FeatureFlagSwitch, {
        props: { label, value: undefined }
      });
      const button = getByRole('button');
      expect(button.getAttribute('aria-label')).toBe('Test Flag: Not Applicable');
      expect(getByText('N/A')).not.toBeNull();
    });
  });

  describe('Interactions (Cycling)', () => {
    it('cycles from YES to NO on click', async () => {
      const { getByRole, getByText } = render(FeatureFlagSwitch, {
        props: { label, value: 'YES' }
      });
      const button = getByRole('button');
      await fireEvent.click(button);
      expect(button.getAttribute('aria-label')).toBe('Test Flag: No');
      expect(getByText('No')).not.toBeNull();
    });

    it('cycles from NO to NOT_APPLICABLE on click', async () => {
      const { getByRole, getByText } = render(FeatureFlagSwitch, {
        props: { label, value: 'NO' }
      });
      const button = getByRole('button');
      await fireEvent.click(button);
      expect(button.getAttribute('aria-label')).toBe('Test Flag: Not Applicable');
      expect(getByText('N/A')).not.toBeNull();
    });

    it('cycles from NOT_APPLICABLE to YES on click', async () => {
      const { getByRole, getByText } = render(FeatureFlagSwitch, {
        props: { label, value: 'NOT_APPLICABLE' }
      });
      const button = getByRole('button');
      await fireEvent.click(button);
      expect(button.getAttribute('aria-label')).toBe('Test Flag: Yes');
      expect(getByText('Yes')).not.toBeNull();
    });
  });

  describe('Keyboard Accessibility', () => {
    it('cycles on Enter key', async () => {
      const { getByRole, getByText } = render(FeatureFlagSwitch, {
        props: { label, value: 'YES' }
      });
      const button = getByRole('button');
      await fireEvent.keyDown(button, { key: 'Enter' });
      expect(getByText('No')).not.toBeNull();
    });

    it('cycles on Space key', async () => {
      const { getByRole, getByText } = render(FeatureFlagSwitch, {
        props: { label, value: 'YES' }
      });
      const button = getByRole('button');
      await fireEvent.keyDown(button, { key: ' ' });
      expect(getByText('No')).not.toBeNull();
    });
  });

  describe('Disabled State', () => {
    it('does not cycle when disabled', async () => {
      const { getByRole, getByText } = render(FeatureFlagSwitch, {
        props: { label, value: 'YES', disabled: true }
      });
      const button = getByRole('button');
      await fireEvent.click(button);
      expect(getByText('Yes')).not.toBeNull();
      await fireEvent.keyDown(button, { key: 'Enter' });
      expect(getByText('Yes')).not.toBeNull();
    });
  });

  describe('Callbacks', () => {
    it('calls onUpdate with the next state', async () => {
      const onUpdate = vi.fn();
      const { getByRole } = render(FeatureFlagSwitch, {
        props: { label, value: 'YES', onUpdate }
      });
      const button = getByRole('button');
      await fireEvent.click(button);
      expect(onUpdate).toHaveBeenCalledWith('NO');
    });
  });
});
