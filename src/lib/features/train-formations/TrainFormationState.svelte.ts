/**
 * Train Formation feature state — Svelte 5 reactive class.
 *
 * Manages the list of formations and the currently active detail view.
 * All mutations go through the service layer and update local state
 * optimistically where possible.
 */

import { setContext, getContext } from 'svelte';
import { toaster } from '$lib/toaster';
import * as m from '$lib/paraglide/messages.js';
import type {
  TrainFormationSummary,
  TrainFormationDetail,
  FormationElementView,
  FormationCategoryView,
  PrototypeGroupView,
  PrototypeView,
  CreateTrainFormationArgs,
  UpdateTrainFormationArgs,
  AddFormationElementArgs,
  ReorderFormationElementsArgs,
  AssignRollingStockToElementArgs,
  SetTractionOverrideArgs,
  CreateCustomPrototypeArgs
} from '$lib/bindings';
import { getErrorMessage } from '$lib/services';
import * as svc from './services/formations.service.js';
import { hasTraction } from './domain/traction.js';

const CONTEXT_KEY = Symbol('train-formations');

export class TrainFormationState {
  #summaries = $state<TrainFormationSummary[]>([]);
  #detail = $state<TrainFormationDetail | null>(null);
  #categories = $state<FormationCategoryView[]>([]);
  #prototypeGroups = $state<PrototypeGroupView[]>([]);
  #isLoading = $state(false);
  #isDetailLoading = $state(false);
  #isPrototypesLoading = $state(false);

  // ── Derived ──────────────────────────────────────────────────────────────

  get summaries(): TrainFormationSummary[] {
    return this.#summaries;
  }

  get detail(): TrainFormationDetail | null {
    return this.#detail;
  }

  get categories(): FormationCategoryView[] {
    return this.#categories;
  }

  get prototypeGroups(): PrototypeGroupView[] {
    return this.#prototypeGroups;
  }

  get isLoading(): boolean {
    return this.#isLoading;
  }

  get isDetailLoading(): boolean {
    return this.#isDetailLoading;
  }

  get isPrototypesLoading(): boolean {
    return this.#isPrototypesLoading;
  }

