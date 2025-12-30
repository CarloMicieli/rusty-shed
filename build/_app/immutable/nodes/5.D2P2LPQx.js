import {
  L as Ce,
  p as ge,
  f as N,
  d as t,
  s as r,
  D as O,
  m as s,
  u as F,
  r as e,
  a6 as Ze,
  a7 as Ge,
  t as R,
  O as ue,
  e as c,
  a8 as He,
  b as g,
  c as be,
  E as oe,
  a as re,
  G as De,
  N as me,
  a9 as we,
  A as Le,
  n as K,
  k as ke,
  j as Me,
  a4 as qe,
  R as _e,
  l as Fe,
  a5 as Re,
  aa as Qe,
  V as he,
  ab as Xe,
  y as ze,
  o as Ie,
  a2 as Ye,
  ac as Je,
  ad as Ke,
  Z as We,
  B as Oe,
  h as $e,
  ae as et,
  Y as tt,
  $ as at,
  T as rt
} from '../chunks/C2P8ifMu.js';
import '../chunks/8AwhvTxk.js';
import { f as st } from '../chunks/B5syxjUO.js';
import {
  c as it,
  a as Ue,
  b as Ee,
  d as lt,
  e as nt,
  f as Ne,
  g as Pe,
  h as Ae,
  i as ct,
  j as ot,
  k as dt,
  l as vt
} from '../chunks/BUXTf0nH.js';
import { r as Se, t as je, c as q, a as Be } from '../chunks/DAlnca8K.js';
const ut = (S) => S;
function ft(S, { delay: a = 0, duration: b = 400, easing: i = ut } = {}) {
  const n = +getComputedStyle(S).opacity;
  return { delay: a, duration: b, easing: i, css: (d) => `opacity: ${d * n}` };
}
const _t = (S, a = we) => {
  const b = F(() => je(a()));
  var i = mt(),
    n = t(i);
  {
    var d = (m) => {
      var I = oe(),
        h = re(I);
      (De(
        h,
        () => s(b),
        (u, f) => {
          f(u, { size: 12 });
        }
      ),
        g(m, I));
    };
    O(n, (m) => {
      s(b) && m(d);
    });
  }
  var y = r(n);
  (e(i),
    R(
      (m, I) => {
        (ue(i, 1, m), c(y, ` ${I ?? ''}`));
      },
      [() => `badge ${Se(a()).variant}`, () => Se(a()).label() ?? a()]
    ),
    g(S, i));
};
var mt = N('<span><!> </span>'),
  gt = N('<p class="line-clamp-2 text-sm text-surface-300"> </p>'),
  bt = N('<div class="flex flex-wrap gap-2"></div>'),
  xt = N(
    '<article class="group hover:border-accent-500/60 rounded-xl border border-surface-700/60 bg-surface-900 p-4 shadow-lg shadow-surface-900/40 transition hover:-translate-y-1"><div><div class="absolute inset-0 bg-gradient-to-t from-surface-900/80 to-transparent"></div> <div class="absolute top-3 left-3 rounded-full bg-surface-900/60 p-2"><!></div></div> <div class="space-y-2"><div class="flex items-start justify-between gap-2"><div><p class="text-xs tracking-[0.18em] text-surface-500 uppercase"> </p> <h3 class="text-lg leading-tight font-semibold"> </h3></div> <div class="flex gap-2 opacity-0 transition group-hover:opacity-100"><button class="variant-soft-surface btn-icon btn btn-icon-sm"><!></button> <button class="variant-soft-error btn-icon btn btn-icon-sm"><!></button></div></div> <div class="flex flex-wrap gap-2 text-xs text-surface-400"><span class="variant-soft-surface badge"> </span> <span class="variant-soft-surface badge"> </span> <span class="variant-soft-surface badge"> </span></div> <!> <!></div></article>'
  );
