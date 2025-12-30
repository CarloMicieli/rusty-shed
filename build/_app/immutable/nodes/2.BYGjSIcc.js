const __vite__mapDeps = (
  i,
  m = __vite__mapDeps,
  d = m.f || (m.f = ['../chunks/zzNGAeR9.js', '../chunks/mPlcS5K-.js'])
) => i.map((i) => d[i]);
import {
  p as fa,
  f,
  d as t,
  r as a,
  s as e,
  D as M,
  t as z,
  e as o,
  O as Fa,
  b as l,
  c as ma,
  af as Ha,
  E as xa,
  a as ea,
  ag as Na,
  ah as Oa,
  L as Va,
  y as Ma,
  N as ra,
  m as r,
  G as Ua,
  u as $,
  M as qa,
  j as oa,
  o as Ga,
  ai as Ja,
  aj as Sa,
  P as Qa,
  ae as Pa,
  ak as Ka,
  A as Ya,
  n as j,
  J as Xa,
  R as Ra,
  aa as Za,
  k as ia,
  al as $a,
  ab as at,
  V as Ea,
  B as tt,
  h as et,
  Y as rt,
  $ as st,
  am as it,
  an as ot,
  H as dt
} from '../chunks/C2P8ifMu.js';
import { g as Da } from '../chunks/D0U87bhI.js';
import { r as ja } from '../chunks/Cas8_-DH.js';
import '../chunks/8AwhvTxk.js';
import { a as lt, b as nt } from '../chunks/fcFX1gix.js';
import {
  d as vt,
  a as ct,
  b as _t,
  c as ut,
  e as ft,
  f as mt,
  g as ht,
  h as bt,
  w as pt,
  i as gt,
  j as xt,
  k as yt,
  l as wt,
  m as kt,
  n as jt,
  o as Ct,
  p as It,
  q as Lt,
  r as Dt,
  s as St,
  t as Vt,
  u as zt,
  v as At,
  x as Et,
  y as Mt,
  z as Pt,
  A as Rt,
  B as Tt,
  C as Wt,
  D as Bt,
  E as Ft,
  F as Ht,
  G as Nt,
  H as Ot,
  I as Ta,
  J as Ut,
  K as qt,
  L as Gt,
  M as Jt,
  N as Qt,
  O as Kt,
  P as Yt,
  Q as Xt
} from '../chunks/DdIlMJnB.js';
import { d as Zt } from '../chunks/DCW-6SkD.js';
import { w as ga } from '../chunks/BFM2npA3.js';
import { _ as Wa } from '../chunks/PPVm8Dsz.js';
import { t as $t } from '../chunks/B6BiQSwf.js';
var ae = f(
  '<div class="variant-filled-surface hover:variant-filled-secondary space-y-2 card p-4 transition-colors duration-200"><div class="flex items-center justify-between text-surface-400"><span class="text-sm font-bold tracking-widest uppercase"> </span> <!></div> <div class="flex items-end gap-2"><h3 class="h3 font-bold text-primary-500"> </h3> <span> </span></div></div>'
);
function te(E, s) {
  fa(s, !0);
  var d = ae(),
    _ = t(d),
    g = t(_),
    y = t(g, !0);
  a(g);
  var L = e(g, 2);
  {
    var C = (c) => {
        Ha(c, { size: 16, class: 'text-success-500' });
      },
      x = (c) => {
        var k = xa(),
          D = ea(k);
        {
          var T = (i) => {
              Na(i, { size: 16, class: 'text-error-500' });
            },
            b = (i) => {
              Oa(i, { size: 16, class: 'text-surface-400' });
            };
          M(
            D,
            (i) => {
              s.stat.trend === 'down' ? i(T) : i(b, !1);
            },
            !0
          );
        }
        l(c, k);
      };
    M(L, (c) => {
      s.stat.trend === 'up' ? c(C) : c(x, !1);
    });
  }
  a(_);
  var w = e(_, 2),
    m = t(w),
    A = t(m, !0);
  a(m);
  var u = e(m, 2),
    v = t(u, !0);
  (a(u),
    a(w),
    a(d),
    z(() => {
      (o(y, s.stat.label),
        o(A, s.stat.value),
        Fa(
          u,
          1,
          `${s.stat.trend === 'up' ? 'text-success-500' : s.stat.trend === 'down' ? 'text-error-500' : 'text-surface-400'} text-sm font-medium`
        ),
        o(v, s.stat.trendValue));
    }),
    l(E, d),
    ma());
}
var ee = f(
    '<button type="button" class="variant-ghost-surface hover:variant-filled-primary group btn justify-start border border-surface-700/50 p-4 transition-all duration-200"><!> <span class="group-hover:text-on-primary font-semibold tracking-wide uppercase"> </span></button>'
  ),
  re = f('<div class="grid grid-cols-1 gap-2 sm:grid-cols-3"></div>');
