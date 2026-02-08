# Implementation Checklist: ZIP Export Feature

**Date**: February 8, 2026  
**Feature**: 016-Data Archive Export  
**Based on Research**: Complete library analysis and benchmarks

---

## Phase 1: Foundation (Weeks 1-2)

### Infrastructure: archive_writer.rs

- [ ] Create `src-tauri/src/export/infrastructure/archive_writer.rs`
- [ ] Implement `ArchiveWriter` struct wrapping `ZipWriter`
- [ ] Configure Deflate compression level 6
- [ ] Implement methods:
  - [ ] `new(path: &Path) -> Result<Self>`
  - [ ] `add_file(&mut self, name: &str, content: &[u8]) -> Result<u64>`
  - [ ] `finish(self) -> Result<()>`
- [ ] Add progress callback trait
- [ ] Error handling (custom error type)
- [ ] Unit tests (basic file addition)

### Infrastructure: manifest_builder.rs

- [ ] Create `src-tauri/src/export/infrastructure/manifest_builder.rs`
- [ ] Import `ManifestDto` from import feature
- [ ] Implement `ManifestBuilder` struct
- [ ] Methods:
  - [ ] `from_database(models, items, sellers) -> Result<ManifestDto>`
  - [ ] `to_json(&self) -> Result<Vec<u8>>`
  - [ ] `add_model(&mut self, model: RailwayModel)`
  - [ ] `add_collection_item(&mut self, item: CollectionItem)`
- [ ] Validation logic
- [ ] Unit tests (serialization, round-trip)

### Application: execute_export.rs

- [ ] Create `src-tauri/src/export/application/execute_export.rs`
- [ ] Implement `ExportUseCase` struct
- [ ] Methods:
  - [ ] `collect_data(&self) -> Result<ExportData>`
  - [ ] `create_archive(&self, destination: &Path) -> Result<String>`
  - [ ] `with_progress(&self, callback: ProgressCallback) -> Result<()>`
- [ ] Repository integration (read-only)
- [ ] Database queries
- [ ] Error propagation

### Interface: commands.rs

- [ ] Create `src-tauri/src/export/interface/commands.rs`
- [ ] Create Tauri command: `cmd_export_collection()`
- [ ] Parameters: destination, entity_types
- [ ] Returns: file path, size in bytes
- [ ] Error handling (user-friendly messages)
- [ ] Window state management

### Testing: Unit Tests

- [ ] Test archive creation with mock files
- [ ] Test compression works (verify file size reduction)
- [ ] Test manifest serialization
- [ ] Test error handling (no disk space, invalid path)
- [ ] Test file permissions

**Acceptance Criteria**:

- [ ] Can create valid ZIP archive
- [ ] Archive contains manifest.json
- [ ] Compression ratio 60-70% for mixed content
- [ ] Memory usage stays <500MB for 500MB+ exports
- [ ] Export completes in reasonable time (<10s for 500MB)

---

## Phase 2: Progress & Features (Weeks 2-3)

### Progress Tracking

- [ ] Define `ExportProgress` struct (phase, percentage, eta)
- [ ] Implement phase-based progress calculation
- [ ] Implement ETA calculation (based on rate)
- [ ] Create progress event enum
- [ ] Emit events to frontend every 500ms
- [ ] Update progress after each file

### Error Handling & Recovery

- [ ] Implement temp file pattern (write to .tmp)
- [ ] Atomic rename on success
- [ ] Cleanup on failure (delete .tmp)
- [ ] Disk space validation before export
- [ ] Graceful error messages
- [ ] User cancellation support

### Additional Infrastructure

- [ ] Create `file_picker.rs` (Tauri dialog integration)
- [ ] Create `media_collector.rs` (image gathering)
- [ ] Create `disk_space_checker.rs` (space validation)

### Frontend Integration

- [ ] Create `src/lib/features/export/ExportDialog.svelte`
- [ ] Create `src/lib/features/export/ExportProgress.svelte`
- [ ] Listen to export:progress events
- [ ] Display progress bar with percentage
- [ ] Show ETA in user-friendly format
- [ ] Show completion notification
- [ ] Show error notifications

### Testing: Integration Tests