function pt(S, a) {
  ge(a, !0);
  const b = F(() => a.item.tags?.[0] ?? 'default'),
    i = F(() => je(s(b)));
  function n() {
    a.onEdit?.(a.item);
  }
  function d() {
    a.onDelete?.(a.item.id);
  }
  var y = xt(),
    m = t(y),
    I = r(t(m), 2),
    h = t(I);
  {
    var u = (x) => {
      var k = oe(),
        A = re(k);
      (De(
        A,
        () => s(i),
        (L, te) => {
          te(L, { size: 20, class: 'text-accent-300' });
        }
      ),
        g(x, k));
    };
    O(h, (x) => {
      s(i) && x(u);
    });
  }
  (e(I), e(m));
  var f = r(m, 2),
    C = t(f),
    D = t(C),
    j = t(D),
    z = t(j);
  e(j);
  var B = r(j, 2),
    U = t(B, !0);
  (e(B), e(D));
  var P = r(D, 2),
    w = t(P);
  w.__click = n;
  var Y = t(w);
  (Ze(Y, { size: 16 }), e(w));
  var X = r(w, 2);
  X.__click = d;
  var J = t(X);
  (Ge(J, { size: 16 }), e(X), e(P), e(C));
  var V = r(C, 2),
    Z = t(V),
    se = t(Z, !0);
  e(Z);
  var G = r(Z, 2),
    $ = t(G, !0);
  e(G);
  var W = r(G, 2),
    ee = t(W, !0);
  (e(W), e(V));
  var _ = r(V, 2);
  {
    var T = (x) => {
      var k = gt(),
        A = t(k, !0);
      (e(k), R(() => c(A, a.item.description)), g(x, k));
    };
    O(_, (x) => {
      a.item.description && x(T);
    });
  }
  var E = r(_, 2);
  {
    var H = (x) => {
      var k = bt();
      (me(
        k,
        20,
        () => a.item.tags,
        (A) => A,
        (A, L) => {
          _t(A, () => L);
        }
      ),
        e(k),
        g(x, k));
    };
    O(E, (x) => {
      a.item.tags?.length && x(H);
    });
  }
  (e(f),
    e(y),
    R(
      (x, k) => {
        (ue(m, 1, x),
          c(z, `${a.item.brand ?? ''} • ${a.item.catalogNumber ?? ''}`),
          c(U, a.item.title),
          c(se, a.item.scale),
          c($, a.item.powerSystem),
          c(ee, k));
      },
      [
        () => `relative mb-3 h-32 overflow-hidden rounded-lg ${Se(s(b)).gradient}`,
        () => new Date(a.item.createdAt).toLocaleDateString()
      ]
    ),
    He(1, y, () => ft),
    g(S, y),
    be());
}
Ce(['click']);
var yt = N('<button> </button>'),
  ht = N('<button><!> <span> </span></button>'),
  wt = N(
    '<aside class="space-y-4 rounded-xl border border-surface-700/60 bg-surface-900 p-4"><div class="flex items-center justify-between"><h3 class="text-sm font-semibold tracking-wide text-surface-300 uppercase"> </h3> <button class="text-accent-400 hover:text-accent-300 text-xs"> </button></div> <div class="space-y-2"><p class="text-xs font-medium tracking-wide text-surface-400 uppercase"> </p> <input class="input-md input w-full bg-surface-900"/></div> <div class="space-y-2"><p class="text-xs font-medium tracking-wide text-surface-400 uppercase"> </p> <div class="flex flex-wrap gap-2"><button>All</button> <!></div></div> <div class="space-y-2"><p class="text-xs font-medium tracking-wide text-surface-400 uppercase"> </p> <div class="flex flex-wrap gap-2"></div></div></aside>'
  );
