import { k as f, v, w as S, m as n, n as c, x as g, l as b, u as y } from './C2P8ifMu.js';
import { t as I } from './B6BiQSwf.js';
import './8AwhvTxk.js';
import { c as C, a as T, b as L, d as x } from './Cxbe8PBw.js';
import { c as D, a as E, b as z, d as A, e as K, s as N } from './B5syxjUO.js';
import { r as O } from './DxQretAU.js';
import { commands as m } from './zzNGAeR9.js';
const k = {
    steam: {
      key: 'steam',
      label: () => K(),
      variant: 'variant-filled-primary',
      gradient: 'bg-gradient-to-br from-primary-500/20 via-surface-700 to-surface-900',
      iconKey: 'steam'
    },
    diesel: {
      key: 'diesel',
      label: () => A(),
      variant: 'variant-filled-secondary',
      gradient: 'bg-gradient-to-br from-secondary-500/20 via-surface-700 to-surface-900',
      iconKey: 'diesel'
    },
    electric: {
      key: 'electric',
      label: () => z(),
      variant: 'variant-filled-tertiary',
      gradient: 'bg-gradient-to-br from-tertiary-500/20 via-surface-700 to-surface-900',
      iconKey: 'electric'
    },
    passenger: {
      key: 'passenger',
      label: () => E(),
      variant: 'variant-soft-surface',
      gradient: 'bg-gradient-to-br from-accent-500/15 via-surface-700 to-surface-900',
      iconKey: 'passenger'
    },
    freight: {
      key: 'freight',
      label: () => D(),
      variant: 'variant-soft-surface',
      gradient: 'bg-gradient-to-br from-warning-500/15 via-surface-700 to-surface-900',
      iconKey: 'freight'
    }
  },
  $ = {
    key: 'default',
    label: () => 'Tag',
    variant: 'variant-soft-surface',
    gradient: 'bg-gradient-to-br from-surface-600 to-surface-800',
    iconKey: 'default'
  };
function M(o) {
  const e = o.toLowerCase();
  return k[e] ?? { ...$, key: o };
}
function U(o) {
  const t = ['steam', 'diesel', 'electric', 'passenger', 'freight'].filter((s) =>
      o.map((a) => a.toLowerCase()).includes(s)
    ),
    r = o.filter((s) => !t.includes(s.toLowerCase())).sort((s, a) => s.localeCompare(a));
  return [...t, ...r];
}
function R(o) {
  const e = M(o);
  return O(e.iconKey);
}
var q = ['forEach', 'isDisjointFrom', 'isSubsetOf', 'isSupersetOf'],
  F = ['difference', 'intersection', 'symmetricDifference', 'union'],
  _ = !1;
