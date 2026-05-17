export type LibraryTab = 'manufacturers' | 'sellers' | 'buyers';

export interface LibraryUiState {
  activeTab: LibraryTab;
  searchQuery: string;
  loading: boolean;
  error: string | null;
}
