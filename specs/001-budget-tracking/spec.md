# Feature Specification: Budget Tracking

**Feature Branch**: `001-budget-tracking`  
**Created**: January 30, 2026  
**Status**: Draft  
**Input**: User description: "Budget tracking feature allowing rail hobbyists to manage spending with financial goals, category tracking, and roll-over mechanics"

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Set Monthly/Yearly Budget (Priority: P1)

As a rail hobbyist, I want to set my hobby budget so that I can track my spending against a financial goal. Users can choose to set either a yearly budget (which is automatically divided into monthly amounts) or a monthly budget directly.

**Why this priority**: This is the foundation of the feature - without setting a budget, no tracking or analysis can occur. It delivers immediate value by establishing the user's financial planning baseline.

**Independent Test**: Can be fully tested by opening the budget configuration page, entering a yearly or monthly amount, saving it, and verifying the budget is stored and displayed correctly on the dashboard. Delivers the value of having a defined spending limit.

**Acceptance Scenarios**:

1. **Given** a new user opens the budget configuration, **When** they enter a yearly budget of $1200, **Then** the system calculates and displays a monthly budget of $100
2. **Given** a user has set a yearly budget, **When** they switch to monthly budget mode, **Then** the current monthly amount is pre-filled and can be edited independently
3. **Given** a user enters budget values, **When** they save the configuration, **Then** the budget is persisted and reflected in all dashboard views
4. **Given** a user has not set a budget, **When** they view the dashboard, **Then** they see a prompt to configure their budget first

---

### User Story 2 - Track Monthly Spending with Rollover (Priority: P1)

As a rail hobbyist, I want my unspent budget to roll over to the next month so that I can save up for larger purchases. When spending exceeds the budget, the deficit reduces next month's available funds.

**Why this priority**: This is the core mechanic that differentiates this feature from simple expense tracking. It enables users to accumulate funds for expensive items and provides realistic budget management.

**Independent Test**: Can be fully tested by setting a $100 monthly budget, recording $80 in purchases in month 1, and verifying that month 2 shows $120 available budget ($100 base + $20 rollover). Also test overspending to verify deficit handling.

**Acceptance Scenarios**:

1. **Given** a user has a $100 monthly budget and spent $80 in January, **When** February arrives, **Then** the available budget shows $120 ($100 + $20 rollover)
2. **Given** a user spent $120 against a $100 budget in March, **When** April arrives, **Then** the available budget shows $80 ($100 - $20 deficit)
3. **Given** a user has accumulated a $50 surplus over several months, **When** they make a $150 purchase, **Then** the system shows they used $100 base budget plus $50 from rollover
4. **Given** December 31st passes, **When** January 1st arrives, **Then** the rollover balance resets to $0 for the new fiscal year

---

### User Story 3 - View Budget Status on Dashboard (Priority: P1)

As a rail hobbyist, I want to see my budget status at a glance on the dashboard so that I can make informed purchasing decisions. The dashboard displays three key visualizations: budget remaining (donut chart), yearly spending (bar chart), and 5-year activity (heatmap).

**Why this priority**: The dashboard provides immediate visibility into budget health and spending patterns. It's the primary interface users interact with and must deliver value from day one.

**Independent Test**: Can be fully tested by setting a budget, recording some purchases, and verifying that the dashboard accurately displays: (1) remaining budget percentage, (2) monthly spending bars, and (3) activity intensity over time. Delivers value by providing actionable financial insights.

**Acceptance Scenarios**:

1. **Given** a user has spent $25 of their $100 monthly budget, **When** they view the dashboard, **Then** the donut chart shows 75% remaining with a green-to-yellow color gradient
2. **Given** a user has spending data for the current year, **When** they view the dashboard, **Then** the bar chart displays 12 vertical bars (one per month) with the current month updated in real-time
3. **Given** a user spent $120 against a $100 monthly budget, **When** they view the yearly spending chart, **Then** the bar for that month exceeds the budget goal line displayed horizontally
4. **Given** a user has 5 years of spending history, **When** they view the dashboard, **Then** the activity grid displays a heatmap with color intensity representing spending levels by quarter
5. **Given** a user is approaching their budget limit, **When** the remaining budget drops below 20%, **Then** the donut chart color transitions to red as a warning

