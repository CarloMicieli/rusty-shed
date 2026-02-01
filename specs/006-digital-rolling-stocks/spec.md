# Feature Specification: Digital Rolling Stock Management

**Feature Branch**: `006-digital-rolling-stocks`  
**Created**: 2026-01-30  
**Status**: Draft  
**Input**: User description: "Build a new feature for the app to manage digital rolling stocks. Add a new page 'My Digital Rolling Stocks' in the navigation to let the user manage their digital roster. Include a summary section, a view indexed by DCC address, filtering, and a decoder installation popup."

## User Scenarios & Testing _(mandatory)_

### User Story 1 - View Digital Roster Summary (Priority: P1)

As a model railway collector, I want to see a summary of my digitalized rolling stock so that I can quickly understand what percentage of my motorized collection has been equipped with digital control.

**Why this priority**: This provides immediate value by giving users visibility into their digital conversion progress. It's the foundation for the digital management feature and requires no user interaction beyond viewing.

**Independent Test**: Can be fully tested by navigating to the "My Digital Rolling Stocks" page and verifying the summary statistics are displayed correctly based on the user's collection data.

**Acceptance Scenarios**:

1. **Given** the user has a collection with rolling stocks (some with decoders, some without), **When** they navigate to the "My Digital Rolling Stocks" page, **Then** they see the percentage of digital rolling stock displayed prominently
2. **Given** the user has rolling stocks with the "is_dummy" flag set to true, **When** the digital percentage is calculated, **Then** dummy rolling stocks are excluded from both the numerator and denominator
3. **Given** a rolling stock has "DCC_SOUND" or "DCC_FITTED" control type but no entry in the digital inventory, **When** the digital percentage is calculated, **Then** it is still counted as digital (factory-fitted decoder)
4. **Given** the user has no rolling stock in their collection, **When** they view the summary, **Then** they see a message indicating no data is available (or 0% with appropriate context)

---

### User Story 2 - Browse Digital Rolling Stock by DCC Address (Priority: P1)

As a model railway operator, I want to browse my digital rolling stock indexed by their DCC address so that I can ensure each locomotive has a unique address and avoid conflicts on my command station.

**Why this priority**: Core functionality that enables users to manage their digital roster. The DCC address is the primary identifier for digital operations.

**Independent Test**: Can be fully tested by viewing the digital roster list and verifying rolling stocks are displayed with their DCC addresses, categories, railway companies, scales, and power methods.

**Acceptance Scenarios**:

1. **Given** the user has digital rolling stock with assigned DCC addresses, **When** they view the digital roster, **Then** they see a list of rolling stock sorted/indexed by DCC address
2. **Given** a rolling stock has a "Function" type decoder (like passenger cars with lighting), **When** the digital roster is displayed, **Then** that rolling stock is excluded from the list
3. **Given** the user views a rolling stock entry, **When** they look at the details, **Then** they see the rolling stock category, railway company, scale, and power method
4. **Given** a rolling stock is factory-fitted (DCC_SOUND or DCC_FITTED control) but has no entry in digital_rolling_stocks, **When** the roster list is displayed, **Then** that rolling stock is NOT shown in the list (no DCC address to display), but IS counted in the summary percentage

---

### User Story 3 - Filter Digital Rolling Stock (Priority: P2)

As a model railway operator with a large collection, I want to filter my digital roster by DCC address or road number/description so that I can quickly locate a specific rolling stock.

**Why this priority**: Enhances usability for users with larger collections. Essential for practical use but the core viewing functionality works without it.

**Independent Test**: Can be fully tested by entering a search term in the filter field and verifying only matching rolling stocks are displayed.

**Acceptance Scenarios**:

1. **Given** the user is on the digital roster page, **When** they enter a DCC address number in the filter, **Then** only rolling stocks matching that address are displayed
2. **Given** the user is on the digital roster page, **When** they enter a road number or description text in the filter, **Then** only rolling stocks matching that text are displayed
3. **Given** the user applies a filter that matches no rolling stock, **When** they view the results, **Then** they see an empty state message indicating no matches found
4. **Given** the user has applied a filter, **When** they clear the filter, **Then** the full digital roster is displayed again

---

### User Story 4 - Change DCC Address (Priority: P2)

As a model railway operator, I want to change the DCC address of a rolling stock so that I can reassign addresses to avoid conflicts or reorganize my digital roster.

**Why this priority**: Critical for maintaining a conflict-free digital roster, but users can view their roster without this capability.

**Independent Test**: Can be fully tested by selecting a rolling stock, changing its DCC address, and verifying the change is persisted and reflected in the list.

**Acceptance Scenarios**:

1. **Given** the user selects a rolling stock from the digital roster, **When** they choose to change the DCC address and enter a new valid address, **Then** the address is updated successfully
2. **Given** the user enters a DCC address that is already assigned to another rolling stock, **When** they attempt to save, **Then** a warning message is displayed indicating the address conflict
3. **Given** a duplicate address warning is shown, **When** the user proceeds despite the warning, **Then** the address is still updated (soft warning, not a hard block)
4. **Given** the user enters an invalid DCC address (e.g., outside valid range 1-9999), **When** they attempt to save, **Then** a validation error is displayed and the save is prevented

> **Implementation Note**: Address validation MUST occur both client-side (immediate UX feedback) and server-side (Constitution: domain logic in Rust). The `DccAddress` value object in Rust enforces the 1-9999 constraint.

