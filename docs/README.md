# Requirements: Rusty Shed (Model Railway Manager)

## 🚂 Project Overview

**Rusty Shed** is a personal digital logbook for model railway enthusiasts. It allows a single user to track what they own, what they want (Wish Lists), and how they maintain their fleet.

The app is **local-first**, meaning your data stays on your computer, but can be exported for safekeeping.

---

## 📂 How Data is Organized

To keep things consistent, the app follows a specific hierarchy.

### 1. The Collection Item (RailwayModel)

This represents the "Product" you purchased (e.g., a box from a hobby shop).

- **Mandatory:** Brand (e.g., ACME), Description, Scale (e.g., H0), Catalog Number, and Power Method (DC/AC).
- **Optional:** details, and personal notes.
- **Purchase Info:** Includes date, price, currency (ISO codes like EUR/USD), and the seller name.

### 2. The Physical Units (RollingStock)

Every "Collection Item" contains one or more physical units.

- **Identification:** Road number, Series, and Railway company (e.g., FS).
- **Classification:** Category (Locomotive, Passenger Car, etc.) and Epoch (e.g., IV or V).
- **Technical Details:** Physical length (stored in **mm**), Livery colors, and DCC interface type.
- **Control:** Tracks if it is "DCC Ready" or has a "Decoder Installed."

### 3. Maintenance Logs

You can record "Service Events" for any specific unit:

- What was done (Description), who did it, and the cost.
- The date of service and a "Next Due" reminder date.

### 4. Wish Lists

Keep track of future purchases.

- Create multiple named lists.
- Add items by catalog number and rank them by priority.

---

## 🛠 Functional Features (What it does)

1. **Smart Search:** Quickly find models by brand, era (Epoch), or whether they are DCC-capable.
2. **Inventory Management:** Edit quantities, move items between "Serviceable" and "Under Repair."
3. **Financial Reporting:** Automatically calculate the total value of your collection based on purchase history.
4. **Data Portability:** Manual backup and restore via file export/import.

---

## 📐 System Rules (The "Must-Haves")

- **Precision:** All measurements use millimeters; all prices use standard currency codes to avoid rounding errors.
- **Unique Identity:** Every single entry (Model, Unit, Maintenance Event) is assigned a unique "UUID" (a long, unique ID string) so data never gets mixed up.
- **Reliability:** The app must remain fast even with thousands of items in the database.
- **Integrity:** You cannot have a "Collection Item" without at least one "Physical Unit" inside it.

---

## 💻 Technical Strategy (Internal Instructions)

- **Backend (Rust):** Use `thiserror` for custom error types (e.g., `DatabaseError`, `ValidationError`).
- **Frontend (Svelte 5):**
- **Service Layer:** All Tauri `invoke` calls must be wrapped in a `service.ts` file.
- **State Management:** Use Class-based **Runes** to handle Loading and Error states.
- **Localization:** Use **Paraglide** to map Rust error codes to user-friendly, localized messages.
