# Mobile Redesign Contract Mapping

## Scope

Maps feature 043 US3 mobile behavior to the contract in specs/043-mobile-redesign/contracts/mobile-redesign.openapi.yaml.

## Mapping

| Contract Operation       | UI/State Implementation                                                                      | Verification                                                                     |
| ------------------------ | -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| getMobileLayoutProfile   | src/lib/state/match-media.svelte.ts and responsive gates in route/component layout           | drawer and route tests with viewport mocks                                       |
| openMobileDrawerLayer    | src/lib/state/drawer-registry.svelte.ts openParent/openChild                                 | src/**tests**/state/drawer-registry.svelte.test.ts                               |
| closeMobileDrawerLayer   | src/lib/state/drawer-registry.svelte.ts closeTop/closeById                                   | src/**tests**/routes/mobile-back-stack.test.ts                                   |
| getMobileMediaCapability | camera probe in src/lib/components/model-details/ImageUpload.svelte and ImageDropZone.svelte | src/**tests**/lib/components/model-details/ImageUpload.mobile-capability.test.ts |
| attachMobileMedia        | ImageCropDialog flows invoked from ImageUpload/ImageDropZone with fallback notice            | existing ImageUpload/ImageDropZone tests + contract test                         |

## Notes

- Drawer depth remains bounded to parent + child only.
- Camera fallback degrades to gallery picker without dropping current pending image state.
- Contract conformance checks are centralized in src/**tests**/contracts/mobile-redesign.contract.test.ts.