function kt(S, a) {
  ge(a, !0);
  const b = 300;
  let i = null,
    n = ke('');
  Le(() => {
    K(n, a.filters.query ?? '', !0);
  });
  function d(_) {
    (K(n, _, !0),
      i && clearTimeout(i),
      (i = setTimeout(() => {
        a.onSearch?.(_);
      }, b)));
  }
  function y(_) {
    a.onSetScale?.(_);
  }
  function m(_) {
    a.onToggleTag?.(_);
  }
  function I() {
    (K(n, ''), a.onClear?.());
  }
  var h = wt(),
    u = t(h),
    f = t(u),
    C = t(f, !0);
  e(f);
  var D = r(f, 2);
  D.__click = I;
  var j = t(D, !0);
  (e(D), e(u));
  var z = r(u, 2),
    B = t(z),
    U = t(B, !0);
  e(B);
  var P = r(B, 2);
  (_e(P), (P.__input = (_) => d(_.target.value)), e(z));
  var w = r(z, 2),
    Y = t(w),
    X = t(Y, !0);
  e(Y);
  var J = r(Y, 2),
    V = t(J);
  let Z;
  V.__click = () => y(null);
  var se = r(V, 2);
  (me(
    se,
    17,
    () => a.availableScales,
    (_) => _.id,
    (_, T) => {
      var E = yt();
      let H;
      E.__click = () => y(s(T).id);
      var x = t(E, !0);
      (e(E),
        R(() => {
          ((H = ue(E, 1, 'variant-soft-surface badge', null, H, {
            'variant-filled-primary': a.filters.scale === s(T).id
          })),
            c(x, s(T).display));
        }),
        g(_, E));
    }
  ),
    e(J),
    e(w));
  var G = r(w, 2),
    $ = t(G),
    W = t($, !0);
  e($);
  var ee = r($, 2);
  (me(
    ee,
    20,
    () => a.availableTags,
    (_) => _,
    (_, T) => {
      var E = oe(),
        H = re(E);
      {
        var x = (k) => {
          const A = F(() => je(T));
          var L = ht();
          let te;
          L.__click = () => m(T);
          var ie = t(L);
          {
            var xe = (l) => {
              var v = oe(),
                p = re(v);
              (De(
                p,
                () => s(A),
                (Q, le) => {
                  le(Q, { size: 14 });
                }
              ),
                g(l, v));
            };
            O(ie, (l) => {
              s(A) && l(xe);
            });
          }
          var o = r(ie, 2),
            de = t(o, !0);
          (e(o),
            e(L),
            R(
              (l, v, p) => {
                ((te = ue(L, 1, l, null, te, v)), c(de, p));
              },
              [
                () => `badge ${Se(T).variant}`,
                () => ({ 'variant-filled-primary': a.filters.tags.has(T) }),
                () => Se(T).label()
              ]
            ),
            g(k, L));
        };
        O(H, (k) => {
          T && k(x);
        });
      }
      g(_, E);
    }
  ),
    e(ee),
    e(G),
    e(h),
    R(
      (_, T, E, H, x, k) => {
        (c(C, _),
          c(j, T),
          c(U, E),
          Me(P, 'placeholder', H),
          qe(P, s(n)),
          c(X, x),
          (Z = ue(V, 1, 'variant-soft-surface badge', null, Z, {
            'variant-filled-primary': a.filters.scale === null
          })),
          c(W, k));
      },
      [() => it(), () => Ue(), () => Ee(), () => Ee(), () => lt(), () => nt()]
    ),
    g(S, h),
    be());
}
Ce(['click', 'input']);
var St = N('<option> </option>'),
  Ct = N(
    '<div class="fixed inset-0 z-50 flex justify-end bg-black/40" role="presentation" tabindex="-1"><div class="h-full w-full max-w-xl overflow-y-auto border-l border-surface-700/60 bg-surface-900 p-6 shadow-2xl" role="dialog" aria-modal="true" tabindex="-1"><div class="mb-4 flex items-center justify-between"><div><p class="text-xs tracking-[0.2em] text-surface-500 uppercase"> </p> <h3 class="text-xl font-semibold"> </h3></div> <button class="variant-ghost-surface btn-icon btn btn-icon-sm"><!></button></div> <div class="space-y-4"><label class="block space-y-1"><span class="text-sm text-surface-300">Brand</span> <input class="input w-full bg-surface-800"/></label> <label class="block space-y-1"><span class="text-sm text-surface-300">Catalog Number</span> <input class="input w-full bg-surface-800"/></label> <label class="block space-y-1"><span class="text-sm text-surface-300">Title</span> <input class="input w-full bg-surface-800"/></label> <div class="grid grid-cols-2 gap-3"><label class="block space-y-1"><span class="text-sm text-surface-300">Scale</span> <select class="input w-full bg-surface-800"></select></label> <label class="block space-y-1"><span class="text-sm text-surface-300">Power</span> <input class="input w-full bg-surface-800"/></label></div> <label class="block space-y-1"><span class="text-sm text-surface-300">Description</span> <textarea class="input w-full bg-surface-800" rows="3"></textarea></label> <label class="block space-y-1"><span class="text-sm text-surface-300">Tags (comma separated)</span> <input class="input w-full bg-surface-800"/></label></div> <div class="mt-6 flex justify-end gap-3"><button class="variant-ghost-surface btn">Cancel</button> <button class="variant-filled-primary btn"> </button></div></div></div>'
  );