---

### User Story 4 - Add Extra Budget (One-time Funds) (Priority: P2)

As a rail hobbyist, I want to add extra funds to a specific month so that I can account for unexpected income like gifts, bonuses, or income from selling old equipment.

**Why this priority**: While not critical for basic budget tracking, this feature enables users to accurately model real-world financial scenarios where additional funds become available.

**Independent Test**: Can be fully tested by opening a specific month, clicking "Add Extra Budget", entering $50 as a gift, and verifying that the month's available budget increases by $50 and is reflected in the dashboard.

**Acceptance Scenarios**:

1. **Given** a user has a $100 monthly budget for March, **When** they add $50 as extra budget for March, **Then** March shows a total available budget of $150 (before rollover)
2. **Given** a user opens the extra budget modal for April, **When** they enter $200 and provide a reason "Sold vintage locomotive", **Then** the extra budget is recorded and displayed in the budget management table
3. **Given** a user has added extra budget to a month, **When** the month ends with surplus, **Then** the surplus (including extra budget portion) rolls over to the next month
4. **Given** a user wants to remove extra budget, **When** they open the extra budget modal and set the amount to $0, **Then** the extra budget is cleared for that month

---

### User Story 5 - View Detailed Budget Management Page (Priority: P2)

As a rail hobbyist, I want a detailed view of my budget breakdown so that I can analyze spending patterns, manage monthly allocations, and review historical data. This page provides a table for the current year and an archive view for the past 5 years.

**Why this priority**: This provides power users with detailed analysis capabilities beyond the dashboard overview. It's valuable but not essential for basic budget tracking.

**Independent Test**: Can be fully tested by navigating to the budget management page and verifying: (1) a 12-row table showing current year data (base budget, extra budget, actual spend, rollover, status), and (2) an accordion showing quarterly summaries for the past 5 years.

**Acceptance Scenarios**:

1. **Given** a user navigates to the budget management page, **When** the page loads, **Then** they see a 12-row table (Jan-Dec) with columns for Base Budget, Extra Budget, Actual Spend, Roll-over, and Final Status
2. **Given** the current month is March, **When** viewing the budget table, **Then** Jan, Feb show completed data, March shows in-progress data, and Apr-Dec show projected budgets
3. **Given** a user has 5 years of historical data, **When** they expand the historical archive, **Then** they see accordion sections for each year (2025-2021) with quarterly summaries (Q1-Q4)
4. **Given** a user clicks on a quarter in the historical view, **When** the quarterly summary modal opens, **Then** it displays total spending for that quarter broken down by category
5. **Given** a user toggles between Yearly and Monthly mode in the header, **When** they make the change, **Then** all budget calculations update accordingly across all views

---

### User Story 6 - Track Spending by Category (Priority: P2)

As a rail hobbyist, I want to see my spending broken down by category so that I can understand where my hobby budget is going. Categories include: Locomotives, Passenger Cars, Freight Cars, EMU, Railcars, Starter Sets, Railway Tracks, and Decoders.

**Why this priority**: Category tracking provides valuable insights but isn't required for basic budget functionality. Users can track total spending first and add category analysis later.

**Independent Test**: Can be fully tested by recording purchases in different categories, then viewing the quarterly summary modal to verify spending is correctly attributed and totaled by category.

**Acceptance Scenarios**:

1. **Given** a user records purchases across multiple categories in Q1, **When** they view the Q1 summary, **Then** spending is broken down showing amounts for each category (Locomotives: $200, Tracks: $50, etc.)
2. **Given** a user has 5 years of purchase history, **When** they view historical data, **Then** category totals are preserved for each quarter
3. **Given** a user clicks on a category in the breakdown, **When** the detail view opens, **Then** they see a list of individual purchases in that category for the selected period
4. **Given** a user has made purchases in only 3 categories, **When** viewing the breakdown, **Then** only categories with spending are displayed (empty categories are hidden)

---

### Edge Cases

- **What happens when a user changes their budget mid-month?** The new budget takes effect immediately. The rollover calculation for the current month uses the new budget amount. Historical months retain their original budget values.

- **How does the system handle the transition from December to January with rollover reset?** On January 1st, the rollover balance resets to $0, but the actual spending history from December is preserved. The reset only affects the carried-forward balance, not the historical record.

- **What happens when a user deletes a purchase from a previous month that affects rollover?** The system recalculates the rollover chain from the modified month forward. All subsequent months' rollover values are updated automatically.

- **How are leap years handled in the yearly budget calculation?** The yearly budget is divided by 12 regardless of leap years, maintaining consistent monthly amounts. The extra day in February does not affect budget allocation.

- **What happens when a user has no purchases in a month?** The full monthly budget amount rolls over to the next month. The bar chart shows a $0 bar for that month, and the available budget for the next month increases by the full amount.

- **How does the system handle negative rollover exceeding the monthly budget?** If the deficit is larger than the next month's base budget, the available budget can go negative. The donut chart shows 0% remaining and turns red. Users are warned but not blocked from making purchases.

- **What happens to historical quarterly data when a user changes which categories they track?** Historical data retains the original category structure. New categories only appear in data from the point they were added forward. This prevents retroactive data corruption.

- **How does currency formatting handle internationalization?** The system uses the currency setting from app preferences to format all monetary values consistently. When the currency changes, all displayed values update to the new currency symbol and formatting rules.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST allow users to set either a yearly budget (automatically divided into 12 monthly amounts) or a monthly budget amount
- **FR-002**: System MUST persist budget configuration and apply it consistently across all views
- **FR-003**: System MUST allow users to add extra one-time budget to any specific month via a modal interface
- **FR-004**: System MUST calculate monthly rollover by subtracting actual spending from the available budget (base budget + extra budget + previous rollover)
- **FR-005**: System MUST reset rollover balance to $0 on January 1st of each year while preserving historical spending data
- **FR-006**: System MUST track spending in the following categories: Locomotives, Passenger Cars, Freight Cars, EMU, Railcars, Starter Sets, Railway Tracks, and Decoders
- **FR-007**: System MUST maintain current year spending data at monthly granularity for real-time calculations
- **FR-008**: System MUST aggregate historical spending data (older than current year) into quarterly periods (Q1-Q4) for the past 5 years
- **FR-009**: System MUST display a donut chart showing remaining budget percentage for the current month with dynamic color gradient (green >50%, yellow 20-50%, red <20%)
- **FR-010**: System MUST display a bar chart showing monthly spending for all 12 months with a horizontal budget goal line overlay
- **FR-011**: System MUST display an activity heatmap showing spending intensity by quarter for the past 5 years
- **FR-012**: System MUST provide a budget management page with a 12-row table showing: Base Budget, Extra Budget, Actual Spend, Roll-over, and Final Status for each month
- **FR-013**: System MUST provide a historical archive view with accordion sections for each of the past 5 years showing quarterly summaries
- **FR-014**: System MUST inherit currency formatting from the application's global settings
- **FR-015**: System MUST recalculate rollover chain when historical purchases are added, modified, or deleted
- **FR-016**: System MUST allow users to toggle between Yearly and Monthly budget input modes
- **FR-017**: System MUST display a quarterly summary modal when users click on a quarter in the activity heatmap, showing category breakdown
- **FR-018**: System MUST validate that budget amounts are non-negative numeric values
- **FR-019**: System MUST handle months where spending exceeds budget by showing negative rollover that reduces subsequent month's available funds
- **FR-020**: System MUST preserve category totals in historical quarterly data even when category definitions change