- [ ] Test export → import round-trip (full data preservation)
- [ ] Test progress events arrive within 500ms intervals
- [ ] Test cancellation mid-export
- [ ] Test error recovery (incomplete ZIP deleted)
- [ ] Test large exports (1000+ files)

**Acceptance Criteria**:

- [ ] Progress updates every 500ms
- [ ] ETA accurate within 10%
- [ ] No partial archives on failure
- [ ] Exports 500MB in <10 seconds
- [ ] Memory never exceeds 500MB

---

## Phase 3: Polish & Optimization (Weeks 3+)

### Performance Optimization

- [ ] Profile compression performance
- [ ] Tune buffer sizes if needed
- [ ] Benchmark different compression levels
- [ ] Add "fast export" option (level 1)
- [ ] Document performance characteristics

### Additional Features

- [ ] Compression level selection (UI option)
- [ ] tar.gz alternative format (feature flag)
- [ ] Orphaned media file detection
- [ ] Export preview/summary display
- [ ] Entity type selection checkboxes
- [ ] Archive size estimation

### Documentation

- [ ] Add code comments
- [ ] Document public API
- [ ] Create user guide
- [ ] Add troubleshooting section
- [ ] Document architecture decisions

### Edge Cases & Testing

- [ ] Test with very large files (>1GB)
- [ ] Test with thousands of small files
- [ ] Test on slow storage (external HDD)
- [ ] Test with read-only destination
- [ ] Test with special characters in filenames
- [ ] Test concurrent exports (if needed)

**Acceptance Criteria**:

- [ ] All edge cases handled gracefully
- [ ] Performance within benchmarks
- [ ] Documentation complete
- [ ] Code coverage >80%

---

## Code Structure

### Create These Files

```
src-tauri/src/export/
├── mod.rs                                    (module entry point)
├── domain/
│   ├── mod.rs
│   ├── export_session.rs                    (aggregate)
│   ├── export_config.rs                     (value object)
│   ├── entity_selection.rs                  (value object)
│   └── export_result.rs                     (value object)
├── application/
│   ├── mod.rs
│   ├── execute_export.rs                    (use case - PRIMARY)
│   ├── preview_export.rs                    (use case - secondary)
│   └── collect_export_data.rs               (helper)
├── infrastructure/
│   ├── mod.rs
│   ├── archive_writer.rs                    (ZipWriter wrapper - PRIMARY)
│   ├── manifest_builder.rs                  (JSON serialization - PRIMARY)
│   ├── media_collector.rs                   (image gathering)
│   ├── file_picker.rs                       (Tauri dialog)
│   └── disk_space_checker.rs                (space validation)
└── interface/
    ├── mod.rs
    └── commands.rs                          (Tauri IPC - PRIMARY)

src/lib/features/export/
├── components/
│   ├── ExportDialog.svelte                  (main workflow)
│   ├── ExportEntitySelector.svelte          (entity selection)
│   ├── ExportPreview.svelte                 (preview summary)
│   ├── ExportProgress.svelte                (progress display)
│   └── ExportReport.svelte                  (completion report)
├── export.controller.svelte.ts              (state management)
└── types.ts                                 (frontend types)
```

### Update Existing Files

```
src-tauri/src/lib.rs
├── Add: mod export;

src-tauri/Cargo.toml
├── Check: zip = "0.6" already included
├── Optional: upgrade to zip = "7.4.0"

src/app.d.ts
├── Add: export command types (generated by specta)

messages/en.json
├── Add: export.* i18n keys

messages/it.json
├── Add: export.* translations
```

---

## Testing Checklist

### Unit Tests (archive_writer.rs)

- [ ] `test_create_simple_zip` - Basic file addition
- [ ] `test_compression_ratio` - Verify 60-70% compression
- [ ] `test_large_file_handling` - Files >1GB
- [ ] `test_file_ordering` - Files in correct order
- [ ] `test_memory_efficiency` - 164KB memory with 1000 files
- [ ] `test_error_on_invalid_path` - Handle bad paths
- [ ] `test_error_on_disk_full` - Handle disk space issues

### Unit Tests (manifest_builder.rs)