function Dt(S, a) {
  ge(a, !0);
  const b = {
    brand: '',
    catalogNumber: '',
    title: '',
    scale: 'H0',
    powerSystem: 'DC',
    description: '',
    tags: []
  };
  let i = ke(Fe({ ...b }));
  Le(() => {
    a.open &&
      K(
        i,
        a.editing
          ? {
              brand: a.editing.brand,
              catalogNumber: a.editing.catalogNumber,
              title: a.editing.title,
              scale: a.editing.scale,
              powerSystem: a.editing.powerSystem,
              description: a.editing.description ?? '',
              tags: a.editing.tags ?? []
            }
          : { ...b },
        !0
      );
  });
  function n() {
    a.onClose?.();
  }
  function d() {
    a.onSubmit?.({ form: { ...s(i), tags: s(i).tags ?? [] }, editingId: a.editing?.id ?? null });
  }
  function y(u) {
    s(i).tags = u
      .split(',')
      .map((f) => f.trim())
      .filter(Boolean);
  }
  var m = oe(),
    I = re(m);
  {
    var h = (u) => {
      var f = Ct();
      ((f.__click = n), (f.__keydown = (o) => o.key === 'Escape' && n()));
      var C = t(f);
      ((C.__click = (o) => o.stopPropagation()),
        (C.__keydown = (o) => {
          o.key === 'Escape' && (o.stopPropagation(), n());
        }));
      var D = t(C),
        j = t(D),
        z = t(j),
        B = t(z, !0);
      e(z);
      var U = r(z, 2),
        P = t(U, !0);
      (e(U), e(j));
      var w = r(j, 2);
      w.__click = n;
      var Y = t(w);
      (Re(Y, { size: 16 }), e(w), e(D));
      var X = r(D, 2),
        J = t(X),
        V = r(t(J), 2);
      (_e(V), e(J));
      var Z = r(J, 2),
        se = r(t(Z), 2);
      (_e(se), e(Z));
      var G = r(Z, 2),
        $ = r(t(G), 2);
      (_e($), e(G));
      var W = r(G, 2),
        ee = t(W),
        _ = r(t(ee), 2);
      (me(
        _,
        21,
        () => a.availableScales,
        (o) => o.id,
        (o, de) => {
          var l = St(),
            v = t(l, !0);
          e(l);
          var p = {};
          (R(() => {
            (c(v, s(de).display), p !== (p = s(de).id) && (l.value = (l.__value = s(de).id) ?? ''));
          }),
            g(o, l));
        }
      ),
        e(_),
        e(ee));
      var T = r(ee, 2),
        E = r(t(T), 2);
      (_e(E), e(T), e(W));
      var H = r(W, 2),
        x = r(t(H), 2);
      (Qe(x), e(H));
      var k = r(H, 2),
        A = r(t(k), 2);
      (_e(A), (A.__input = (o) => y(o.target.value)), e(k), e(X));
      var L = r(X, 2),
        te = t(L);
      te.__click = n;
      var ie = r(te, 2);
      ie.__click = d;
      var xe = t(ie, !0);
      (e(ie),
        e(L),
        e(C),
        e(f),
        R(
          (o) => {
            (c(B, a.editing ? 'Edit item' : 'Add item'),
              c(P, a.editing ? a.editing.title : 'New item'),
              qe(A, o),
              c(xe, a.editing ? 'Save changes' : 'Add item'));
          },
          [() => s(i).tags.join(', ')]
        ),
        he(
          V,
          () => s(i).brand,
          (o) => (s(i).brand = o)
        ),
        he(
          se,
          () => s(i).catalogNumber,
          (o) => (s(i).catalogNumber = o)
        ),
        he(
          $,
          () => s(i).title,
          (o) => (s(i).title = o)
        ),
        Xe(
          _,
          () => s(i).scale,
          (o) => (s(i).scale = o)
        ),
        he(
          E,
          () => s(i).powerSystem,
          (o) => (s(i).powerSystem = o)
        ),
        he(
          x,
          () => s(i).description,
          (o) => (s(i).description = o)
        ),
        g(u, f));
    };
    O(I, (u) => {
      a.open && u(h);
    });
  }
  (g(S, m), be());
}
Ce(['click', 'keydown', 'input']);
var Tt = N(
  '<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60" role="presentation" tabindex="-1"><div class="w-full max-w-md rounded-xl border border-surface-700/70 bg-surface-900 p-6" role="dialog" aria-modal="true" tabindex="-1"><h3 class="text-lg font-semibold"> </h3> <p class="mt-2 text-sm text-surface-400"> </p> <div class="mt-5 flex justify-end gap-3"><button class="variant-ghost-surface btn">Cancel</button> <button class="variant-filled-error btn">Confirm</button></div></div></div>'
);
function It(S, a) {
  ge(a, !0);
  const b = ze(a, 'title', 3, 'Delete item'),
    i = ze(a, 'message', 3, 'Are you sure?');
  function n() {
    a.onClose?.();
  }
  function d() {
    a.onConfirm?.();
  }
  var y = oe(),
    m = re(y);
  {
    var I = (h) => {
      var u = Tt();
      ((u.__click = n), (u.__keydown = (w) => w.key === 'Escape' && n()));
      var f = t(u);
      ((f.__click = (w) => w.stopPropagation()),
        (f.__keydown = (w) => {
          w.key === 'Escape' && (w.stopPropagation(), n());
        }));
      var C = t(f),
        D = t(C, !0);
      e(C);
      var j = r(C, 2),
        z = t(j, !0);
      e(j);
      var B = r(j, 2),
        U = t(B);
      U.__click = n;
      var P = r(U, 2);
      ((P.__click = d),
        e(B),
        e(f),
        e(u),
        R(() => {
          (c(D, b()), c(z, i()));
        }),
        g(h, u));
    };
    O(m, (h) => {
      a.open && h(I);
    });
  }
  (g(S, y), be());
}
Ce(['click', 'keydown']);
const fe = (S, a = we, b = we, i = we, n = we) => {
  const d = F(() => n().replace('border-', 'text-').replace('500', '300'));
  var y = jt(),
    m = t(y),
    I = t(m);
  (De(I, i, (j, z) => {
    z(j, { size: 20 });
  }),
    e(m));
  var h = r(m, 2),
    u = t(h),
    f = t(u, !0);
  e(u);
  var C = r(u, 2),
    D = t(C, !0);
  (e(C),
    e(h),
    e(y),
    R(() => {
      (ue(
        y,
        1,
        `variant-soft-surface flex items-center justify-between gap-3 card border-l-4 p-4 ${n()}`
      ),
        ue(m, 1, `rounded-lg bg-surface-800/60 p-3 ${s(d)}`),
        c(f, a()),
        c(D, b()));
    }),
    g(S, y));
};
var jt = N(
    '<div><div><!></div> <div class="text-right"><p class="text-xs font-semibold tracking-wide text-surface-400 uppercase"> </p> <p class="text-xl font-bold text-surface-100"> </p></div></div>'
  ),
  zt = N(
    '<section class="space-y-4"><div class="variant-glass-surface flex flex-col gap-4 card p-4 sm:flex-row sm:items-center sm:justify-between"><div class="space-y-1"><p class="text-xs font-semibold tracking-[0.18em] text-surface-400 uppercase">Collection value</p> <p class="h3 font-bold text-primary-100"> </p></div> <div class="flex items-center gap-3 rounded-xl border border-primary-500/40 bg-primary-500/10 px-4 py-3"><div class="rounded-full bg-primary-500/20 p-3 text-primary-200"><!></div> <div><p class="text-xs font-semibold tracking-[0.12em] text-surface-400 uppercase">Total units</p> <p class="text-2xl font-bold text-surface-50"> </p></div></div></div> <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3"><!> <!> <!> <!> <!> <!></div></section>'
  );
