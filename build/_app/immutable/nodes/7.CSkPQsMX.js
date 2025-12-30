import {
  L as $,
  p as X,
  f as y,
  s as h,
  S as ue,
  D as N,
  b as f,
  c as Y,
  d as a,
  M as ne,
  r as s,
  E as K,
  a as O,
  N as Z,
  m as t,
  H as ee,
  t as P,
  O as le,
  e as H,
  A as _e,
  n as C,
  k as re,
  Q as he,
  R as me,
  U as be,
  V as ge,
  l as xe,
  W as we,
  X as ye,
  B as pe,
  h as ke,
  Y as Ie,
  u as F,
  $ as We
} from '../chunks/C2P8ifMu.js';
import '../chunks/8AwhvTxk.js';
import { a as Se } from '../chunks/DCbz5KVu.js';
import { w as p } from '../chunks/BFM2npA3.js';
import { r as Te } from '../chunks/DxQretAU.js';
var De = y('<p class="text-sm text-surface-400">Wishlist is empty</p>'),
  Re = y('<span class="variant-soft-primary badge text-[10px] uppercase">Default</span>'),
  Me = y(
    '<div role="button" tabindex="0"><div class="flex items-center gap-2"><!> <div class="flex flex-col"><span class="font-semibold"> </span> <span class="text-xs text-surface-400"> </span></div></div> <div class="flex items-center gap-2"><!> <button class="btn-icon btn btn-icon-sm" type="button">✕</button></div></div>'
  ),
  ze = y(
    '<aside class="space-y-4 rounded-2xl border border-surface-700/50 bg-surface-900 p-4"><div class="flex items-center justify-between"><h2 class="h5 font-semibold tracking-tight">Wishlists</h2> <button class="variant-soft-primary btn btn-sm"><!> <span>Create New List</span></button></div> <div class="space-y-2"><!></div></aside>'
  );
function Ce(S, e) {
  X(e, !0);
  var d = ze(),
    n = a(d),
    b = h(a(n), 2);
  b.__click = () => e.onCreate?.();
  var E = a(b);
  (ue(E, { size: 16 }), ne(2), s(b), s(n));
  var T = h(n, 2),
    M = a(T);
  {
    var j = (l) => {
        var i = De();
        f(l, i);
      },
      L = (l) => {
        var i = K(),
          u = O(i);
        (Z(
          u,
          17,
          () => e.wishlists,
          (g) => g.id,
          (g, c) => {
            var r = Me();
            let k;
            ((r.__click = () => e.onSelect?.(t(c).id)),
              (r.__keydown = (W) => W.key === 'Enter' && e.onSelect?.(t(c).id)));
            var x = a(r),
              w = a(x);
            ee(w, { size: 16 });
            var D = h(w, 2),
              o = a(D),
              _ = a(o, !0);
            s(o);
            var v = h(o, 2),
              I = a(v);
            (s(v), s(D), s(x));
            var z = h(x, 2),
              Q = a(z);
            {
              var U = (W) => {
                var V = Re();
                f(W, V);
              };
              N(Q, (W) => {
                t(c).is_default && W(U);
              });
            }
            var q = h(Q, 2);
            ((q.__click = (W) => {
              (W.stopPropagation(), e.onDelete?.(t(c).id));
            }),
              s(z),
              s(r),
              P(() => {
                ((k = le(r, 1, 'btn w-full justify-between gap-3 text-left', null, k, {
                  'variant-filled-primary': t(c).id === e.activeId,
                  'variant-ghost-surface': t(c).id !== e.activeId
                })),
                  H(_, t(c).name),
                  H(I, `${t(c).count ?? ''} items`));
              }),
              f(g, r));
          }
        ),
          f(l, i));
      };
    N(M, (l) => {
      e.wishlists.length === 0 ? l(j) : l(L, !1);
    });
  }
  (s(T), s(d), f(S, d), Y());
}
$(['click', 'keydown']);
var Ee = y('<input class="input-lg variant-ghost-surface input"/>'),
  je = y('<button type="button" class="text-left h4 font-bold"> </button>'),
  Le = y(
    '<div class="flex flex-wrap items-center justify-between gap-3"><div class="flex items-center gap-2"><!> <!></div> <div class="flex items-center gap-2"><button><!> <span>Set as Default</span></button></div></div>'
  ),
  He = y(
    '<div class="rounded-xl border border-dashed border-surface-700/60 p-8 text-center text-surface-400">Wishlist is empty</div>'
  );