- [ ] `test_serialize_manifest` - Valid JSON output
- [ ] `test_deserialize_manifest` - Roundtrip compatibility
- [ ] `test_add_model` - Model serialization
- [ ] `test_add_collection_item` - Item serialization
- [ ] `test_validation_fails_missing_required` - Validation works

### Integration Tests (export feature)

- [ ] `test_end_to_end_export` - Full export workflow
- [ ] `test_round_trip_import` - Export → import identity
- [ ] `test_progress_events` - Progress emitted correctly
- [ ] `test_cancellation` - Cancel mid-export
- [ ] `test_large_collection` - 1000+ records + 500 images
- [ ] `test_error_recovery` - Partial ZIP cleaned up
- [ ] `test_disk_space_validation` - Rejected if insufficient space

### Performance Tests

- [ ] `bench_500_images` - Should complete <5 seconds
- [ ] `bench_5000_records` - Should complete <10 seconds
- [ ] `bench_memory_profile` - Should stay <500MB
- [ ] `bench_compression_level_1_vs_6_vs_9` - Compare speeds

---

## Dependency Status

### Already Available

```toml
zip = "0.6"       # Already in Cargo.toml ✅
flate2 = "1.1"    # Already in Cargo.toml ✅
serde_json = "..."  # Already in Cargo.toml ✅
tauri = "2.x"     # Already in Cargo.toml ✅
tokio = "1.x"     # Implicit via tauri ✅
```

### No New Dependencies Required ✅

All necessary libraries are already included. Recommend optional upgrade:

```toml
# Optional: Upgrade for better performance
# zip = "7.4.0"  # Adds Zstandard support, 20% faster
```

---

## Git Workflow

### Branch Strategy

```
main ← feature/export-archive ← archive-writer, progress-tracking, error-recovery
                                 ↓
                               Phase 1 PR (working MVP)
                                 ↓
                               Phase 2 PR (progress + features)
                                 ↓
                               Phase 3 PR (polish + optimization)
```

### Commit Messages (Conventional Commits)

```
feat: implement streaming ZIP export with progress tracking
feat: add archive_writer for ZipWriter integration
feat: add manifest_builder for export serialization
feat: implement execute_export use case
feat: add progress callbacks and event emission
feat: implement disk space validation
fix: handle disk full and invalid path errors
test: add round-trip export/import validation
docs: add export feature architecture documentation
```

---

## Definition of Done

- [ ] Code follows project style guide
- [ ] All unit tests pass (coverage >80%)
- [ ] All integration tests pass
- [ ] Performance benchmarks met (<10s for 500MB)
- [ ] Memory efficiency verified (<500MB)
- [ ] Round-trip validation passes (export → import identity)
- [ ] Works on Windows, macOS, Linux
- [ ] Code reviewed and approved
- [ ] Documentation complete
- [ ] User-facing strings use Paraglide-JS
- [ ] Error messages are user-friendly
- [ ] No clippy warnings
- [ ] rustfmt passes

---

## Timeline Estimate

| Phase       | Tasks                                            | Effort   | Duration  |
| ----------- | ------------------------------------------------ | -------- | --------- |
| **Phase 1** | Archive writer, manifest builder, basic commands | 36 hours | 1 week    |
| **Phase 2** | Progress tracking, error handling, features      | 24 hours | 3-4 days  |
| **Phase 3** | Optimization, polish, documentation              | 16 hours | 2-3 days  |
| **Total**   | Full feature implementation                      | 76 hours | 2-3 weeks |

---

## Success Metrics

✅ **Memory**: Stay <500MB for any export size  
✅ **Speed**: Complete 500MB export in <10 seconds  
✅ **Progress**: Updates every 500ms minimum  
✅ **Reliability**: 100% valid archives verified by import  
✅ **Compatibility**: Works on all Tauri platforms  
✅ **Compression**: 60-70% for mixed content  
✅ **Error Handling**: No partial archives on failure  
✅ **User Experience**: Clear progress feedback with ETA

---

**Status**: Ready for implementation  
**Research**: ✅ Complete  
**Architecture**: ✅ Defined  
**Dependencies**: ✅ Available  
**Timeline**: ✅ Estimated

---

_Next: Start with Phase 1, Week 1: Implement archive_writer.rs_
