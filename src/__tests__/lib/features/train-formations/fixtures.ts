import type {
  FormationCategoryView,
  FormationElementView,
  PrototypeGroupView,
  PrototypeView,
  TrainFormationDetail,
  TrainFormationSummary
} from '$lib/bindings';

export function makeCategory(
  overrides: Partial<FormationCategoryView> = {}
): FormationCategoryView {
  return {
    id: 'trn:formation-category:eurocity',
    name: 'EuroCity',
    is_custom: false,
    ...overrides
  };
}

export function makePrototype(overrides: Partial<PrototypeView> = {}): PrototypeView {
  return {
    id: 'trn:prototype:sbb-re44-ii',
    railway_company_id: 'trn:railway-company:sbb-cff-ffs',
    company_name: 'SBB',
    series_code: 'Re 4/4 II',
    car_type: 'Locomotive',
    service_level: null,
    category: 'Passenger',
    is_motorized: true,
    default_is_dummy: false,
    is_custom: false,
    ...overrides
  };
}

export function makeElement(overrides: Partial<FormationElementView> = {}): FormationElementView {
  const prototype = makePrototype(overrides.prototype);

  return {
    id: 'trn:element:1',
    position_order: 0,
    prototype,
    owned_rolling_stock_id: null,
    snapshot_series_code: null,
    snapshot_company_name: null,
    stock_not_found: false,
    owned_count_for_prototype: 0,
    traction_override: 0,
    is_traction_slot: prototype.is_motorized && !prototype.default_is_dummy,
    ...overrides
  };
}

export function makeDetail(overrides: Partial<TrainFormationDetail> = {}): TrainFormationDetail {
  return {
    id: 'trn:formation:1',
    name: 'Gottardo 1974',
    category: makeCategory(),
    start_year: 1974,
    end_year: 1982,
    epoch: 'IV',
    notes: 'Classic consist',
    elements: [makeElement()],
    has_traction: true,
    ...overrides
  };
}

export function makeSummary(overrides: Partial<TrainFormationSummary> = {}): TrainFormationSummary {
  return {
    id: 'trn:formation:1',
    name: 'Gottardo 1974',
    category: makeCategory(),
    epoch: 'IV',
    element_count: 3,
    has_traction: true,
    owned_count: 2,
    planned_count: 1,
    ...overrides
  };
}

export function makePrototypeGroup(
  overrides: Partial<PrototypeGroupView> = {}
): PrototypeGroupView {
  return {
    railway_company_id: 'trn:railway-company:sbb-cff-ffs',
    company_name: 'SBB',
    prototypes: [makePrototype()],
    ...overrides
  };
}
