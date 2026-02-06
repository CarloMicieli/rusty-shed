import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';

// Mock paraglide messages
vi.mock('$lib/paraglide/messages.js', () => ({
  model_no_details: () => 'No detailed description available for this model.'
}));

import ModelDetailsContent from '../ModelDetailsContent.svelte';

describe('ModelDetailsContent', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Rendering', () => {
    it('should render details text when provided', () => {
      const details = 'This is a detailed description of the railway model.';
      
      render(ModelDetailsContent, {
        props: {
          details
        }
      });

      expect(screen.getByText(details)).toBeInTheDocument();
    });

    it('should render empty state message when details is null', () => {
      render(ModelDetailsContent, {
        props: {
          details: null
        }
      });

      expect(screen.getByText('No detailed description available for this model.')).toBeInTheDocument();
    });

    it('should render empty state message when details is undefined', () => {
      render(ModelDetailsContent, {
        props: {
          details: undefined
        }
      });

      expect(screen.getByText('No detailed description available for this model.')).toBeInTheDocument();
    });

    it('should render empty state message when details is empty string', () => {
      render(ModelDetailsContent, {
        props: {
          details: ''
        }
      });

      expect(screen.getByText('No detailed description available for this model.')).toBeInTheDocument();
    });

    it('should render multiline details correctly', () => {
      const details = 'Line 1\nLine 2\nLine 3';
      
      render(ModelDetailsContent, {
        props: {
          details
        }
      });

      // Check for presence of each line
      expect(screen.getByText(/Line 1/)).toBeInTheDocument();
      expect(screen.getByText(/Line 2/)).toBeInTheDocument();
      expect(screen.getByText(/Line 3/)).toBeInTheDocument();
    });
  });

  describe('Styling', () => {
    it('should apply prose classes for content styling', () => {
      const { container } = render(ModelDetailsContent, {
        props: {
          details: 'Test content'
        }
      });

      const proseContainer = container.querySelector('.prose');
      expect(proseContainer).toBeInTheDocument();
      expect(proseContainer?.className).toContain('dark:prose-invert');
      expect(proseContainer?.className).toContain('max-w-none');
    });

    it('should apply muted foreground to empty state', () => {
      const { container } = render(ModelDetailsContent, {
        props: {
          details: null
        }
      });

      const emptyState = screen.getByText('No detailed description available for this model.');
      expect(emptyState.className).toContain('text-muted-foreground');
    });
  });

  describe('Content Formatting', () => {
    it('should render text content in paragraph', () => {
      const details = 'Simple text content';
      
      render(ModelDetailsContent, {
        props: {
          details
        }
      });

      expect(screen.getByText(details)).toBeInTheDocument();
    });

    it('should handle long text content', () => {
      const longDetails = 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. '.repeat(20);
      
      render(ModelDetailsContent, {
        props: {
          details: longDetails
        }
      });

      // Use a partial match since long text may be split
      expect(screen.getByText(/Lorem ipsum dolor sit amet/)).toBeInTheDocument();
    });

    it('should handle special characters', () => {
      const details = 'Model with special chars: é, ü, ñ';
      
      render(ModelDetailsContent, {
        props: {
          details
        }
      });

      expect(screen.getByText(/Model with special chars/)).toBeInTheDocument();
    });
  });

  describe('Accessibility', () => {
    it('should render semantic paragraph element', () => {
      const details = 'Test content';
      
      const { container } = render(ModelDetailsContent, {
        props: {
          details
        }
      });

      const paragraph = container.querySelector('p');
      expect(paragraph).toBeInTheDocument();
      expect(paragraph?.textContent).toBe(details);
    });

    it('should be readable by screen readers', () => {
      const details = 'Important railway model information';
      
      render(ModelDetailsContent, {
        props: {
          details
        }
      });

      const text = screen.getByText(details);
      expect(text).toBeVisible();
    });
  });
});