function se(E, s) {
  let d = Ma(s, 'actions', 19, () => []);
  var _ = re();
  (ra(
    _,
    21,
    d,
    (g) => g.id,
    (g, y) => {
      const L = $(() => r(y).icon);
      var C = ee();
      C.__click = () => r(y).onClick?.();
      var x = t(C);
      Ua(
        x,
        () => r(L),
        (A, u) => {
          u(A, {
            class:
              'text-accent-500 group-hover:text-on-primary mr-3 transition-transform group-hover:scale-110',
            size: 20
          });
        }
      );
      var w = e(x, 2),
        m = t(w, !0);
      (a(w), a(C), z(() => o(m, r(y).label)), l(g, C));
    }
  ),
    a(_),
    l(E, _));
}
Va(['click']);
var ie = f(
    '<img class="h-full w-full object-cover transition-transform duration-500 group-hover:scale-105"/> <div class="from-background/90 absolute inset-0 bg-gradient-to-t to-transparent"></div>',
    1
  ),
  oe = f(
    '<div class="absolute inset-0 flex items-center justify-center bg-gradient-to-br from-surface-800 to-surface-700 text-2xl font-semibold text-surface-200"> </div>'
  ),
  de = f('<p class="text-xs tracking-wider text-surface-300 uppercase"> </p>'),
  le = f(
    '<div class="group overflow-hidden card transition-all duration-200 hover:ring-1 hover:ring-primary-500"><div class="relative aspect-video overflow-hidden"><!> <div class="from-background/90 absolute inset-0 bg-gradient-to-t to-transparent"></div> <div class="absolute bottom-0 left-0 p-4"><h4 class="h4 font-bold text-surface-50"> </h4> <!></div></div></div>'
  );
function ne(E, s) {
  fa(s, !0);
  var d = le(),
    _ = t(d),
    g = t(_);
  {
    var y = (u) => {
        var v = ie(),
          c = ea(v);
        (qa(2),
          z(() => {
            (oa(c, 'src', s.item.imageUrl), oa(c, 'alt', s.item.title));
          }),
          l(u, v));
      },
      L = (u) => {
        var v = oe(),
          c = t(v, !0);
        (a(v), z((k) => o(c, k), [() => s.item.title.slice(0, 2).toUpperCase()]), l(u, v));
      };
    M(g, (u) => {
      s.item.imageUrl ? u(y) : u(L, !1);
    });
  }
  var C = e(g, 4),
    x = t(C),
    w = t(x, !0);
  a(x);
  var m = e(x, 2);
  {
    var A = (u) => {
      var v = de(),
        c = t(v, !0);
      (a(v), z(() => o(c, s.item.subtitle)), l(u, v));
    };
    M(m, (u) => {
      s.item.subtitle && u(A);
    });
  }
  (a(C), a(_), a(d), z(() => o(w, s.item.title)), l(E, d), ma());
}
var ve = f(
    '<tr><td class="font-bold"> </td><td class="font-mono text-primary-400"> </td><td><span class="variant-soft-surface badge"> </span></td><td><span class="variant-soft-secondary badge font-bold"> </span></td><td><span class="variant-filled-surface badge font-bold tracking-wider"> </span></td><td class="text-surface-300"> </td></tr>'
  ),
  ce = f(
    '<div class="table-container"><table class="table-hover table"><thead><tr><th> </th><th> </th><th> </th><th> </th><th> </th><th> </th></tr></thead><tbody></tbody></table></div>'
  );
