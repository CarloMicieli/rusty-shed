import { _ as g } from './PPVm8Dsz.js';
import { k as f, l as S, u as v, m as i, n as r } from './C2P8ifMu.js';
import { t as y } from './B6BiQSwf.js';
import './8AwhvTxk.js';
import { c as W, a as w, b as k, d as L } from './Cxbe8PBw.js';
async function l(p, t) {
  const { invoke: s } = await g(
    async () => {
      const { invoke: e } = await import('./mPlcS5K-.js');
      return { invoke: e };
    },
    [],
    import.meta.url
  );
  return s(p, t ?? {});
}
function u() {
  return typeof crypto < 'u' && crypto.randomUUID
    ? crypto.randomUUID()
    : Math.random().toString(36).slice(2);
}
function _(p) {
  y.loading({ id: p, title: k(), duration: 4e3 });
}
function m(p) {
  y.success({ id: p, title: L(), duration: 2e3 });
}
function d(p, t) {
  y.error({ id: p, title: w(), duration: 5e3, action: t ? { label: W(), onClick: t } : void 0 });
}
class b {
  #t = f(S([]));
  #s = f(S({}));
  #i = f(null);
  #o = f(!1);
  #r = null;
  #a = v(() => i(this.#t).find((t) => t.is_default) ?? null);
  #n = v(() => (i(this.#i) ? (i(this.#t).find((t) => t.id === i(this.#i)) ?? null) : null));
  #h = v(() => (i(this.#i) ? (i(this.#s)[i(this.#i)] ?? []) : []));
  get wishlists() {
    return i(this.#t);
  }
  get itemsByWishlist() {
    return i(this.#s);
  }
  get activeWishlistId() {
    return i(this.#i);
  }
  get isLoading() {
    return i(this.#o);
  }
  get defaultWishlist() {
    return i(this.#a);
  }
  get activeWishlist() {
    return i(this.#n);
  }
  get wishlistItems() {
    return i(this.#h);
  }
  #e() {
    this.#r = {
      wishlists: structuredClone(i(this.#t)),
      itemsByWishlist: structuredClone(i(this.#s)),
      activeWishlistId: i(this.#i)
    };
  }
  revertSnapshot() {
    this.#r &&
      (r(this.#t, this.#r.wishlists, !0),
      r(this.#s, this.#r.itemsByWishlist, !0),
      r(this.#i, this.#r.activeWishlistId, !0));
  }
  async fetchWishlists() {
    r(this.#o, !0);
    try {
      const t = await l('get_wishlists');
      r(this.#t, t ?? [], !0);
      const s = i(this.#t).find((e) => e.is_default);
      !i(this.#i) && s && r(this.#i, s.id, !0);
    } catch (t) {
      (console.error(t), d(u()));
    } finally {
      r(this.#o, !1);
    }
  }
  async loadWishlistItems(t) {
    try {
      const s = await l('get_wishlist_by_id', { id: t });
      let e = null;
      (s && typeof s == 'object' && 'status' in s
        ? (e = s.status === 'ok' ? s.data : null)
        : (e = s),
        r(this.#s, { ...i(this.#s), [t]: e?.items ?? [] }, !0));
    } catch (s) {
      (console.error(s), d(u()));
    }
  }
  async selectWishlist(t) {
    (r(this.#i, t, !0), i(this.#s)[t] || (await this.loadWishlistItems(t)));
  }
  async createWishlist(t, s = !1) {
    const e = u();
    this.#e();
    const o = `temp-${e}`,
      n = {
        id: o,
        name: t,
        notes: null,
        is_default: s,
        count: 0,
        updated_at: new Date().toISOString(),
        total_value: {}
      },
      h = s ? i(this.#t).map((c) => ({ ...c, is_default: !1 })) : i(this.#t);
    (r(this.#t, [...h, n], !0), s && r(this.#i, o), _(e));
    try {
      const c = await l('create_wishlist', { input: { name: t, notes: null, is_default: s } });
      return (
        r(
          this.#t,
          i(this.#t).map((a) => (a.id === o ? c : a)),
          !0
        ),
        c.is_default && r(this.#i, c.id, !0),
        m(e),
        c
      );
    } catch (c) {
      return (
        console.error(c),
        this.revertSnapshot(),
        d(e, () => {
          (this.revertSnapshot(), this.createWishlist(t, s));
        }),
        null
      );
    }
  }
  async renameWishlist(t, s) {
    const e = u();
    (this.#e(),
      r(
        this.#t,
        i(this.#t).map((o) => (o.id === t ? { ...o, name: s } : o)),
        !0
      ),
      _(e));
    try {
      (await l('rename_wishlist', { input: { id: t, name: s } }), m(e));
    } catch (o) {
      (console.error(o),
        this.revertSnapshot(),
        d(e, () => {
          (this.revertSnapshot(), this.renameWishlist(t, s));
        }));
    }
  }
  async deleteWishlist(t) {
    const s = u();
    (this.#e(),
      r(
        this.#t,
        i(this.#t).filter((o) => o.id !== t),
        !0
      ));
    const e = { ...i(this.#s) };
    (delete e[t], r(this.#s, e, !0), i(this.#i) === t && r(this.#i, null), _(s));
    try {
      (await l('delete_wishlist', { id: t }), m(s));
    } catch (o) {
      (console.error(o),
        this.revertSnapshot(),
        d(s, () => {
          (this.revertSnapshot(), this.deleteWishlist(t));
        }));
    }
  }
  async setDefaultWishlist(t) {
    const s = u();
    (this.#e(),
      r(
        this.#t,
        i(this.#t).map((e) => ({ ...e, is_default: e.id === t })),
        !0
      ),
      r(this.#i, t, !0),
      _(s));
    try {
      (await l('set_default_wishlist', { id: t }), m(s));
    } catch (e) {
      (console.error(e),
        this.revertSnapshot(),
        d(s, () => {
          (this.revertSnapshot(), this.setDefaultWishlist(t));
        }));
    }
  }
  async addItem(t, s) {
    const e = u();
    this.#e();
    const o = {
        id: `temp-${e}`,
        railway_model_id: s,
        priority: 'NORMAL',
        status: 'WANTED',
        added_date: new Date().toISOString().slice(0, 10),
        removed_date: null,
        notes: null,
        desired_price: null,
        purchased_price: null
      },
      n = i(this.#s)[t] ?? [];
    (r(this.#s, { ...i(this.#s), [t]: [...n, o] }, !0),
      r(
        this.#t,
        i(this.#t).map((h) => (h.id === t ? { ...h, count: h.count + 1 } : h)),
        !0
      ),
      _(e));
    try {
      const h = await l('add_to_wishlist', { input: { wishlist_id: t, railway_model_id: s } }),
        c = i(this.#s)[t] ?? [];
      return (
        r(this.#s, { ...i(this.#s), [t]: c.map((a) => (a.id === o.id ? h : a)) }, !0),
        m(e),
        h
      );
    } catch (h) {
      return (
        console.error(h),
        this.revertSnapshot(),
        d(e, () => {
          (this.revertSnapshot(), this.addItem(t, s));
        }),
        null
      );
    }
  }
  async removeItem(t, s) {
    const e = u();
    this.#e();
    const o = i(this.#s)[t] ?? [];
    (r(this.#s, { ...i(this.#s), [t]: o.filter((n) => n.id !== s) }, !0),
      r(
        this.#t,
        i(this.#t).map((n) => (n.id === t ? { ...n, count: Math.max(0, n.count - 1) } : n)),
        !0
      ),
      _(e));
    try {
      (await l('remove_from_wishlist', { item_id: s }), m(e));
    } catch (n) {
      (console.error(n),
        this.revertSnapshot(),
        d(e, () => {
          (this.revertSnapshot(), this.removeItem(t, s));
        }));
    }
  }
  async moveItemToList(t, s, e) {
    const o = u();
    this.#e();
    const n = i(this.#s)[s] ?? [],
      h = i(this.#s)[e] ?? [],
      c = n.find((a) => a.id === t);
    if (c) {
      (r(this.#s, { ...i(this.#s), [s]: n.filter((a) => a.id !== t), [e]: [...h, c] }, !0),
        r(
          this.#t,
          i(this.#t).map((a) =>
            a.id === s
              ? { ...a, count: Math.max(0, a.count - 1) }
              : a.id === e
                ? { ...a, count: a.count + 1 }
                : a
          ),
          !0
        ),
        _(o));
      try {
        (await l('move_item_to_list', { input: { item_id: t, destination_wishlist_id: e } }), m(o));
      } catch (a) {
        (console.error(a),
          this.revertSnapshot(),
          d(o, () => {
            (this.revertSnapshot(), this.moveItemToList(t, s, e));
          }));
      }
    }
  }
}
const E = new b();
export { E as w };