function Et(S, a) {
  ge(a, !0);
  const b = F(
    () =>
      a.summary.locomotives_count +
      a.summary.passenger_cars_count +
      a.summary.freight_cars_count +
      a.summary.train_sets_count +
      a.summary.railcars_count +
      a.summary.electric_multiple_units_count
  );
  var i = zt(),
    n = t(i),
    d = t(n),
    y = r(t(d), 2),
    m = t(y, !0);
  (e(y), e(d));
  var I = r(d, 2),
    h = t(I),
    u = t(h);
  (Ie(u, { size: 22 }), e(h));
  var f = r(h, 2),
    C = r(t(f), 2),
    D = t(C, !0);
  (e(C), e(f), e(I), e(n));
  var j = r(n, 2),
    z = t(j);
  fe(
    z,
    () => 'Locomotives',
    () => a.summary.locomotives_count,
    () => Ie,
    () => 'border-primary-500'
  );
  var B = r(z, 2);
  fe(
    B,
    () => 'Passenger cars',
    () => a.summary.passenger_cars_count,
    () => Ie,
    () => 'border-info-500'
  );
  var U = r(B, 2);
  fe(
    U,
    () => 'Freight cars',
    () => a.summary.freight_cars_count,
    () => Ye,
    () => 'border-warning-500'
  );
  var P = r(U, 2);
  fe(
    P,
    () => 'Train sets',
    () => a.summary.train_sets_count,
    () => Je,
    () => 'border-secondary-500'
  );
  var w = r(P, 2);
  fe(
    w,
    () => 'Railcars',
    () => a.summary.railcars_count,
    () => Ke,
    () => 'border-success-500'
  );
  var Y = r(w, 2);
  (fe(
    Y,
    () => 'EMU',
    () => a.summary.electric_multiple_units_count,
    () => We,
    () => 'border-accent-500'
  ),
    e(j),
    e(i),
    R(() => {
      (c(m, a.totalValue), c(D, s(b)));
    }),
    g(S, i),
    be());
}
const Nt = (S) => {
  var a = At();
  (me(
    a,
    20,
    () => Array.from({ length: 6 }, (b, i) => i),
    (b) => b,
    (b, i) => {
      var n = Pt();
      (R(() => Me(n, 'aria-label', `loading-card-${i}`)), g(b, n));
    }
  ),
    e(a),
    g(S, a));
};
var Pt = N('<div class="h-56 animate-pulse rounded-xl bg-surface-800/80"></div>'),
  At = N('<div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3"></div>'),
  Bt = N(
    '<div class="flex flex-col items-center justify-center space-y-3 rounded-xl border border-dashed border-surface-700/60 bg-surface-900 p-10 text-center"><!> <h3 class="text-lg font-semibold"> </h3> <p class="text-sm text-surface-400"> </p> <button class="variant-filled-primary btn"> </button></div>'
  ),
  Lt = N(
    '<div class="flex flex-col items-center justify-center space-y-3 rounded-xl border border-dashed border-warning-500/40 bg-surface-900 p-8 text-center"><!> <h3 class="text-lg font-semibold"> </h3> <button class="variant-soft-warning btn"> </button></div>'
  ),
  Mt = N('<div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3"></div>'),
  qt = N(
    '<div class="space-y-6"><div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between"><div><p class="text-sm tracking-[0.2em] text-surface-400 uppercase"> </p> <h1 class="h2 font-bold"> </h1> <p class="text-sm text-surface-400"> </p></div> <div class="flex flex-col gap-3 md:flex-row md:items-center"><button class="variant-filled-primary btn gap-2"><!> </button></div></div> <!> <div class="grid gap-4 lg:grid-cols-[280px,1fr]"><!> <section class="space-y-4"><!></section></div></div> <!> <!>',
    1
  );
