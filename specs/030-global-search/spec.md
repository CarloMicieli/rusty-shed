# Feature Specification: Global Search

**Feature Branch**: `030-global-search`
**Created**: 2026-02-26
**Status**: Draft
**Input**: Unified search interface querying collection and wishlist items with context-aware routing, debounced input, and Command Palette–style overlay.

---

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Find Any Item by Partial Text (Priority: P1)

A collector wants to quickly locate a specific model across their entire collection and wishlist without navigating menu by menu. They activate the search interface, type a partial term — a brand name, a road number, a livery colour, or a word from the description — and see matching items from both their collection and wishlist grouped and labelled by source. They click a result and land directly on that item's detail page.

**Why this priority**: This is the core value of the feature. A collector's library can span hundreds or thousands of items; finding a specific one instantly reduces the most common friction in day-to-day use.

**Independent Test**: Can be fully tested by activating search, entering a partial term that matches at least one collection item and at least one wishlist item, and verifying both appear with correct labels and correct navigation on click.

**Acceptance Scenarios**:

1. **Given** the user has items in both their collection and wishlist, **When** they activate search and type a partial brand name, **Then** matching items from both sources appear, each clearly labelled with its source (Collection or Wishlist).
2. **Given** results are displayed, **When** the user clicks a Collection result, **Then** they are navigated to that collection item's detail page.
3. **Given** results are displayed, **When** the user clicks a Wishlist result, **Then** they are navigated to that wishlist item's detail page.
4. **Given** the user types a partial road number, **Then** items whose road number contains that string are included in results.
5. **Given** the user types a partial livery or depot name, **Then** items matching that text in those fields are included in results.
6. **Given** the user types a word from a model's description, **Then** items containing that word in their description are included in results.

---

### User Story 2 - Debounced Input Prevents Overload (Priority: P1)

The search waits until the user pauses typing for 300 milliseconds before executing, ensuring the application remains responsive even during rapid keystrokes.

**Why this priority**: Without this behaviour the database would be queried on every keystroke, causing visible lag and degraded performance on lower-end hardware.

**Independent Test**: Can be tested by typing rapidly and verifying that no search is executed until typing stops for 300 ms, then exactly one search fires with the final value.

**Acceptance Scenarios**:

1. **Given** the user types multiple characters quickly, **When** less than 300 ms has elapsed since the last keystroke, **Then** no search is executed.
2. **Given** the user stops typing, **When** 300 ms passes, **Then** exactly one search is executed with the current input value.
3. **Given** the user clears the input entirely, **When** the input is empty, **Then** no search is executed and any previous results are cleared.

---

### User Story 3 - Empty State Guides the User (Priority: P2)

When a search term produces no matches in either collection or wishlist, the user sees a friendly empty state that explains the outcome and offers a direct path to adding a new model.

**Why this priority**: Prevents dead ends; especially important for new users with small collections who may search before adding items.

**Independent Test**: Can be tested by searching for a term guaranteed not to match any existing item and verifying the empty state message and the "Add a new model" call-to-action appear.

**Acceptance Scenarios**:

1. **Given** no items in collection or wishlist match the search term, **When** the search completes, **Then** a "No models found" message is displayed.
2. **Given** the empty state is shown, **When** the user activates the "Add a new model" option, **Then** they are directed to the new-model creation workflow.

---

### User Story 4 - Loading Indicator During Search (Priority: P3)

While the system is executing a search, a loading indicator is visible within the search bar so the user knows the action is being processed.

**Why this priority**: Removes ambiguity about whether the system received the input; important for perceived responsiveness on slower machines.

**Independent Test**: Can be tested by introducing an artificial delay and verifying the indicator appears immediately after the debounce fires and disappears once results render.

**Acceptance Scenarios**:

1. **Given** the debounce delay has elapsed and a search is in progress, **When** results have not yet returned, **Then** a loading indicator is visible within the search input area.
2. **Given** the search completes, **When** results or the empty state are displayed, **Then** the loading indicator disappears.

---

### Edge Cases

- What happens when the user activates search and immediately closes it without typing anything?
- How does partial matching handle brand names that contain punctuation (e.g., "A.C.M.E.")?
- If the same physical model exists as both a collection item and a wishlist item, are both results shown separately?
- When exactly 50 results are returned, is there a visual indicator that additional matches may exist beyond the displayed limit?
- How does the search behave when both the collection and wishlist are completely empty?
- What is shown if the search is activated during a slow operation that has locked the database momentarily?

---

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: Users MUST be able to activate the search interface from any screen via a dedicated keyboard shortcut and by clicking the search bar in the application header.
- **FR-002**: The search interface MUST delay executing a query until the user has stopped typing for 300 milliseconds.
- **FR-003**: The system MUST search across the following fields simultaneously: model description/details, series code, road number, depot, livery, and brand/manufacturer name.
- **FR-004**: The system MUST search across both collection items and wishlist items in a single operation, returning a unified result list.
- **FR-005**: Each search result MUST carry a source context label indicating whether it belongs to the Collection or the Wishlist.
- **FR-006**: Clicking a result labelled as Collection MUST navigate the user to that item's collection detail page.
- **FR-007**: Clicking a result labelled as Wishlist MUST navigate the user to that item's wishlist detail page.
- **FR-008**: Matching MUST be partial — a search term must match any item whose searchable fields contain that term as a substring, not only exact full-field matches.
- **FR-009**: The number of results returned per query MUST be capped at 50 items; results beyond this limit are not displayed in the initial view.
- **FR-010**: When a query returns zero results, the system MUST display a "No models found" message and present an option to add a new model.
- **FR-011**: Users MUST be able to type a search term in the header search bar and press Enter to navigate to a dedicated search results page at a stable URL containing the query string, where all matching results are displayed.
- **FR-012**: The search interface MUST display a loading indicator while a query is in progress.

### Key Entities

- **Search Result**: Represents a matched item. Carries: display name, brand/manufacturer, source context (Collection or Wishlist), and the route to that item's detail page.
- **Collection Item**: An item the user owns; has a unique detail page reachable via its collection identifier.
- **Wishlist Item**: An item the user intends to acquire; has a unique detail page reachable via its wishlist identifier.
- **Search Query**: A text string entered by the user; used for partial matching against all searchable fields; minimum 1 character to trigger a search.

---

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can locate any item by partial term (3 or more characters) and see results within 1 second of the debounce delay completing, across a collection of up to 1,000 items.
- **SC-002**: 100% of search results route the user to the correct detail page based on their source context (Collection vs. Wishlist) — no result ever navigates to the wrong page.
- **SC-003**: The search input remains fully responsive during active typing with no visible freeze or lag, regardless of collection size up to 5,000 items.
- **SC-004**: The empty state message and add-model prompt appear in 100% of cases where a valid search term returns zero results.
- **SC-005**: The loading indicator appears within 50 milliseconds of the debounce period completing and disappears no later than when the first result or empty state is rendered.

---

## Assumptions

- The search covers the currently authenticated user's data only; catalogue-wide search (models not yet in the collection or wishlist) is out of scope for this feature.
- Brand/manufacturer data is accessible alongside item records, so no additional lookup step is required to display it in results.
- The 50-result cap is applied at the data layer; no pagination is included in this initial version — a future iteration may add a "see all results" path if demand arises.
- Results are ordered by relevance (best match first); no user-controlled sort is required in this version.
- A model appearing in both the collection and the wishlist will produce two separate results, each linking to its respective detail page.
