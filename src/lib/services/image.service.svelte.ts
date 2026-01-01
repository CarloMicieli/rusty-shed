import { safeInvoke } from '$lib/services';

export class ImageService {
  async resolveImagePath(id: string, category: string): Promise<string | null> {
    const result = await safeInvoke<string>('get_image_path', { id, category });
    if (!result.ok) {
      console.error('Failed to resolve image path:', result.error);
      return null;
    }
    return result.data;
  }
}

export const imageService = new ImageService();