function _e(E, s) {
  fa(s, !0);
  var d = ce(),
    _ = t(d),
    g = t(_),
    y = t(g),
    L = t(y),
    C = t(L, !0);
  a(L);
  var x = e(L),
    w = t(x, !0);
  a(x);
  var m = e(x),
    A = t(m, !0);
  a(m);
  var u = e(m),
    v = t(u, !0);
  a(u);
  var c = e(u),
    k = t(c, !0);
  a(c);
  var D = e(c),
    T = t(D, !0);
  (a(D), a(y), a(g));
  var b = e(g);
  (ra(
    b,
    21,
    () => s.data,
    (i) => i.id,
    (i, n) => {
      var h = ve(),
        S = t(h),
        W = t(S, !0);
      a(S);
      var F = e(S),
        B = t(F, !0);
      a(F);
      var R = e(F),
        aa = t(R),
        ta = t(aa, !0);
      (a(aa), a(R));
      var K = e(R),
        q = t(K),
        Y = t(q, !0);
      (a(q), a(K));
      var X = e(K),
        G = t(X),
        Z = t(G, !0);
      (a(G), a(X));
      var J = e(X),
        va = t(J, !0);
      (a(J),
        a(h),
        z(() => {
          (o(W, r(n).manufacturer ?? '—'),
            o(B, r(n).productCode ?? '—'),
            o(ta, r(n).category ?? '—'),
            o(Y, r(n).scale ?? '—'),
            o(Z, r(n).railwayCompany ?? '—'),
            o(va, r(n).description ?? '—'));
        }),
        l(i, h));
    }
  ),
    a(b),
    a(_),
    a(d),
    z(
      (i, n, h, S, W, F) => {
        (o(C, i), o(w, n), o(A, h), o(v, S), o(k, W), o(T, F));
      },
      [() => vt(), () => ct(), () => _t(), () => ut(), () => Zt(), () => ft()]
    ),
    l(E, d),
    ma());
}
var ue = f('<span class="ml-1 font-mono text-primary-400"> </span>'),
  fe = f('<span class="variant-soft-surface badge flex items-center gap-1"><!> </span>'),
  me = f('<span class="variant-soft-surface badge flex items-center gap-1"><!> </span>'),
  he = f(
    '<div class="variant-filled-surface space-y-3 card border-l-4 border-surface-600 p-4 transition-colors hover:border-primary-500/50"><div class="flex items-start justify-between"><div><h4 class="h4 font-bold"> <!></h4> <p class="line-clamp-2 pt-1 text-sm text-surface-300"> </p></div> <span class="variant-filled-secondary badge font-bold"> </span></div> <div class="flex flex-wrap gap-2 border-t border-surface-700/50 pt-3"><!> <!></div></div>'
  );
function be(E, s) {
  fa(s, !0);
  var d = he(),
    _ = t(d),
    g = t(_),
    y = t(g),
    L = t(y),
    C = e(L);
  {
    var x = (b) => {
      var i = ue(),
        n = t(i);
      (a(i), z(() => o(n, `#${s.depot.productCode ?? ''}`)), l(b, i));
    };
    M(C, (b) => {
      s.depot.productCode && b(x);
    });
  }
  a(y);
  var w = e(y, 2),
    m = t(w, !0);
  (a(w), a(g));
  var A = e(g, 2),
    u = t(A, !0);
  (a(A), a(_));
  var v = e(_, 2),
    c = t(v);
  {
    var k = (b) => {
      var i = fe(),
        n = t(i);
      Ga(n, { size: 12 });
      var h = e(n);
      (a(i), z(() => o(h, ` ${s.depot.category ?? ''}`)), l(b, i));
    };
    M(c, (b) => {
      s.depot.category && b(k);
    });
  }
  var D = e(c, 2);
  {
    var T = (b) => {
      var i = me(),
        n = t(i);
      Ja(n, { size: 12 });
      var h = e(n);
      (a(i), z(() => o(h, ` ${s.depot.railwayCompany ?? ''}`)), l(b, i));
    };
    M(D, (b) => {
      s.depot.railwayCompany && b(T);
    });
  }
  (a(v),
    a(d),
    z(() => {
      (o(L, `${s.depot.manufacturer ?? '—' ?? ''} `),
        o(m, s.depot.description ?? '—'),
        o(u, s.depot.scale ?? '—'));
    }),
    l(E, d),
    ma());
}
var pe = f('<div class="skeleton h-28 w-full rounded-container"></div>'),
  ge = f('<div class="skeleton h-10 w-full rounded-sm"></div>'),
  xe = f(
    '<div class="space-y-4 lg:hidden"></div> <div class="hidden overflow-hidden rounded-container border border-surface-700/50 bg-surface-900/50 p-6 lg:block"><div class="skeleton mb-6 h-8 w-1/4 rounded"></div> <div class="space-y-4"></div></div>',
    1
  ),
  ye = f(
    '<div class="flex flex-col items-center justify-center rounded-container border-2 border-dashed border-surface-700/60 bg-surface-800/30 p-12 text-center"><div class="variant-soft-surface mb-4 badge-icon h-16 w-16"><!></div> <h4 class="h4 font-bold opacity-80"> </h4> <p class="mt-2 max-w-xs text-sm text-surface-400"> </p> <button class="variant-filled-primary mt-6 btn"><!> </button></div>'
  ),
  we = f(
    '<div class="space-y-4 lg:hidden"></div> <div class="hidden overflow-hidden rounded-container border border-surface-700/50 bg-surface-800 lg:block"><!></div>',
    1
  );