function Ft(S, a) {
  ge(a, !0);
  const b = (l) => {
      var v = Bt(),
        p = t(v);
      rt(p, { class: 'text-surface-500', size: 32 });
      var Q = r(p, 2),
        le = t(Q, !0);
      e(Q);
      var M = r(Q, 2),
        ve = t(M, !0);
      e(M);
      var ne = r(M, 2);
      ne.__click = function (...pe) {
        d.startCreate?.apply(this, pe);
      };
      var ae = t(ne, !0);
      (e(ne),
        e(v),
        R(
          (pe, ce, ye) => {
            (c(le, pe), c(ve, ce), c(ae, ye));
          },
          [() => dt(), () => Pe(), () => Ae()]
        ),
        g(l, v));
    },
    i = (l) => {
      var v = Lt(),
        p = t(v);
      Re(p, { class: 'text-warning-400', size: 28 });
      var Q = r(p, 2),
        le = t(Q, !0);
      e(Q);
      var M = r(Q, 2);
      M.__click = P;
      var ve = t(M, !0);
      (e(M),
        e(v),
        R(
          (ne, ae) => {
            (c(le, ne), c(ve, ae));
          },
          [() => vt(), () => Ue()]
        ),
        g(l, v));
    };
  function n() {
    let l = ke(!1),
      v = ke(null),
      p = ke(null);
    return {
      get showDrawer() {
        return s(l);
      },
      get editing() {
        return s(v);
      },
      get confirmDeleteId() {
        return s(p);
      },
      startCreate: () => {
        (K(v, null), K(l, !0));
      },
      edit: (ae) => {
        (K(v, ae, !0), K(l, !0));
      },
      closeDrawer: () => {
        (K(l, !1), K(v, null));
      },
      requestDelete: (ae) => {
        K(p, ae, !0);
      },
      clearDelete: () => {
        K(p, null);
      }
    };
  }
  const d = n(),
    y = Fe({
      locomotives_count: 0,
      passenger_cars_count: 0,
      freight_cars_count: 0,
      train_sets_count: 0,
      railcars_count: 0,
      electric_multiple_units_count: 0
    }),
    m = F(() => y),
    I = '--',
    h = F(() => q.rawItems),
    u = F(() => q.filteredItems),
    f = F(() => q.filters),
    C = F(() => q.availableTags),
    D = F(() => q.isLoading);
  Oe(() => {
    q.fetchCollection();
  });
  async function j(l) {
    const { form: v, editingId: p } = l;
    (p ? await q.updateItem({ id: p, ...v }) : await q.createItem(v), d.closeDrawer());
  }
  function z(l) {
    (q.setQuery(l), q.fetchCollection(l));
  }
  function B(l) {
    q.setScale(l);
  }
  function U(l) {
    q.toggleTag(l);
  }
  function P() {
    (q.clearFilters(), q.fetchCollection(''));
  }
  async function w() {
    d.confirmDeleteId && (await q.deleteItem(d.confirmDeleteId), d.clearDelete());
  }
  var Y = qt();
  $e('14ny0om', (l) => {
    tt(
      (v) => {
        at.title = v ?? '';
      },
      [() => Ne()]
    );
  });
  var X = re(Y),
    J = t(X),
    V = t(J),
    Z = t(V),
    se = t(Z, !0);
  e(Z);
  var G = r(Z, 2),
    $ = t(G, !0);
  e(G);
  var W = r(G, 2),
    ee = t(W, !0);
  (e(W), e(V));
  var _ = r(V, 2),
    T = t(_);
  T.__click = function (...l) {
    d.startCreate?.apply(this, l);
  };
  var E = t(T);
  et(E, { size: 18 });
  var H = r(E);
  (e(T), e(_), e(J));
  var x = r(J, 2);
  Et(x, {
    get summary() {
      return s(m);
    },
    totalValue: I
  });
  var k = r(x, 2),
    A = t(k);
  kt(A, {
    get filters() {
      return s(f);
    },
    get availableTags() {
      return s(C);
    },
    get availableScales() {
      return Be;
    },
    onSearch: z,
    onSetScale: B,
    onToggleTag: U,
    onClear: P
  });
  var L = r(A, 2),
    te = t(L);
  {
    var ie = (l) => {
        Nt(l);
      },
      xe = (l) => {
        var v = oe(),
          p = re(v);
        {
          var Q = (M) => {
              b(M);
            },
            le = (M) => {
              var ve = oe(),
                ne = re(ve);
              {
                var ae = (ce) => {
                    i(ce);
                  },
                  pe = (ce) => {
                    var ye = Mt();
                    (me(
                      ye,
                      21,
                      () => s(u),
                      (Te) => Te.id,
                      (Te, Ve) => {
                        pt(Te, {
                          get item() {
                            return s(Ve);
                          },
                          get onEdit() {
                            return d.edit;
                          },
                          get onDelete() {
                            return d.requestDelete;
                          }
                        });
                      }
                    ),
                      e(ye),
                      g(ce, ye));
                  };
                O(
                  ne,
                  (ce) => {
                    !s(D) && s(h).length > 0 && s(u).length === 0 ? ce(ae) : ce(pe, !1);
                  },
                  !0
                );
              }
              g(M, ve);
            };
          O(
            p,
            (M) => {
              !s(D) && s(h).length === 0 ? M(Q) : M(le, !1);
            },
            !0
          );
        }
        g(l, v);
      };
    O(te, (l) => {
      s(D) && s(h).length === 0 ? l(ie) : l(xe, !1);
    });
  }
  (e(L), e(k), e(X));
  var o = r(X, 2);
  Dt(o, {
    get open() {
      return d.showDrawer;
    },
    get editing() {
      return d.editing;
    },
    get availableScales() {
      return Be;
    },
    get onClose() {
      return d.closeDrawer;
    },
    onSubmit: j
  });
  var de = r(o, 2);
  {
    let l = F(() => !!d.confirmDeleteId),
      v = F(() => ot()),
      p = F(() => ct());
    It(de, {
      get open() {
        return s(l);
      },
      get title() {
        return s(v);
      },
      get message() {
        return s(p);
      },
      get onClose() {
        return d.clearDelete;
      },
      onConfirm: w
    });
  }
  (R(
    (l, v, p, Q) => {
      (c(se, l), c($, v), c(ee, p), c(H, ` ${Q ?? ''}`));
    },
    [() => st(), () => Ne(), () => Pe(), () => Ae()]
  ),
    g(S, Y),
    be());
}
Ce(['click']);
function Ht(S) {
  Ft(S, {});
}
export { Ht as component };