---

### User Story 5 - Install Decoder (Priority: P3)

As a model railway collector, I want to install a decoder into one of my rolling stocks so that I can digitalize my analog locomotives.

**Why this priority**: Extends the digital management capability to include adding new digital rolling stocks. The feature works for viewing existing digital stock without this.

**Independent Test**: Can be fully tested by opening the "Install Decoder" popup, selecting a rolling stock, choosing a decoder, setting a DCC address, and verifying the installation is recorded.

**Acceptance Scenarios**:

1. **Given** the user clicks "Install Decoder" button, **When** the popup appears, **Then** it slides in from the right side (consistent with app design patterns)
2. **Given** the decoder installation form is displayed, **When** the user views the form, **Then** they see fields for: rolling stock selection, decoder selection, installation date (defaulted to today), and DCC address
3. **Given** the user is selecting a rolling stock, **When** they view the dropdown, **Then** only rolling stocks from their collection that don't have function decoders are shown
4. **Given** the user completes the form with valid data, **When** they submit the installation, **Then** the digital rolling stock entry is created and appears in the digital roster
5. **Given** the user enters a DCC address already in use, **When** they view the form, **Then** a warning is displayed (but submission is allowed)

---

### User Story 6 - Replace Existing Decoder (Priority: P3)

As a model railway collector, I want to replace an existing decoder with a new one so that I can upgrade or fix a faulty decoder installation.

**Why this priority**: Edge case handling that completes the decoder installation workflow. Core installation works without this.

**Independent Test**: Can be fully tested by attempting to install a decoder on a rolling stock that already has one and verifying the confirmation dialog appears.

**Acceptance Scenarios**:

1. **Given** the user selects a rolling stock that already has a decoder installed, **When** they attempt to install a new decoder, **Then** a confirmation dialog asks if they want to replace the existing decoder
2. **Given** the user confirms the replacement, **When** the installation proceeds, **Then** the old decoder is replaced with the new one and the digital rolling stock is updated
3. **Given** the user cancels the replacement, **When** the dialog closes, **Then** no changes are made and they return to the installation form

---

### Edge Cases

- What happens when a user has no collection items at all? Display an empty state with guidance.
- How does the system handle DCC addresses at boundary values (1 and 9999)? Valid addresses within the DCC specification range should be accepted.
- What happens if the decoder selection dropdown has no decoders available? Display a message indicating no decoders are configured in the system.
- How does the system handle rolling stocks that were marked as DCC_FITTED/DCC_SOUND but have no digital_rolling_stock entry? They appear in the summary count as digital but not in the roster list (no DCC address to display).

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST display a "My Digital Rolling Stocks" navigation item in the main menu
- **FR-002**: System MUST calculate the digital rolling stock percentage excluding rolling stocks with the "is_dummy" flag set to true
- **FR-003**: System MUST count rolling stocks with "DCC_SOUND" or "DCC_FITTED" control values as digital, even without a digital_rolling_stock entry
- **FR-004**: System MUST display the digital roster as a list indexed/sortable by DCC address
- **FR-005**: System MUST exclude rolling stocks with "Function" type decoders from the main digital roster list
- **FR-006**: System MUST display category, railway company, scale, and power method for each digital rolling stock entry
- **FR-007**: System MUST provide a filter that searches by DCC address number or road number/description text
- **FR-008**: System MUST allow users to change the DCC address of a digital rolling stock
- **FR-009**: System MUST display a warning when the user enters a DCC address that is already assigned to another rolling stock
- **FR-010**: System MUST validate that DCC addresses are within the valid range (1-9999 per DCC specification)
- **FR-011**: System MUST provide an "Install Decoder" button that opens a right-sliding panel
- **FR-012**: System MUST allow selection of a rolling stock from the user's collection in the decoder installation form
- **FR-013**: System MUST allow selection of a decoder from the available decoders list
- **FR-014**: System MUST default the installation date to the current date
- **FR-015**: System MUST require a DCC address for new decoder installations
- **FR-016**: System MUST display a confirmation dialog when installing a decoder on a rolling stock that already has one
- **FR-017**: System MUST update the existing digital rolling stock entry when a decoder is replaced

### Key Entities

- **Digital Rolling Stock**: Represents a rolling stock with an installed decoder. Contains: unique ID, reference to owned rolling stock, DCC address, reference to decoder, and installation metadata.
- **Owned Rolling Stock**: A rolling stock item in the user's collection. Contains: reference to catalog rolling stock, notes, and optionally digital setup information.
- **Decoder**: A master record for a decoder product. Contains: manufacturer, product code, decoder type (Plain, Sound, Function, MultiProtocol), digital protocol, and physical interface type.
- **DCC Address**: A numeric value (1-9999) that uniquely identifies a decoder on a DCC command station. Should be unique within a user's collection but the system allows duplicates with warnings.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can view their digital rolling stock summary within 1 second of navigating to the page
- **SC-002**: Users can locate a specific rolling stock using the filter in under 5 seconds for collections with up to 500 items
- **SC-003**: 95% of users can successfully install a decoder on their first attempt without documentation
- **SC-004**: Users receive immediate visual feedback (warning) when entering a duplicate DCC address before submitting
- **SC-005**: All user actions (view, filter, edit address, install decoder) complete within 2 seconds
- **SC-006**: The digital percentage calculation correctly reflects the user's collection state as defined in the requirements
