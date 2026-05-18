export type LibraryTab = 'manufacturers' | 'sellers';

export interface LibraryUiState {
  activeTab: LibraryTab;
  searchQuery: string;
  loading: boolean;
  error: string | null;
}