function ke(E, s) {
  fa(s, !0);
  let d = Ma(s, 'data', 19, () => []),
    _ = Ma(s, 'isLoading', 3, !1);
  var g = xa(),
    y = ea(g);
  {
    var L = (x) => {
        var w = xe(),
          m = ea(w);
        (ra(
          m,
          20,
          () => Array(3),
          Sa,
          (v, c, k) => {
            var D = pe();
            (oa(D, 'aria-label', `loading-depot-card-${k}`), l(v, D));
          }
        ),
          a(m));
        var A = e(m, 2),
          u = e(t(A), 2);
        (ra(
          u,
          20,
          () => Array(5),
          Sa,
          (v, c, k) => {
            var D = ge();
            (oa(D, 'aria-label', `loading-depot-row-${k}`), l(v, D));
          }
        ),
          a(u),
          a(A),
          l(x, w));
      },
      C = (x) => {
        var w = xa(),
          m = ea(w);
        {
          var A = (v) => {
              var c = ye(),
                k = t(c),
                D = t(k);
              (Qa(D, { size: 32, class: 'opacity-50' }), a(k));
              var T = e(k, 2),
                b = t(T, !0);
              a(T);
              var i = e(T, 2),
                n = t(i, !0);
              a(i);
              var h = e(i, 2);
              h.__click = () => Da(ja('/catalogue/new-model'));
              var S = t(h);
              Pa(S, { size: 18, class: 'mr-2' });
              var W = e(S);
              (a(h),
                a(c),
                z(
                  (F, B, R) => {
                    (o(b, F), o(n, B), o(W, ` ${R ?? ''}`));
                  },
                  [() => mt(), () => ht(), () => bt()]
                ),
                l(v, c));
            },
            u = (v) => {
              var c = we(),
                k = ea(c);
              (ra(
                k,
                21,
                d,
                (b) => b.id,
                (b, i) => {
                  be(b, {
                    get depot() {
                      return r(i);
                    }
                  });
                }
              ),
                a(k));
              var D = e(k, 2),
                T = t(D);
              (_e(T, {
                get data() {
                  return d();
                }
              }),
                a(D),
                l(v, c));
            };
          M(
            m,
            (v) => {
              !d() || d().length === 0 ? v(A) : v(u, !1);
            },
            !0
          );
        }
        l(x, w);
      };
    M(y, (x) => {
      _() ? x(L) : x(C, !1);
    });
  }
  (l(E, g), ma());
}
Va(['click']);
var je = f('<option> <!></option>'),
  Ce = f(
    '<div class="variant-soft-error rounded-container border border-error-700/40 p-3 text-sm text-error-100"> </div>'
  ),
  Ie = f(
    '<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4"><div class="w-full max-w-lg rounded-container border border-surface-700/70 bg-surface-900 shadow-xl"><div class="flex items-center justify-between border-b border-surface-800 px-4 py-3"><h3 class="text-base font-semibold tracking-wide uppercase"> </h3> <button class="variant-ghost-surface btn btn-sm" aria-label="close"> </button></div> <div class="space-y-4 p-4"><div class="space-y-2"><label for="wishlist-select" class="text-xs font-semibold tracking-wide text-surface-300 uppercase"> </label> <div class="grid grid-cols-1 gap-2 md:grid-cols-2"><select id="wishlist-select" class="select"><option disabled> </option><!></select> <input class="input" type="text"/></div></div> <div class="space-y-2"><label for="model-id" class="text-xs font-semibold tracking-wide text-surface-300 uppercase"> </label> <input id="model-id" class="input" type="text"/></div> <div class="space-y-2"><label for="wishlist-notes" class="text-xs font-semibold tracking-wide text-surface-300 uppercase"> </label> <textarea id="wishlist-notes" class="textarea" rows="3"></textarea></div> <!></div> <div class="flex items-center justify-end gap-2 border-t border-surface-800 px-4 py-3"><button class="variant-ghost-surface btn btn-sm"> </button> <button class="variant-filled-primary btn btn-sm"> </button></div></div></div>'
  );