function Ne(S, e) {
  X(e, !0);
  let d = re(!1),
    n = re('');
  _e(() => {
    e.wishlist && (t(d) || C(n, e.wishlist.name, !0));
  });
  async function b() {
    e.wishlist &&
      (t(n).trim() && t(n) !== e.wishlist.name
        ? e.onRename?.(t(n).trim())
        : C(n, e.wishlist.name, !0),
      C(d, !1));
  }
  function E(l) {
    (l.key === 'Enter' && b(),
      l.key === 'Escape' && (C(d, !1), e.wishlist && C(n, e.wishlist.name, !0)));
  }
  var T = K(),
    M = O(T);
  {
    var j = (l) => {
        var i = Le(),
          u = a(i),
          g = a(u);
        ee(g, { class: 'text-accent-500', size: 20 });
        var c = h(g, 2);
        {
          var r = (_) => {
              var v = Ee();
              (me(v),
                (v.__keydown = E),
                be('blur', v, b),
                ge(
                  v,
                  () => t(n),
                  (I) => C(n, I)
                ),
                f(_, v));
            },
            k = (_) => {
              var v = je();
              ((v.__click = () => C(d, !0)), (v.__keydown = (z) => z.key === 'Enter' && C(d, !0)));
              var I = a(v, !0);
              (s(v), P(() => H(I, t(n))), f(_, v));
            };
          N(c, (_) => {
            t(d) ? _(r) : _(k, !1);
          });
        }
        s(u);
        var x = h(u, 2),
          w = a(x);
        let D;
        w.__click = () => e.onSetDefault?.();
        var o = a(w);
        (he(o, { size: 16 }),
          ne(2),
          s(w),
          s(x),
          s(i),
          P(
            () =>
              (D = le(w, 1, 'variant-soft-primary btn', null, D, {
                'variant-filled-primary': e.wishlist.is_default
              }))
          ),
          f(l, i));
      },
      L = (l) => {
        var i = He();
        f(l, i);
      };
    N(M, (l) => {
      e.wishlist ? l(j) : l(L, !1);
    });
  }
  (f(S, T), Y());
}
$(['keydown', 'click']);
var Be = y(
    '<div class="col-span-full rounded-xl border border-dashed border-surface-700/60 p-6 text-center text-surface-400">Wishlist is empty</div>'
  ),
  Ae = y('<option disabled selected>Create another list to move</option>'),
  Fe = y('<option> </option>'),
  Ke = y(
    '<div class="rounded-xl border border-surface-700/50 bg-surface-800 p-4 shadow-sm"><div class="mb-3 flex items-center gap-2"><!> <span class="text-sm font-semibold"> </span></div> <div class="flex items-center justify-between text-xs text-surface-400"><span> </span> <span> </span></div> <div class="mt-3 flex items-center gap-2"><button class="variant-ghost-surface btn btn-sm">Delete</button> <div class="flex flex-1 items-center gap-2"><select class="select-sm variant-ghost-surface select w-full"><!></select> <button class="variant-soft-primary btn btn-sm">Move</button></div></div></div>'
  );
