// Auto-generated from messages/en.json
// This mock allows tests to safely call all Paraglide message functions
// without needing the full compiled Paraglide runtime

export function createMessagesMock() {
  // Create a Proxy that returns a function for any message key
  return new Proxy({}, {
    get: (_target: any, prop: string | symbol) => {
      // Return a function that returns the key name for debugging
      return () => String(prop);
    }
  } as ProxyHandler<object>);
}

// Export individual message functions as a fallback
export const mockMessageFunctions = () => ({
  // Core message keys that might be accessed
  app_name: () => 'Rusty Shed',
  app_collection: () => 'Collection',
  add_model_section_rolling_stock: () => 'Rolling Stocks',
  add_model_section_purchase: () => 'Purchase Information',
  add_model_empty_rolling_stocks_title: () => 'No rolling stocks added',
  add_model_empty_rolling_stocks_subtitle: () => 'Add individual units...',
  add_model_add_first_rolling_stock: () => 'Add your first rolling stock',
  add_model_add_rolling_stock: () => 'Add rolling stock',
  add_model_remove_rolling_stock: () => 'Remove',
  add_model_rolling_stock_items_label: () => 'Items',
  add_model_section_model: () => 'Railway Model',
  add_model_railway_company: () => 'Railway Company',
  add_model_series_code: () => 'Series Code',
  add_model_road_number: () => 'Road Number',
  add_model_rs_category: () => 'Category',
  add_model_seller: () => 'Seller',
  add_model_purchase_type: () => 'Purchase Type',
  add_model_purchase_type_standard: () => 'Standard',
  add_model_purchase_type_preorder: () => 'Preorder',
  add_model_purchase_date: () => 'Purchase Date',
  add_model_preorder_date: () => 'Expected Date',
  add_model_expected_date: () => 'Expected Date',
  add_model_price: () => 'Price',
  add_model_deposit_amount: () => 'Deposit Amount',
  add_model_preorder_total: () => 'Preorder Total',
  add_model_purchase_condition: () => 'Purchase Condition',
  add_model_model_condition: () => 'Model Condition',
  add_model_box_condition: () => 'Box Condition',
  add_model_notes: () => 'Notes',
  add_model_notes_placeholder: () => 'Add any notes...',
  add_model_remaining_balance: () => 'Remaining Balance',
  placeholder_amount: () => 'Amount',
  form_new_model_select_placeholder: () => '-- Select --',
  wishlist_modal_manufacturer: () => 'Manufacturer',
  action_delete: () => 'Delete',
  enum_category_locomotives: () => 'Locomotives',
  enum_category_passenger_cars: () => 'Passenger Cars',
  enum_category_freight_cars: () => 'Freight Cars',
  enum_category_railcars: () => 'Railcars',
  enum_category_electric_multiple_units: () => 'Electric Multiple Units',
  enum_category_train_sets: () => 'Train Sets',
  rolling_stock_field_company: () => 'Company',
  rolling_stock_field_series: () => 'Series',
  rolling_stock_field_category: () => 'Category',
  rolling_stock_field_road: () => 'Road Number'
});