function Le(E, s) {
  fa(s, !0);
  const d = Ka(),
    _ = $(() => ga.wishlists),
    g = $(() => ga.defaultWishlist);
  let y = $(() => r(g)?.id ?? null),
    L = ia(''),
    C = ia(''),
    x = ia(''),
    w = ia(!1),
    m = ia(null);
  Ya(() => {
    !r(y) && r(g) && j(y, r(g).id);
  });
  async function A() {
    if ((j(m, null), !r(C).trim())) {
      j(m, zt(), !0);
      return;
    }
    j(w, !0);
    try {
      let V = r(y);
      if (r(L).trim()) {
        const H = await ga.createWishlist(r(L).trim(), !1);
        if (!H) {
          j(m, At(), !0);
          return;
        }
        V = H.id;
      }
      if (!V) {
        j(m, Et(), !0);
        return;
      }
      if (!(await ga.addItem(V, r(C).trim()))) {
        j(m, Mt(), !0);
        return;
      }
      (d('saved'), u());
    } finally {
      j(w, !1);
    }
  }
  function u() {
    (d('close'), j(L, ''), j(C, ''), j(x, ''), j(m, null));
  }
  var v = Ie(),
    c = t(v),
    k = t(c),
    D = t(k),
    T = t(D, !0);
  a(D);
  var b = e(D, 2);
  b.__click = u;
  var i = t(b, !0);
  (a(b), a(k));
  var n = e(k, 2),
    h = t(n),
    S = t(h),
    W = t(S, !0);
  a(S);
  var F = e(S, 2),
    B = t(F),
    R = t(B),
    aa = t(R, !0);
  (a(R), (R.value = R.__value = ''));
  var ta = e(R);
  (ra(
    ta,
    17,
    () => r(_),
    (V) => V.id,
    (V, O) => {
      var H = je(),
        ha = t(H),
        ba = e(ha);
      {
        var pa = (ua) => {
          var ya = Xa('(default)');
          l(ua, ya);
        };
        M(ba, (ua) => {
          r(O).is_default && ua(pa);
        });
      }
      a(H);
      var la = {};
      (z(() => {
        (o(ha, `${r(O).name ?? ''} `),
          la !== (la = r(O).id) && (H.value = (H.__value = r(O).id) ?? ''));
      }),
        l(V, H));
    }
  ),
    a(B));
  var K = e(B, 2);
  (Ra(K), a(F), a(h));
  var q = e(h, 2),
    Y = t(q),
    X = t(Y, !0);
  a(Y);
  var G = e(Y, 2);
  (Ra(G), a(q));
  var Z = e(q, 2),
    J = t(Z),
    va = t(J, !0);
  a(J);
  var ca = e(J, 2);
  (Za(ca), a(Z));
  var za = e(Z, 2);
  {
    var Aa = (V) => {
      var O = Ce(),
        H = t(O, !0);
      (a(O), z(() => o(H, r(m))), l(V, O));
    };
    M(za, (V) => {
      r(m) && V(Aa);
    });
  }
  a(n);
  var Ca = e(n, 2),
    _a = t(Ca);
  _a.__click = u;
  var Ia = t(_a, !0);
  a(_a);
  var da = e(_a, 2);
  da.__click = A;
  var La = t(da, !0);
  (a(da),
    a(Ca),
    a(c),
    a(v),
    z(
      (V, O, H, ha, ba, pa, la, ua, ya, p, I, P) => {
        (o(T, V),
          o(i, O),
          o(W, H),
          oa(B, 'aria-label', ha),
          $a(R, !r(y)),
          o(aa, ba),
          oa(K, 'placeholder', pa),
          o(X, la),
          oa(G, 'placeholder', ua),
          o(va, ya),
          oa(ca, 'placeholder', p),
          (_a.disabled = r(w)),
          o(Ia, I),
          (da.disabled = r(w)),
          o(La, P));
      },
      [
        () => pt(),
        () => gt(),
        () => xt(),
        () => yt(),
        () => wt(),
        () => kt(),
        () => jt(),
        () => Ct(),
        () => It(),
        () => Lt(),
        () => Dt(),
        () => (r(w) ? St() : Vt())
      ]
    ),
    at(
      B,
      () => r(y),
      (V) => j(y, V)
    ),
    Ea(
      K,
      () => r(L),
      (V) => j(L, V)
    ),
    Ea(
      G,
      () => r(C),
      (V) => j(C, V)
    ),
    Ea(
      ca,
      () => r(x),
      (V) => j(x, V)
    ),
    l(E, v),
    ma());
}
Va(['click']);
async function De() {
  try {
    const s = await Wa(
      () => import('../chunks/zzNGAeR9.js'),
      __vite__mapDeps([0, 1]),
      import.meta.url
    );
    if (s?.commands?.dashboardSummary) {
      const d = await s.commands.dashboardSummary();
      if (d.status === 'ok') {
        const _ = d.data;
        return {
          totals: {
            collection_items: _.totals.collectionItems,
            wishlists: _.totals.wishlists,
            maintenance_due: _.totals.maintenanceDue,
            total_value: _.totals.totalValue
              ? {
                  amount: Number(_.totals.totalValue.amount),
                  currency: _.totals.totalValue.currency
                }
              : null
          },
          recent_items: _.recentItems,
          depot_items: _.depotItems
        };
      }
      throw new Error('Failed to fetch dashboard summary');
    }
  } catch {}
  const { invoke: E } = await Wa(
    async () => {
      const { invoke: s } = await import('../chunks/mPlcS5K-.js');
      return { invoke: s };
    },
    [],
    import.meta.url
  );
  return E('dashboard_summary');
}
class Se {
  #a = ia(null);
  #t = ia(!1);
  #e = ia(null);
  get data() {
    return r(this.#a);
  }
  get isLoading() {
    return r(this.#t);
  }
  get error() {
    return r(this.#e);
  }
  #r = $(() => (r(this.#a)?.totals?.maintenance_due ?? 0) > 0);
  get hasMaintenance() {
    return r(this.#r);
  }
  set hasMaintenance(s) {
    j(this.#r, s);
  }
  #s = $(() => r(this.#a)?.recent_items.length ?? 0);
  get recentItemsCount() {
    return r(this.#s);
  }
  set recentItemsCount(s) {
    j(this.#s, s);
  }
  async load() {
    if (!r(this.#t)) {
      (j(this.#t, !0), j(this.#e, null));
      try {
        const s = await De();
        j(this.#a, s, !0);
      } catch (s) {
        (console.error('Dashboard Store Error:', s),
          j(this.#e, 'dashboard_load_failed'),
          $t.error({
            id: 'dashboard-load',
            title: 'Dashboard failed to load',
            description: 'Please check your connection or try again.',
            duration: 4e3
          }));
      } finally {
        j(this.#t, !1);
      }
    }
  }
  async retry() {
    (j(this.#a, null), await this.load());
  }
}
const Ve = new Se();
var ze = f(
    '<div class="variant-soft-error flex flex-col items-center justify-center rounded-container border border-error-500/30 p-12 text-center"><div class="variant-filled-error mb-4 badge-icon h-12 w-12"><!></div> <h2 class="h2 font-bold"> </h2> <p class="mt-2 text-surface-200"> </p> <div class="mt-6 flex gap-4"><button class="variant-filled-primary btn btn-lg"> </button> <button class="variant-ghost-surface btn btn-lg"><!> </button></div></div>'
  ),
  Ae = f('<span class="variant-soft-error badge animate-pulse font-semibold"> </span>'),
  Ee = f('<div class="skeleton h-28 rounded-container"></div>'),
  Me = f('<div class="skeleton aspect-video w-full rounded-container"></div>'),
  Pe = f('<div class="grid grid-cols-1 gap-4 lg:grid-cols-2"></div>'),
  Re = f(
    '<div class="variant-soft-surface rounded-container border border-dashed border-surface-700/60 p-10 text-center text-surface-300"><p> </p></div>'
  ),
  Te = f('<div class="min-w-[80%] snap-center lg:min-w-0"><!></div>'),
  We = f(
    '<div class="hide-scrollbar flex snap-x snap-mandatory gap-4 overflow-x-auto pb-4 lg:grid lg:grid-cols-2"></div>'
  ),
  Be = f(
    '<div class="card border-2 border-dashed border-surface-500/20 p-8 text-center"><p class="mb-4 text-surface-300"> </p> <button class="variant-filled-secondary btn"><!> </button></div>'
  ),
  Fe = f(
    '<div class="space-y-8"><section><div class="mb-4 flex items-center justify-between"><h3 class="h3 text-sm font-bold tracking-wider text-surface-300 uppercase"> </h3> <!></div> <div class="grid grid-cols-2 gap-4 lg:grid-cols-4"><!></div></section> <div class="grid grid-cols-1 gap-8 lg:grid-cols-3"><div class="space-y-8 lg:col-span-2"><section><div class="mb-4 flex items-center justify-between"><h3 class="h3 text-sm font-bold tracking-wider text-surface-300 uppercase"> </h3> <a class="text-accent-500 text-sm font-bold hover:underline"> </a></div> <!></section> <section><div class="mb-4 flex items-center justify-between"><h3 class="h3 text-sm font-bold tracking-wider text-surface-300 uppercase"> </h3></div> <!></section></div> <aside><div class="sticky top-24 space-y-4"><h3 class="h3 text-sm font-bold tracking-wider text-surface-300 uppercase"> </h3> <!></div></aside></div></div>'
  ),
  He = f('<!> <!>', 1);
function Ze(E, s) {
  fa(s, !0);
  const d = Ve,
    _ = $(() => d.data?.totals ?? null),
    g = $(() => A(r(_))),
    y = $(() => d.data?.recent_items ?? []),
    L = $(() => d.data?.depot_items ?? []);
  let C = ia(!1);
  tt(() => {
    d.load();
  });
  function x() {
    d.retry();
  }
  function w() {
    Da(ja('/'));
  }
  function m(i) {
    if (!i) return '—';
    const n = i.amount / 100;
    return `${i.currency} ${n.toLocaleString(void 0, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
  }
  function A(i) {
    const n = m(i?.total_value ?? null),
      h = i?.collection_items ?? 0,
      S = i?.maintenance_due ?? 0;
    return [
      { label: Jt(), value: n, trend: 'neutral', trendValue: '' },
      { label: Qt(), value: `${h}`, trend: 'neutral', trendValue: '' },
      {
        label: Yt(),
        value: `${S}`,
        trend: S > 0 ? 'down' : 'neutral',
        trendValue: S > 0 ? `${S} ${Kt()}` : '—'
      }
    ];
  }
  const u = $(() => [
    {
      id: 'add-railway-model',
      label: Ta(),
      icon: Pa,
      onClick: () => Da(ja('/catalogue/new-model'))
    },
    {
      id: 'add-wishlist-item',
      label: Ut(),
      icon: dt,
      onClick: () => {
        (ga.wishlists.length || ga.fetchWishlists(), j(C, !0));
      }
    }
  ]);
  var v = He();
  et('1uha8ag', (i) => {
    rt(
      (n, h) => {
        st.title = `${n ?? ''} | ${h ?? ''}`;
      },
      [() => lt(), () => nt()]
    );
  });
  var c = ea(v);
  {
    var k = (i) => {
        var n = ze(),
          h = t(n),
          S = t(h);
        (it(S, {}), a(h));
        var W = e(h, 2),
          F = t(W, !0);
        a(W);
        var B = e(W, 2),
          R = t(B, !0);
        a(B);
        var aa = e(B, 2),
          ta = t(aa);
        ta.__click = x;
        var K = t(ta, !0);
        a(ta);
        var q = e(ta, 2);
        q.__click = w;
        var Y = t(q);
        ot(Y, { class: 'mr-2 h-4 w-4' });
        var X = e(Y);
        (a(q),
          a(aa),
          a(n),
          z(
            (G, Z, J, va) => {
              (o(F, G), o(R, Z), o(K, J), o(X, ` ${va ?? ''}`));
            },
            [() => Pt(), () => Rt(), () => Tt(), () => Wt()]
          ),
          l(i, n));
      },
      D = (i) => {
        var n = Fe(),
          h = t(n),
          S = t(h),
          W = t(S),
          F = t(W, !0);
        a(W);
        var B = e(W, 2);
        {
          var R = (p) => {
            var I = Ae(),
              P = t(I);
            (a(I),
              z((N) => o(P, `${r(_).maintenance_due ?? ''} ${N ?? ''}`), [() => qt()]),
              l(p, I));
          };
          M(B, (p) => {
            r(_)?.maintenance_due && p(R);
          });
        }
        a(S);
        var aa = e(S, 2),
          ta = t(aa);
        {
          var K = (p) => {
              var I = xa(),
                P = ea(I);
              (ra(
                P,
                16,
                () => Array(4),
                Sa,
                (N, Q) => {
                  var U = Ee();
                  l(N, U);
                }
              ),
                l(p, I));
            },
            q = (p) => {
              var I = xa(),
                P = ea(I);
              (ra(
                P,
                17,
                () => r(g),
                (N) => N.label,
                (N, Q) => {
                  te(N, {
                    get stat() {
                      return r(Q);
                    }
                  });
                }
              ),
                l(p, I));
            };
          M(ta, (p) => {
            d.isLoading ? p(K) : p(q, !1);
          });
        }
        (a(aa), a(h));
        var Y = e(h, 2),
          X = t(Y),
          G = t(X),
          Z = t(G),
          J = t(Z),
          va = t(J, !0);
        a(J);
        var ca = e(J, 2),
          za = t(ca, !0);
        (a(ca), a(Z));
        var Aa = e(Z, 2);
        {
          var Ca = (p) => {
              var I = Pe();
              (ra(
                I,
                20,
                () => Array(2),
                Sa,
                (P, N) => {
                  var Q = Me();
                  l(P, Q);
                }
              ),
                a(I),
                l(p, I));
            },
            _a = (p) => {
              var I = xa(),
                P = ea(I);
              {
                var N = (U) => {
                    var sa = Re(),
                      na = t(sa),
                      wa = t(na, !0);
                    (a(na), a(sa), z((ka) => o(wa, ka), [() => Xt()]), l(U, sa));
                  },
                  Q = (U) => {
                    var sa = We();
                    (ra(
                      sa,
                      21,
                      () => r(y),
                      (na) => na.id,
                      (na, wa) => {
                        var ka = Te(),
                          Ba = t(ka);
                        (ne(Ba, {
                          get item() {
                            return r(wa);
                          }
                        }),
                          a(ka),
                          l(na, ka));
                      }
                    ),
                      a(sa),
                      l(U, sa));
                  };
                M(
                  P,
                  (U) => {
                    r(y).length ? U(Q, !1) : U(N);
                  },
                  !0
                );
              }
              l(p, I);
            };
          M(Aa, (p) => {
            d.isLoading ? p(Ca) : p(_a, !1);
          });
        }
        a(G);
        var Ia = e(G, 2),
          da = t(Ia),
          La = t(da),
          V = t(La, !0);
        (a(La), a(da));
        var O = e(da, 2);
        {
          var H = (p) => {
              var I = Be(),
                P = t(I),
                N = t(P, !0);
              a(P);
              var Q = e(P, 2);
              Q.__click = () => Da(ja('/catalogue/new-model'));
              var U = t(Q);
              Pa(U, { class: 'mr-2' });
              var sa = e(U);
              (a(Q),
                a(I),
                z(
                  (na, wa) => {
                    (o(N, na), o(sa, ` ${wa ?? ''}`));
                  },
                  [() => Gt(), () => Ta()]
                ),
                l(p, I));
            },
            ha = (p) => {
              ke(p, {
                get data() {
                  return r(L);
                },
                get isLoading() {
                  return d.isLoading;
                }
              });
            };
          M(O, (p) => {
            !d.isLoading && r(L).length === 0 ? p(H) : p(ha, !1);
          });
        }
        (a(Ia), a(X));
        var ba = e(X, 2),
          pa = t(ba),
          la = t(pa),
          ua = t(la, !0);
        a(la);
        var ya = e(la, 2);
        (se(ya, {
          get actions() {
            return r(u);
          }
        }),
          a(pa),
          a(ba),
          a(Y),
          a(n),
          z(
            (p, I, P, N, Q, U) => {
              (o(F, p), o(va, I), oa(ca, 'href', P), o(za, N), o(V, Q), o(ua, U));
            },
            [() => Bt(), () => Ft(), () => ja('/my-collection'), () => Ht(), () => Nt(), () => Ot()]
          ),
          l(i, n));
      };
    M(c, (i) => {
      d.error ? i(k) : i(D, !1);
    });
  }
  var T = e(c, 2);
  {
    var b = (i) => {
      Le(i, { $$events: { close: () => j(C, !1) } });
    };
    M(T, (i) => {
      r(C) && i(b);
    });
  }
  (l(E, v), ma());
}
Va(['click']);
export { Ze as component };