function Oe(S, e) {
  X(e, !0);
  const d = xe({}),
    b = Te('heart');
  function E(i, u) {
    d[i] = u;
  }
  function T(i) {
    const u = e.activeWishlistId;
    if (!u) return;
    const g = d[i] ?? e.otherTargets[0]?.id;
    g && e.onMove?.({ itemId: i, fromId: u, toId: g });
  }
  var M = K(),
    j = O(M);
  {
    var L = (i) => {
        var u = Be();
        f(i, u);
      },
      l = (i) => {
        var u = K(),
          g = O(u);
        (Z(
          g,
          17,
          () => e.items,
          (c) => c.id,
          (c, r) => {
            var k = Ke(),
              x = a(k),
              w = a(x);
            {
              var D = (m) => {
                  b(m, { size: 16, class: 'text-accent-400' });
                },
                o = (m) => {
                  ee(m, { size: 16, class: 'text-accent-400' });
                };
              N(w, (m) => {
                b ? m(D) : m(o, !1);
              });
            }
            var _ = h(w, 2),
              v = a(_, !0);
            (s(_), s(x));
            var I = h(x, 2),
              z = a(I),
              Q = a(z, !0);
            s(z);
            var U = h(z, 2),
              q = a(U, !0);
            (s(U), s(I));
            var W = h(I, 2),
              V = a(W);
            V.__click = () =>
              e.activeWishlistId &&
              e.onRemove?.({ itemId: t(r).id, wishlistId: e.activeWishlistId });
            var te = h(V, 2),
              R = a(te);
            R.__change = (m) => E(t(r).id, m.currentTarget.value);
            var oe = a(R);
            {
              var ve = (m) => {
                  var B = Ae();
                  ((B.value = B.__value = ''), f(m, B));
                },
                de = (m) => {
                  var B = K(),
                    ce = O(B);
                  (Z(
                    ce,
                    17,
                    () => e.otherTargets,
                    (G) => G.id,
                    (G, J) => {
                      var A = Fe(),
                        fe = a(A, !0);
                      s(A);
                      var ie = {};
                      (P(() => {
                        (H(fe, t(J).name),
                          ie !== (ie = t(J).id) && (A.value = (A.__value = t(J).id) ?? ''));
                      }),
                        f(G, A));
                    }
                  ),
                    f(m, B));
                };
              N(oe, (m) => {
                e.otherTargets.length === 0 ? m(ve) : m(de, !1);
              });
            }
            s(R);
            var ae;
            we(R);
            var se = h(R, 2);
            ((se.__click = () => T(t(r).id)),
              s(te),
              s(W),
              s(k),
              P(() => {
                (H(v, t(r).railway_model_id),
                  H(Q, t(r).status),
                  H(q, t(r).priority),
                  (R.disabled = e.otherTargets.length === 0),
                  ae !== (ae = d[t(r).id] ?? e.otherTargets[0]?.id ?? '') &&
                    ((R.value = (R.__value = d[t(r).id] ?? e.otherTargets[0]?.id ?? '') ?? ''),
                    ye(R, d[t(r).id] ?? e.otherTargets[0]?.id ?? '')),
                  (se.disabled = e.otherTargets.length === 0));
              }),
              f(c, k));
          }
        ),
          f(i, u));
      };
    N(j, (i) => {
      e.items.length === 0 ? i(L) : i(l, !1);
    });
  }
  (f(S, M), Y());
}
$(['click', 'change']);
var Pe = y(
  '<div class="grid gap-6 lg:grid-cols-[320px,1fr]"><!> <section class="space-y-4 rounded-2xl border border-surface-700/50 bg-surface-900 p-6"><!> <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"><!></div></section></div>'
);
function Qe(S, e) {
  X(e, !0);
  const d = F(() => p.wishlists),
    n = F(() => p.activeWishlist),
    b = F(() => p.activeWishlistId),
    E = F(() => p.wishlistItems),
    T = F(() => t(d).filter((o) => o.id !== t(b)));
  pe(() => {
    p.fetchWishlists();
  });
  function M() {
    p.createWishlist('Create New List');
  }
  function j(o) {
    p.selectWishlist(o);
  }
  function L(o) {
    p.deleteWishlist(o);
  }
  function l(o) {
    t(n) && p.renameWishlist(t(n).id, o);
  }
  function i() {
    t(n) && p.setDefaultWishlist(t(n).id);
  }
  function u(o) {
    const { itemId: _, wishlistId: v } = o;
    p.removeItem(v, _);
  }
  function g(o) {
    const { itemId: _, fromId: v, toId: I } = o;
    p.moveItemToList(_, v, I);
  }
  var c = Pe();
  ke('5lvve0', (o) => {
    Ie(
      (_) => {
        We.title = _ ?? '';
      },
      [() => Se()]
    );
  });
  var r = a(c);
  Ce(r, {
    get wishlists() {
      return t(d);
    },
    get activeId() {
      return t(b);
    },
    onCreate: M,
    onSelect: j,
    onDelete: L
  });
  var k = h(r, 2),
    x = a(k);
  Ne(x, {
    get wishlist() {
      return t(n);
    },
    onRename: l,
    onSetDefault: i
  });
  var w = h(x, 2),
    D = a(w);
  (Oe(D, {
    get items() {
      return t(E);
    },
    get activeWishlistId() {
      return t(b);
    },
    get otherTargets() {
      return t(T);
    },
    onRemove: u,
    onMove: g
  }),
    s(w),
    s(k),
    s(c),
    f(S, c),
    Y());
}
function Ge(S) {
  Qe(S, {});
}
export { Ge as component };
