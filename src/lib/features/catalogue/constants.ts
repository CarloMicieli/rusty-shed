import type { ConstantItem as BaseConstantItem } from '$lib/types/constant_item';

export type ConstantItem = BaseConstantItem;

export const formLabels: Record<string, ConstantItem> = {
  title: { id: 'add-new-railway-model', labelKey: 'form_new_model_title' },
  basicInfo: { id: 'basic-information', labelKey: 'form_new_model_basic_info' },
  manufacturer: { id: 'manufacturer', labelKey: 'form_new_model_manufacturer' },
  productCode: { id: 'product-code', labelKey: 'form_new_model_product_code' },
  productCodePlaceholder: {
    id: 'product-code-placeholder',
    labelKey: 'form_new_model_product_code_placeholder'
  },
  description: { id: 'description', labelKey: 'form_new_model_description' },
  descriptionPlaceholder: {
    id: 'description-placeholder',
    labelKey: 'form_new_model_description_placeholder'
  },
  category: { id: 'category', labelKey: 'form_new_model_category' },
  scale: { id: 'scale', labelKey: 'form_new_model_scale' },
  powerMethod: { id: 'power-method', labelKey: 'form_new_model_power_method' },
  epoch: { id: 'epoch', labelKey: 'form_new_model_epoch' },
  selectPlaceholder: { id: 'select-placeholder', labelKey: 'form_new_model_select_placeholder' },
  deliveryAvailability: {
    id: 'delivery-availability',
    labelKey: 'form_new_model_delivery_availability'
  },
  deliveryDate: { id: 'delivery-date', labelKey: 'form_new_model_delivery_date' },
  deliveryDatePlaceholder: {
    id: 'delivery-date-placeholder',
    labelKey: 'form_new_model_delivery_date_placeholder'
  },
  availabilityStatus: {
    id: 'availability-status',
    labelKey: 'form_new_model_availability_status'
  },
  additionalDetails: { id: 'additional-details', labelKey: 'form_new_model_additional_details' },
  detailsPlaceholder: {
    id: 'details-placeholder',
    labelKey: 'form_new_model_details_placeholder'
  },
  rollingStock: { id: 'rolling-stock', labelKey: 'form_new_model_rolling_stock' },
  railwayCompany: { id: 'railway-company', labelKey: 'form_new_model_railway_company' },
  rollingStockCategory: {
    id: 'rolling-stock-category',
    labelKey: 'form_new_model_rolling_stock_category'
  },
  livery: { id: 'livery', labelKey: 'form_new_model_livery' },
  liveryPlaceholder: { id: 'livery-placeholder', labelKey: 'form_new_model_livery_placeholder' },
  className: { id: 'class-name', labelKey: 'form_new_model_class_name' },
  roadNumber: { id: 'road-number', labelKey: 'form_new_model_road_number' },
  series: { id: 'series', labelKey: 'form_new_model_series' },
  depot: { id: 'depot', labelKey: 'form_new_model_depot' },
  type: { id: 'type', labelKey: 'form_new_model_type' },
  typeName: { id: 'type-name', labelKey: 'form_new_model_type_name' },
  passengerCarType: { id: 'passenger-car-type', labelKey: 'form_new_model_passenger_car_type' },
  freightCarType: { id: 'freight-car-type', labelKey: 'form_new_model_freight_car_type' },
  emuType: { id: 'emu-type', labelKey: 'form_new_model_emu_type' },
  isDummy: { id: 'is-dummy', labelKey: 'form_new_model_is_dummy' },
  technicalDetails: { id: 'technical-details', labelKey: 'form_new_model_technical_details' },
  control: { id: 'control', labelKey: 'form_new_model_control' },
  dccInterface: { id: 'dcc-interface', labelKey: 'form_new_model_dcc_interface' },
  serviceLevel: { id: 'service-level', labelKey: 'form_new_model_service_level' },
  duplicate: { id: 'duplicate', labelKey: 'form_new_model_duplicate' },
  delete: { id: 'delete', labelKey: 'form_new_model_delete' },
  addRollingStock: { id: 'add-rolling-stock', labelKey: 'form_new_model_add_rolling_stock' },
  create: { id: 'create-railway-model', labelKey: 'form_new_model_create' },
  cancel: { id: 'cancel', labelKey: 'form_new_model_cancel' }
};
