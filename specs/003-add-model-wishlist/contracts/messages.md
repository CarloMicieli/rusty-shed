# Message Keys Contract: Add Railway Model to Wishlist

**Feature**: 003-add-model-wishlist  
**Date**: 2026-01-30

## New Paraglide Message Keys

Add to `messages/en.json` and `messages/it.json`:

### English (en.json)

```json
{
  "wishlist_add_model_button": "Add railway model",
  "wishlist_drawer_title": "Add Railway Model",
  "wishlist_drawer_subtitle": "Create a new railway model and add it to your wishlist",
  "wishlist_field_wishlist": "Wishlist",
  "wishlist_field_manufacturer": "Manufacturer",
  "wishlist_field_product_code": "Product Code",
  "wishlist_field_description": "Description",
  "wishlist_field_category": "Category",
  "wishlist_field_scale": "Scale",
  "wishlist_field_power_method": "Power Method",
  "wishlist_field_epoch": "Epoch",
  "wishlist_field_desired_price": "Desired Price",
  "wishlist_field_priority": "Priority",
  "wishlist_field_notes": "Notes",
  "wishlist_rolling_stocks_title": "Rolling Stocks",
  "wishlist_rolling_stock_add": "Add rolling stock",
  "wishlist_rolling_stock_remove": "Remove",
  "wishlist_field_railway_company": "Railway Company",
  "wishlist_field_series_code": "Series Code",
  "wishlist_field_road_number": "Road Number (optional)",
  "wishlist_drawer_submit": "Add to Wishlist",
  "wishlist_drawer_cancel": "Cancel",
  "wishlist_priority_low": "Low",
  "wishlist_priority_normal": "Normal",
  "wishlist_priority_high": "High",
  "wishlist_category_locomotives": "Locomotives",
  "wishlist_category_train_sets": "Train Sets",
  "wishlist_category_starter_sets": "Starter Sets",
  "wishlist_category_freight_cars": "Freight Cars",
  "wishlist_category_passenger_cars": "Passenger Cars",
  "wishlist_category_electric_multiple_units": "Electric Multiple Units",
  "wishlist_category_railcars": "Railcars",
  "wishlist_power_ac": "AC",
  "wishlist_power_dc": "DC",
  "wishlist_power_trix_express": "Trix Express",
  "wishlist_validation_required": "This field is required",
  "wishlist_validation_select_wishlist": "Please select a wishlist",
  "wishlist_validation_rolling_stock_incomplete": "Please complete all rolling stock fields",
  "wishlist_toast_adding": "Adding railway model...",
  "wishlist_toast_success": "Railway model added to wishlist",
  "wishlist_toast_error": "Failed to add railway model",
  "wishlist_no_wishlists": "Create a wishlist first",
  "wishlist_loading_data": "Loading..."
}
```

### Italian (it.json)

```json
{
  "wishlist_add_model_button": "Aggiungi modello ferroviario",
  "wishlist_drawer_title": "Aggiungi Modello Ferroviario",
  "wishlist_drawer_subtitle": "Crea un nuovo modello ferroviario e aggiungilo alla tua lista dei desideri",
  "wishlist_field_wishlist": "Lista dei desideri",
  "wishlist_field_manufacturer": "Produttore",
  "wishlist_field_product_code": "Codice Prodotto",
  "wishlist_field_description": "Descrizione",
  "wishlist_field_category": "Categoria",
  "wishlist_field_scale": "Scala",
  "wishlist_field_power_method": "Alimentazione",
  "wishlist_field_epoch": "Epoca",
  "wishlist_field_desired_price": "Prezzo Desiderato",
  "wishlist_field_priority": "Priorità",
  "wishlist_field_notes": "Note",
  "wishlist_rolling_stocks_title": "Rotabili",
  "wishlist_rolling_stock_add": "Aggiungi rotabile",
  "wishlist_rolling_stock_remove": "Rimuovi",
  "wishlist_field_railway_company": "Compagnia Ferroviaria",
  "wishlist_field_series_code": "Codice Serie",
  "wishlist_field_road_number": "Numero di Servizio (opzionale)",
  "wishlist_drawer_submit": "Aggiungi alla Lista",
  "wishlist_drawer_cancel": "Annulla",
  "wishlist_priority_low": "Bassa",
  "wishlist_priority_normal": "Normale",
  "wishlist_priority_high": "Alta",
  "wishlist_category_locomotives": "Locomotive",
  "wishlist_category_train_sets": "Treni Completi",
  "wishlist_category_starter_sets": "Set Iniziali",
  "wishlist_category_freight_cars": "Carri Merci",
  "wishlist_category_passenger_cars": "Carrozze Passeggeri",
  "wishlist_category_electric_multiple_units": "Elettrotreni",
  "wishlist_category_railcars": "Automotrici",
  "wishlist_power_ac": "CA",
  "wishlist_power_dc": "CC",
  "wishlist_power_trix_express": "Trix Express",
  "wishlist_validation_required": "Questo campo è obbligatorio",
  "wishlist_validation_select_wishlist": "Seleziona una lista dei desideri",
  "wishlist_validation_rolling_stock_incomplete": "Completa tutti i campi del rotabile",
  "wishlist_toast_adding": "Aggiunta modello ferroviario...",
  "wishlist_toast_success": "Modello ferroviario aggiunto alla lista",
  "wishlist_toast_error": "Impossibile aggiungere il modello ferroviario",
  "wishlist_no_wishlists": "Crea prima una lista dei desideri",
  "wishlist_loading_data": "Caricamento..."
}
```

## Usage Pattern

```svelte
<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
</script>

<label>{m.wishlist_field_manufacturer()}</label>
```

## After Adding Keys

Run to regenerate Paraglide types:

```bash
pnpm prepare
```

This generates `src/paraglide/messages.js` with typed functions for each key.
