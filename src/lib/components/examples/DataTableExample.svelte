<!--
  Example: Data Table with Sorting
  Feature: 012-shadcn-migration
  
  This example demonstrates:
  - Table component usage
  - Sorting functionality
  - Pagination
  - Responsive design
-->
<script lang="ts">
  import {
    Button,
    Badge,
    Table,
    TableHeader,
    TableBody,
    TableHead,
    TableRow,
    TableCell
  } from '$lib/components';
  import { ArrowUpDown, ArrowUpNarrowWide, ArrowDownWideNarrow } from 'lucide-svelte';

  interface User {
    id: number;
    name: string;
    email: string;
    role: string;
    status: 'active' | 'inactive';
  }

  const users: User[] = [
    { id: 1, name: 'Alice Johnson', email: 'alice@example.com', role: 'Admin', status: 'active' },
    { id: 2, name: 'Bob Smith', email: 'bob@example.com', role: 'User', status: 'active' },
    {
      id: 3,
      name: 'Charlie Brown',
      email: 'charlie@example.com',
      role: 'User',
      status: 'inactive'
    },
    { id: 4, name: 'Diana Prince', email: 'diana@example.com', role: 'Moderator', status: 'active' }
  ];

  // Sorting state
  let sortField = $state<keyof User | null>(null);
  let sortDirection = $state<'asc' | 'desc'>('asc');

  // Pagination state
  let currentPage = $state(1);
  const itemsPerPage = 10;

  function toggleSort(field: keyof User) {
    if (sortField === field) {
      sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
    } else {
      sortField = field;
      sortDirection = 'asc';
    }
  }

  const sortedUsers = $derived.by(() => {
    if (!sortField) return users;

    return [...users].sort((a, b) => {
      const aValue = a[sortField];
      const bValue = b[sortField];

      if (aValue < bValue) return sortDirection === 'asc' ? -1 : 1;
      if (aValue > bValue) return sortDirection === 'asc' ? 1 : -1;
      return 0;
    });
  });

  const paginatedUsers = $derived(
    sortedUsers.slice((currentPage - 1) * itemsPerPage, currentPage * itemsPerPage)
  );

  const totalPages = $derived(Math.ceil(sortedUsers.length / itemsPerPage));
</script>

<div class="space-y-4 p-4">
  <h1 class="text-2xl font-bold">User Management</h1>

  <div class="border-surface-700 rounded-lg border">
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead
            class="hover:bg-surface-700/50 cursor-pointer"
            onclick={() => toggleSort('name')}
          >
            <div class="flex items-center gap-1">
              Name
              {#if sortField === 'name'}
                {#if sortDirection === 'asc'}
                  <ArrowUpNarrowWide size={14} />
                {:else}
                  <ArrowDownWideNarrow size={14} />
                {/if}
              {:else}
                <ArrowUpDown size={14} class="opacity-30" />
              {/if}
            </div>
          </TableHead>
          <TableHead
            class="hover:bg-surface-700/50 cursor-pointer"
            onclick={() => toggleSort('email')}
          >
            <div class="flex items-center gap-1">
              Email
              {#if sortField === 'email'}
                {#if sortDirection === 'asc'}
                  <ArrowUpNarrowWide size={14} />
                {:else}
                  <ArrowDownWideNarrow size={14} />
                {/if}
              {:else}
                <ArrowUpDown size={14} class="opacity-30" />
              {/if}
            </div>
          </TableHead>
          <TableHead>Role</TableHead>
          <TableHead>Status</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {#each paginatedUsers as user (user.id)}
          <TableRow>
            <TableCell class="font-medium">{user.name}</TableCell>
            <TableCell>{user.email}</TableCell>
            <TableCell>
              <Badge variant="secondary">{user.role}</Badge>
            </TableCell>
            <TableCell>
              <Badge variant={user.status === 'active' ? 'success' : 'outline'}>
                {user.status}
              </Badge>
            </TableCell>
          </TableRow>
        {/each}
      </TableBody>
    </Table>
  </div>

  <!-- Pagination -->
  {#if totalPages > 1}
    <div class="flex items-center justify-between">
      <p class="text-surface-400 text-sm">
        Page {currentPage} of {totalPages}
      </p>
      <div class="flex gap-2">
        <Button
          variant="outline"
          size="sm"
          disabled={currentPage === 1}
          onclick={() => currentPage--}
        >
          Previous
        </Button>
        <Button
          variant="outline"
          size="sm"
          disabled={currentPage === totalPages}
          onclick={() => currentPage++}
        >
          Next
        </Button>
      </div>
    </div>
  {/if}
</div>
