// Export feature types

export interface ExportPreview {
  railway_model_count: number;
  collection_item_count: number;
  seller_count: number;
  maintenance_log_count: number;
  dcc_roster_count: number;
  image_count: number;
  orphaned_image_count: number;
  estimated_size_bytes: number;
  warnings: string[];
}

export interface ExportEntitySelection {
  include_railway_models: boolean;
  include_collection_items: boolean;
  include_sellers: boolean;
  include_maintenance_logs: boolean;
  include_dcc_roster: boolean;
  include_orphaned_images: boolean;
}

export interface ExportResult {
  archive_path: string;
  file_size_bytes: number;
  records_exported: number;
  warnings: string[];
}