class l extends Set {
  #s = new Map();
  #e = f(0);
  #t = f(0);
  #r = v || -1;
  constructor(e) {
    if ((super(), e)) {
      for (var t of e) super.add(t);
      this.#t.v = super.size;
    }
    _ || this.#i();
  }
  #a(e) {
    return v === this.#r ? f(e) : S(e);
  }
  #i() {
    _ = !0;
    var e = l.prototype,
      t = Set.prototype;
    for (const r of q)
      e[r] = function (...s) {
        return (n(this.#e), t[r].apply(this, s));
      };
    for (const r of F)
      e[r] = function (...s) {
        n(this.#e);
        var a = t[r].apply(this, s);
        return new l(a);
      };
  }
  has(e) {
    var t = super.has(e),
      r = this.#s,
      s = r.get(e);
    if (s === void 0) {
      if (!t) return (n(this.#e), !1);
      ((s = this.#a(!0)), r.set(e, s));
    }
    return (n(s), t);
  }
  add(e) {
    return (super.has(e) || (super.add(e), c(this.#t, super.size), g(this.#e)), this);
  }
  delete(e) {
    var t = super.delete(e),
      r = this.#s,
      s = r.get(e);
    return (s !== void 0 && (r.delete(e), c(s, !1)), t && (c(this.#t, super.size), g(this.#e)), t);
  }
  clear() {
    if (super.size !== 0) {
      super.clear();
      var e = this.#s;
      for (var t of e.values()) c(t, !1);
      (e.clear(), c(this.#t, 0), g(this.#e));
    }
  }
  keys() {
    return this.values();
  }
  values() {
    return (n(this.#e), super.values());
  }
  entries() {
    return (n(this.#e), super.entries());
  }
  [Symbol.iterator]() {
    return this.keys();
  }
  get size() {
    return n(this.#t);
  }
}
const V = N;
function u() {
  return typeof crypto < 'u' && crypto.randomUUID
    ? crypto.randomUUID()
    : Math.random().toString(36).slice(2);
}
function p(o) {
  I.loading({ id: o, title: L(), duration: 4e3 });
}
function w(o) {
  I.success({ id: o, title: x(), duration: 2e3 });
}
function h(o, e) {
  I.error({ id: o, title: T(), duration: 5e3, action: e ? { label: C(), onClick: e } : void 0 });
}
class j {
  #s = f(b([]));
  get rawItems() {
    return n(this.#s);
  }
  set rawItems(e) {
    c(this.#s, e, !0);
  }
  #e = f(b({ query: '', scale: null, tags: new l() }));
  get filters() {
    return n(this.#e);
  }
  set filters(e) {
    c(this.#e, e, !0);
  }
  #t = f(!1);
  get isLoading() {
    return n(this.#t);
  }
  set isLoading(e) {
    c(this.#t, e, !0);
  }
  #r = y(() => {
    const e = new l();
    this.rawItems.forEach((r) => r.tags?.forEach((s) => e.add(s)));
    const t = new l([...Object.keys(k), ...e]);
    return U([...t]);
  });
  get availableTags() {
    return n(this.#r);
  }
  set availableTags(e) {
    c(this.#r, e);
  }
  #a = y(() => {
    const { query: e, scale: t, tags: r } = this.filters,
      s = e.trim().toLowerCase();
    return this.rawItems.filter(
      (a) =>
        !(
          (t && a.scale !== t) ||
          (r.size && !a.tags.some((d) => r.has(d))) ||
          (s &&
            !`${a.brand} ${a.catalogNumber} ${a.title} ${a.description ?? ''} ${a.tags.join(' ')}`
              .toLowerCase()
              .includes(s))
        )
    );
  });
  get filteredItems() {
    return n(this.#a);
  }
  set filteredItems(e) {
    c(this.#a, e);
  }
  #i = y(() => this.rawItems.length);
  get totalCount() {
    return n(this.#i);
  }
  set totalCount(e) {
    c(this.#i, e);
  }
  fetchCollection = async (e) => {
    ((this.isLoading = !0), e !== void 0 && (this.filters.query = e));
    try {
      const t = await m.listCollectionItems(e ?? null);
      t.status === 'ok' ? (this.rawItems = t.data ?? []) : h(u());
    } catch (t) {
      (console.error(t), h(u()));
    } finally {
      this.isLoading = !1;
    }
  };
  setQuery = (e) => {
    this.filters.query = e;
  };
  toggleTag = (e) => {
    const t = new l(this.filters.tags);
    (t.has(e) ? t.delete(e) : t.add(e), (this.filters.tags = t));
  };
  setScale = (e) => {
    this.filters.scale = e;
  };
  clearFilters = () => {
    this.filters = { query: '', scale: null, tags: new l() };
  };
  createItem = async (e) => {
    const t = u(),
      r = [...this.rawItems],
      s = {
        id: `temp-${t}`,
        createdAt: new Date().toISOString(),
        description: e.description ?? null,
        tags: e.tags ?? [],
        brand: e.brand,
        catalogNumber: e.catalogNumber,
        title: e.title,
        scale: e.scale,
        powerSystem: e.powerSystem
      };
    ((this.rawItems = [...this.rawItems, s]), p(t));
    try {
      const a = await m.createCollectionItem({
        brand: e.brand,
        catalogNumber: e.catalogNumber,
        title: e.title,
        scale: e.scale,
        powerSystem: e.powerSystem,
        description: e.description ?? null,
        tags: e.tags ?? []
      });
      if (a.status === 'ok') {
        const i = a.data;
        return ((this.rawItems = this.rawItems.map((d) => (d.id === s.id ? i : d))), w(t), i);
      }
      throw a.error;
    } catch (a) {
      return (
        console.error(a),
        (this.rawItems = r),
        h(t, () => {
          ((this.rawItems = r), this.createItem(e));
        }),
        null
      );
    }
  };
  updateItem = async (e) => {
    const t = u(),
      r = [...this.rawItems],
      s = this.rawItems.find((i) => i.id === e.id);
    if (!s) return null;
    const a = {
      ...s,
      brand: e.brand,
      catalogNumber: e.catalogNumber,
      title: e.title,
      scale: e.scale,
      powerSystem: e.powerSystem,
      description: e.description ?? null,
      tags: e.tags ?? []
    };
    ((this.rawItems = this.rawItems.map((i) => (i.id === e.id ? a : i))), p(t));
    try {
      const i = await m.updateCollectionItem(e);
      if (i.status === 'ok')
        return (
          (this.rawItems = this.rawItems.map((d) => (d.id === e.id ? i.data : d))),
          w(t),
          i.data
        );
      throw i.error;
    } catch (i) {
      return (
        console.error(i),
        (this.rawItems = r),
        h(t, () => {
          this.updateItem(e);
        }),
        null
      );
    }
  };
  deleteItem = async (e) => {
    const t = u(),
      r = [...this.rawItems];
    ((this.rawItems = this.rawItems.filter((s) => s.id !== e)), p(t));
    try {
      const s = await m.deleteCollectionItem(e);
      if (s.status === 'ok') return (w(t), !0);
      throw s.error;
    } catch (s) {
      return (
        console.error(s),
        (this.rawItems = r),
        h(t, () => {
          this.deleteItem(e);
        }),
        !1
      );
    }
  };
}
const W = new j();
export { V as a, W as c, M as r, R as t };
