<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import type { Editor } from '@tiptap/core';
  import { Bold, Italic, List, ListOrdered } from 'lucide-svelte';
  import { Button } from '$lib/components/ui/button';

  let { editor }: { editor: Editor | null } = $props();

  let isBold = $derived(editor?.isActive('bold') ?? false);
  let isItalic = $derived(editor?.isActive('italic') ?? false);
  let isBullet = $derived(editor?.isActive('bulletList') ?? false);
  let isOrdered = $derived(editor?.isActive('orderedList') ?? false);
</script>

<div class="flex items-center gap-1 border-b border-border/40 px-2 py-1">
  <Button
    variant={isBold ? 'secondary' : 'ghost'}
    size="icon"
    class="h-7 w-7"
    onclick={() => editor?.chain().focus().toggleBold().run()}
    aria-label={m.toolbar_bold()}
  >
    <Bold class="h-4 w-4" />
  </Button>

  <Button
    variant={isItalic ? 'secondary' : 'ghost'}
    size="icon"
    class="h-7 w-7"
    onclick={() => editor?.chain().focus().toggleItalic().run()}
    aria-label={m.toolbar_italic()}
  >
    <Italic class="h-4 w-4" />
  </Button>

  <div class="mx-1 h-4 w-px bg-border/40"></div>

  <Button
    variant={isBullet ? 'secondary' : 'ghost'}
    size="icon"
    class="h-7 w-7"
    onclick={() => editor?.chain().focus().toggleBulletList().run()}
    aria-label={m.toolbar_bullet_list()}
  >
    <List class="h-4 w-4" />
  </Button>

  <Button
    variant={isOrdered ? 'secondary' : 'ghost'}
    size="icon"
    class="h-7 w-7"
    onclick={() => editor?.chain().focus().toggleOrderedList().run()}
    aria-label={m.toolbar_ordered_list()}
  >
    <ListOrdered class="h-4 w-4" />
  </Button>
</div>