### Key Entities

- **Budget Configuration**: Represents the user's financial planning setup including base amount (yearly or monthly), currency preference, and calculation mode. This is a singleton entity per user.

- **Monthly Budget Record**: Represents a specific month's budget data including base budget amount, any extra budget added, actual spending total, rollover from previous month, and final status. Each record is tied to a specific year-month pair.

- **Extra Budget Entry**: Represents a one-time injection of funds into a specific month, including the amount and optional reason (gift, bonus, sale proceeds). Multiple entries can exist per month.

- **Quarterly Spending Summary**: Represents aggregated spending data for a specific quarter (Q1-Q4) in a specific year. Includes total spending and breakdown by category. Used for historical data older than the current year.

- **Category Spending**: Represents the amount spent in a specific category (Locomotives, Passenger Cars, etc.) within a given time period (month or quarter). Links spending to the defined categories.

- **Purchase Record**: Represents an individual purchase transaction that contributes to spending calculations. Includes amount, category, date, and optional description. This entity likely exists elsewhere in the system and is referenced by the budget feature.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can configure their budget (yearly or monthly) in under 60 seconds from first opening the budget settings
- **SC-002**: Users can view their current budget status (remaining percentage and monthly spending) on the dashboard within 2 seconds of page load
- **SC-003**: The system accurately calculates and displays rollover balances for all 12 months, handling both surplus and deficit scenarios without data corruption
- **SC-004**: Historical data aggregation reduces display complexity by grouping 5 years of data into 20 quarterly periods (4 quarters × 5 years) instead of 60 monthly periods
- **SC-005**: The donut chart updates in real-time within 1 second when a new purchase is recorded
- **SC-006**: Users can add extra budget to any month in under 30 seconds using the modal interface
- **SC-007**: The yearly spending bar chart displays all 12 months with the budget goal line clearly visible, allowing users to identify over-budget months at a glance
- **SC-008**: 90% of users can understand their budget status (remaining vs. spent) within 5 seconds of viewing the dashboard
- **SC-009**: The activity heatmap allows users to compare spending intensity across 5 years of history without visual clutter
- **SC-010**: Category breakdowns in quarterly summaries account for 100% of the spending total with no discrepancies
- **SC-011**: Budget recalculations complete within 3 seconds when historical purchases are modified
- **SC-012**: The feature operates correctly across all currency formats supported by the application settings

## Assumptions

- **Currency Consistency**: All budget and spending amounts use the same currency as defined in the application's global settings. Currency conversion is not supported within the budget feature.

- **Purchase Data Source**: The feature assumes that purchase records (with date, amount, and category) are already captured elsewhere in the application and are accessible for budget calculations.

- **Fiscal Year Alignment**: The fiscal year aligns with the calendar year (January 1 - December 31). Custom fiscal year start dates are not supported.

- **Single User Context**: The budget feature is designed for single-user personal tracking. Multi-user or household budget scenarios are not supported.

- **Historical Data Retention**: The application retains purchase history for at least 5 years. Older data may be archived or deleted but is not required for this feature.

- **Real-time Updates**: Purchase data is immediately available for budget calculations. There is no batch processing delay between recording a purchase and seeing it reflected in budget views.

- **Default Budget Mode**: If not specified, the system defaults to monthly budget input mode rather than yearly.

- **Rollover Calculation Timing**: Rollover calculations occur at the month boundary (last day of month to first day of next month) based on server time.

- **Category Stability**: The predefined category list (Locomotives, Passenger Cars, etc.) is assumed to be stable. Dynamic category creation by users is not supported in the initial version.

- **Performance Threshold**: The application handles up to 1000 purchase transactions per year without performance degradation in budget calculations and visualizations.