  /** Whether the current detail view has at least one traction slot. */
  hasTraction = $derived(hasTraction(this.#detail?.elements ?? []));

  // ── List operations ───────────────────────────────────────────────────────

  async load(): Promise<void> {
    this.#isLoading = true;
    const result = await svc.getTrainFormations();
    this.#isLoading = false;
    if (result.ok) {
      this.#summaries = result.data;
    } else {
      toaster.error(getErrorMessage(result.error));
    }
  }

  async create(args: CreateTrainFormationArgs): Promise<string | null> {
    const result = await svc.createTrainFormation(args);
    if (result.ok) {
      await this.load();
      return result.data.id;
    }
    toaster.error(getErrorMessage(result.error));
    return null;
  }

  async update(id: string, args: UpdateTrainFormationArgs): Promise<boolean> {
    const result = await svc.updateTrainFormation(id, args);
    if (result.ok) {
      await this.load();
      if (this.#detail?.id === id) {
        await this.loadDetail(id);
      }
      return true;
    }
    toaster.error(getErrorMessage(result.error));
    return false;
  }

  async delete(id: string): Promise<boolean> {
    const result = await svc.deleteTrainFormation(id);
    if (result.ok) {
      this.#summaries = this.#summaries.filter((s) => s.id !== id);
      if (this.#detail?.id === id) {
        this.#detail = null;
      }
      return true;
    }
    toaster.error(getErrorMessage(result.error));
    return false;
  }

  // ── Detail operations ─────────────────────────────────────────────────────

  async loadDetail(id: string): Promise<void> {
    this.#isDetailLoading = true;
    const result = await svc.getTrainFormation(id);
    this.#isDetailLoading = false;
    if (result.ok) {
      this.#detail = result.data;
    } else {
      toaster.error(getErrorMessage(result.error));
    }
  }

  // ── Element operations ────────────────────────────────────────────────────

  async addElement(formationId: string, args: AddFormationElementArgs): Promise<boolean> {
    const result = await svc.addFormationElement(formationId, args);
    if (result.ok) {
      if (this.#detail?.id === formationId) {
        await this.loadDetail(formationId);
      }
      return true;
    }
    toaster.error(getErrorMessage(result.error));
    return false;
  }

  async removeElement(elementId: string): Promise<boolean> {
    const formationId = this.#detail?.id;
    const result = await svc.removeFormationElement(elementId);
    if (result.ok) {
      if (formationId) {
        // Optimistic update: remove from local list immediately
        if (this.#detail) {
          this.#detail = {
            ...this.#detail,
            elements: this.#detail.elements.filter((e) => e.id !== elementId)
          };
        }
        toaster.success(m.formations_element_removed());
      }
      return true;
    }
    toaster.error(getErrorMessage(result.error));
    return false;
  }

  async reorderElements(
    formationId: string,
    orderedElements: FormationElementView[]
  ): Promise<void> {
    // Apply optimistic local state
    if (this.#detail?.id === formationId) {
      this.#detail = { ...this.#detail, elements: orderedElements };
    }

    const args: ReorderFormationElementsArgs = {
      element_ids: orderedElements.map((e) => e.id)
    };
    const result = await svc.reorderFormationElements(formationId, args);
    if (!result.ok) {
      // Revert by reloading
      await this.loadDetail(formationId);
      toaster.error(getErrorMessage(result.error));
    }
  }

  // ── Category operations ───────────────────────────────────────────────────

  async loadCategories(): Promise<void> {
    const result = await svc.getFormationCategories();
    if (result.ok) {
      this.#categories = result.data;
    }
  }

  async createCategory(name: string): Promise<FormationCategoryView | null> {
    const result = await svc.createFormationCategory({ name });
    if (result.ok) {
      this.#categories = [...this.#categories, result.data];
      return result.data;
    }
    toaster.error(getErrorMessage(result.error));
    return null;
  }

  // ── Prototype search ──────────────────────────────────────────────────────

  async searchPrototypes(query: string): Promise<void> {
    this.#isPrototypesLoading = true;
    const result = await svc.getPrototypes(query || null);
    this.#isPrototypesLoading = false;
    if (result.ok) {
      this.#prototypeGroups = result.data;
    }
  }

  async createCustomPrototype(args: CreateCustomPrototypeArgs): Promise<PrototypeView | null> {
    const result = await svc.createCustomPrototype(args);
    if (result.ok) {
      return result.data;
    }
    toaster.error(getErrorMessage(result.error));
    return null;
  }

  // ── Ownership ─────────────────────────────────────────────────────────────

  async assignRollingStock(
    elementId: string,
    args: AssignRollingStockToElementArgs
  ): Promise<boolean> {
    const formationId = this.#detail?.id;
    const result = await svc.assignRollingStockToElement(elementId, args);
    if (result.ok) {
      if (this.#detail && formationId) {
        this.#detail = {
          ...this.#detail,
          elements: this.#detail.elements.map((e) => (e.id === elementId ? result.data : e))
        };
      }
      return true;
    }
    toaster.error(getErrorMessage(result.error));
    return false;
  }

  /** Quick-assign the single matching model (for ownedCount === 1 slots). */
  async quickAssign(elementId: string): Promise<void> {
    // Find the element and trigger assign with null so the backend picks the single match
    // The backend resolves the single owned_rolling_stock when assignment resolves
    await this.assignRollingStock(elementId, { owned_rolling_stock_id: null });
  }

  // ── Traction override ─────────────────────────────────────────────────────

  async setTractionOverride(elementId: string, override: number): Promise<void> {
    const args: SetTractionOverrideArgs = { traction_override: override };
    const result = await svc.setTractionOverride(elementId, args);
    if (result.ok && this.#detail) {
      this.#detail = {
        ...this.#detail,
        elements: this.#detail.elements.map((e) => (e.id === elementId ? result.data : e))
      };
    } else if (!result.ok) {
      toaster.error(getErrorMessage(result.error));
    }
  }

  // ── Context helpers ───────────────────────────────────────────────────────

  static setContext(): TrainFormationState {
    const state = new TrainFormationState();
    setContext(CONTEXT_KEY, state);
    return state;
  }

  static getContext(): TrainFormationState {
    return getContext<TrainFormationState>(CONTEXT_KEY);
  }
}
