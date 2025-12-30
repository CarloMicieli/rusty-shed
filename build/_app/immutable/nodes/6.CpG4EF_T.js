import {
  L as Le,
  p as le,
  y as ye,
  f as T,
  d as a,
  G as he,
  r as e,
  s,
  D as se,
  t as X,
  _ as Oe,
  O as Fe,
  e as n,
  b as y,
  c as ce,
  a as ve,
  N as Be,
  E as me,
  m as i,
  u as I,
  k as Z,
  n as w,
  o as ze,
  a0 as Me,
  a1 as Ne,
  a2 as Se,
  l as pe,
  B as qe,
  h as Ge,
  a3 as Pe,
  Y as Ue,
  j as we,
  a4 as Qe,
  $ as Ve,
  R as Xe,
  a5 as Ye
} from '../chunks/C2P8ifMu.js';
import '../chunks/8AwhvTxk.js';
import { a as He } from '../chunks/Cai0cCrU.js';
import {
  d as Je,
  a as Ke,
  b as je,
  c as Te,
  e as ge,
  f as xe,
  g as We,
  h as Ze,
  i as Ce,
  j as $e,
  k as ea,
  l as fe,
  m as aa,
  n as ta,
  o as ra,
  p as sa,
  q as ia,
  r as na,
  s as oa
} from '../chunks/wxoaVA6c.js';
import { d as be } from '../chunks/DCW-6SkD.js';
import { commands as ke } from '../chunks/zzNGAeR9.js';
var da = T(
    '<div class="rounded-xl border border-dashed border-surface-500/20 p-8 text-center"><p class="text-sm text-surface-500"> </p></div>'
  ),
  la = T(
    '<div class="flex flex-wrap items-center justify-between gap-2 border-t border-surface-500/10 pt-4 text-xs text-surface-400"><p> </p> <button type="button" class="variant-ghost-primary btn btn-sm"> </button></div>'
  ),
  ca = T('<div class="grid grid-cols-1 gap-4 lg:grid-cols-2"></div> <!>', 1),
  va = T(
    '<section class="space-y-2 pt-2"><div class="sticky z-10 border-b border-surface-500/10 bg-surface-50/80 backdrop-blur-sm"><div class="flex items-center gap-3 rounded-lg px-2 py-2"><span><!></span> <div class="flex items-center gap-2"><h2 class="text-lg font-semibold tracking-tight"> </h2> <span class="variant-soft-surface badge font-mono text-xs"> </span></div></div></div> <div class="space-y-3"><!></div></section>'
  );
function _e(P, r) {
  le(r, !0);
  let c = ye(r, 'toneClass', 3, 'variant-filled-surface'),
    v = ye(r, 'stickyOffset', 3, 'var(--header-offset, 4rem)'),
    m = Z(!1);
  const g = I(() => (i(m) || r.items.length <= 100 ? r.items : r.items.slice(0, 100))),
    O = I(() => !i(m) && r.items.length > 100);
  var b = va(),
    u = a(b);
  let F;
  var h = a(u),
    B = a(h),
    C = a(B);
  (he(
    C,
    () => r.icon,
    (p, l) => {
      l(p, { size: 16 });
    }
  ),
    e(B));
  var k = s(B, 2),
    d = a(k),
    L = a(d, !0);
  e(d);
  var U = s(d, 2),
    z = a(U, !0);
  (e(U), e(k), e(h), e(u));
  var M = s(u, 2),
    Y = a(M);
  {
    var R = (p) => {
        var l = da(),
          f = a(l),
          A = a(f, !0);
        (e(f), e(l), X(() => n(A, r.emptyMessage)), y(p, l));
      },
      Q = (p) => {
        var l = ca(),
          f = ve(l);
        (Be(
          f,
          21,
          () => i(g),
          (E) => E.id,
          (E, q) => {
            var J = me(),
              $ = ve(J);
            (he(
              $,
              () => r.card,
              (K, ie) => {
                ie(K, {
                  get item() {
                    return i(q);
                  }
                });
              }
            ),
              y(E, J));
          }
        ),
          e(f));
        var A = s(f, 2);
        {
          var D = (E) => {
            var q = la(),
              J = a(q),
              $ = a(J, !0);
            e(J);
            var K = s(J, 2);
            K.__click = () => w(m, !0);
            var ie = a(K, !0);
            (e(K),
              e(q),
              X(
                (ee, ue) => {
                  (n($, ee), n(ie, ue));
                },
                [() => Je({ showing: 100, total: r.items.length }), () => Ke()]
              ),
              y(E, q));
          };
          se(A, (E) => {
            i(O) && E(D);
          });
        }
        y(p, l);
      };
    se(Y, (p) => {
      r.items.length === 0 ? p(R) : p(Q, !1);
    });
  }
  (e(M),
    e(b),
    X(() => {
      ((F = Oe(u, '', F, { top: v() })),
        Fe(B, 1, `badge ${c() ?? ''} flex items-center justify-center p-1.5`),
        n(L, r.title),
        n(z, r.items.length));
    }),
    y(P, b),
    ce());
}
Le(['click']);
var ua = T(
  '<div class="variant-hover space-y-3 card border border-surface-700/40 p-4"><div class="flex items-start justify-between gap-3"><div class="flex items-center gap-2"><span class="variant-filled-primary badge flex items-center gap-1"><!></span> <div class="space-y-1"><p class="text-xs tracking-wide text-surface-500 uppercase"> </p> <h3 class="text-base leading-tight font-semibold"> </h3> <p class="text-sm text-surface-400"> </p></div></div> <span class="variant-soft-warning badge flex items-center gap-1 font-mono text-xs"><!> </span></div> <div class="flex flex-wrap gap-2 text-sm"><span class="variant-soft-primary badge"> </span> <span class="variant-soft-surface badge"> </span> <span class="variant-soft-surface badge"> </span></div></div>'
);
function pa(P, r) {
  le(r, !0);
  var c = ua(),
    v = a(c),
    m = a(v),
    g = a(m),
    O = a(g);
  (ze(O, { size: 14 }), e(g));
  var b = s(g, 2),
    u = a(b),
    F = a(u, !0);
  e(u);
  var h = s(u, 2),
    B = a(h, !0);
  e(h);
  var C = s(h, 2),
    k = a(C, !0);
  (e(C), e(b), e(m));
  var d = s(m, 2),
    L = a(d);
  Me(L, { size: 14 });
  var U = s(L);
  (e(d), e(v));
  var z = s(v, 2),
    M = a(z),
    Y = a(M);
  e(M);
  var R = s(M, 2),
    Q = a(R);
  e(R);
  var p = s(R, 2),
    l = a(p);
  (e(p),
    e(z),
    e(c),
    X(
      (f, A, D, E, q) => {
        (n(F, f),
          n(B, r.item.group),
          n(k, r.item.railwayCompany),
          n(U, ` ${A ?? ''}: ${r.item.dccAddress ?? ''}`),
          n(Y, `${D ?? ''}: ${r.item.roadNumber ?? ''}`),
          n(Q, `${E ?? ''}: ${r.item.railwayCompany ?? ''}`),
          n(l, `${q ?? ''}: ${r.item.livery ?? ''}`));
      },
      [() => je(), () => Te(), () => ge(), () => be(), () => xe()]
    ),
    y(P, c),
    ce());
}
var fa = T(
  '<div class="variant-hover space-y-3 card border border-surface-700/40 p-4"><div class="flex items-start justify-between gap-3"><div class="flex items-center gap-2"><span class="variant-filled-secondary badge flex items-center gap-1"><!></span> <div class="space-y-1"><p class="text-xs tracking-wide text-surface-500 uppercase"> </p> <h3 class="text-base leading-tight font-semibold"> </h3> <p class="text-sm text-surface-400"> </p></div></div> <span class="variant-soft-warning badge flex items-center gap-1 font-mono text-xs"><!> </span></div> <div class="flex flex-wrap gap-2 text-sm"><span class="variant-soft-primary badge"> </span> <span class="variant-soft-surface badge"> </span> <span class="variant-soft-surface badge"> </span></div></div>'
);
function _a(P, r) {
  le(r, !0);
  var c = fa(),
    v = a(c),
    m = a(v),
    g = a(m),
    O = a(g);
  (Ne(O, { size: 14 }), e(g));
  var b = s(g, 2),
    u = a(b),
    F = a(u, !0);
  e(u);
  var h = s(u, 2),
    B = a(h, !0);
  e(h);
  var C = s(h, 2),
    k = a(C, !0);
  (e(C), e(b), e(m));
  var d = s(m, 2),
    L = a(d);
  Me(L, { size: 14 });
  var U = s(L);
  (e(d), e(v));
  var z = s(v, 2),
    M = a(z),
    Y = a(M);
  e(M);
  var R = s(M, 2),
    Q = a(R);
  e(R);
  var p = s(R, 2),
    l = a(p);
  (e(p),
    e(z),
    e(c),
    X(
      (f, A, D, E, q) => {
        (n(F, f),
          n(B, r.item.group),
          n(k, r.item.railwayCompany),
          n(U, ` ${A ?? ''}: ${r.item.dccAddress ?? ''}`),
          n(Y, `${D ?? ''}: ${r.item.roadNumber ?? ''}`),
          n(Q, `${E ?? ''}: ${r.item.railwayCompany ?? ''}`),
          n(l, `${q ?? ''}: ${r.item.livery ?? ''}`));
      },
      [() => je(), () => Te(), () => ge(), () => be(), () => xe()]
    ),
    y(P, c),
    ce());
}
var ma = T('<span class="variant-soft-secondary badge"> </span>'),
  ga = T(
    '<div class="variant-hover space-y-3 card border border-surface-700/40 p-4"><div class="flex items-start justify-between gap-3"><div class="flex items-center gap-2"><span class="variant-filled-surface badge flex items-center gap-1"><!></span> <div class="space-y-1"><p class="text-xs tracking-wide text-surface-500 uppercase"> </p> <h3 class="text-base leading-tight font-semibold"> </h3> <p class="text-sm text-surface-400"> </p></div></div></div> <div class="flex flex-wrap gap-2 text-sm"><span class="variant-soft-primary badge"> </span> <span class="variant-soft-surface badge"> </span> <!> <span class="variant-soft-surface badge"> </span></div></div>'
  );
function xa(P, r) {
  le(r, !0);
  var c = ga(),
    v = a(c),
    m = a(v),
    g = a(m),
    O = a(g);
  (Se(O, { size: 14 }), e(g));
  var b = s(g, 2),
    u = a(b),
    F = a(u, !0);
  e(u);
  var h = s(u, 2),
    B = a(h, !0);
  e(h);
  var C = s(h, 2),
    k = a(C, !0);
  (e(C), e(b), e(m), e(v));
  var d = s(v, 2),
    L = a(d),
    U = a(L);
  e(L);
  var z = s(L, 2),
    M = a(z);
  e(z);
  var Y = s(z, 2);
  {
    var R = (l) => {
      var f = ma(),
        A = a(f);
      (e(f), X((D) => n(A, `${D ?? ''}: ${r.item.serviceLevel ?? ''}`), [() => Ze()]), y(l, f));
    };
    se(Y, (l) => {
      r.item.category === 'passenger' && r.item.serviceLevel && l(R);
    });
  }
  var Q = s(Y, 2),
    p = a(Q);
  (e(Q),
    e(d),
    e(c),
    X(
      (l, f, A, D) => {
        (n(F, l),
          n(B, r.item.type),
          n(k, r.item.railwayCompany),
          n(U, `${f ?? ''}: ${r.item.roadNumber ?? ''}`),
          n(M, `${A ?? ''}: ${r.item.livery ?? ''}`),
          n(p, `${D ?? ''}: ${r.item.railwayCompany ?? ''}`));
      },
      [() => We(), () => ge(), () => xe(), () => be()]
    ),
    y(P, c),
    ce());
}
function ba(P, r = 150) {
  let c = null;
  return (...v) => {
    (c && clearTimeout(c),
      (c = setTimeout(() => {
        P(...v);
      }, r)));
  };
}
var ya = T('<button class="variant-ghost-surface btn btn-sm px-2"><!></button>'),
  ha = T(
    '<div class="flex items-center gap-3 rounded-xl border border-surface-700/60 bg-surface-900 p-4"><div class="border-accent-400 h-4 w-4 animate-spin rounded-full border-2 border-t-transparent" aria-hidden="true"></div> <p class="text-sm text-surface-300">Loading depot…</p></div>'
  ),
  wa = T(
    '<div class="flex flex-col gap-3 rounded-xl border border-amber-500/50 bg-amber-950/50 p-4 text-amber-100"><p class="text-sm font-semibold"> </p> <div class="flex gap-2"><button class="variant-filled-primary btn btn-sm">Retry</button> <button class="variant-ghost-surface btn btn-sm"> </button></div></div>'
  ),
  Ca = T(
    '<div class="flex flex-col items-center justify-center gap-3 rounded-xl border border-dashed border-surface-700/50 bg-surface-900 p-8 text-center"><p class="text-lg font-semibold"> </p> <button class="variant-soft-primary btn"> </button></div>'
  ),
  ka = T('<div class="space-y-8"><!> <!> <!></div>'),
  La = T(
    '<div class="mx-auto max-w-4xl space-y-6 p-4 pt-4" style="--header-offset: 4rem;"><div class="space-y-1"><p class="text-sm tracking-[0.2em] text-surface-400 uppercase"> </p> <h1 class="h2 font-bold"> </h1> <p class="text-sm text-surface-400"> </p></div> <div class="rounded-xl border border-surface-700/60 bg-surface-900 p-3"><div class="input-group items-center gap-2"><!> <input class="w-full bg-transparent text-sm outline-none placeholder:text-surface-500"/> <!></div></div> <!></div>'
  );
function Ra(P, r) {
  le(r, !0);
  let c = Z(!0),
    v = Z(null),
    m = Z(pe([])),
    g = Z(pe([])),
    O = Z(pe([])),
    b = Z(''),
    u = Z('');
  const F = 'var(--header-offset, 4rem)',
    h = ba((t) => {
      w(u, t.trim().toLowerCase(), !0);
    }, 150);
  function B(t) {
    (w(b, t, !0), h(t));
  }
  function C() {
    (w(b, ''), w(u, ''));
  }
  const k = I(() => i(u)),
    d = (t) => (i(k) ? (t == null ? '' : String(t)).toLowerCase().includes(i(k)) : !0),
    L = I(() =>
      i(k)
        ? i(m).filter(
            (t) =>
              d(t.roadNumber) || d(t.railwayCompany) || d(t.group) || d(t.livery) || d(t.dccAddress)
          )
        : i(m)
    ),
    U = I(() =>
      i(k)
        ? i(g).filter(
            (t) =>
              d(t.roadNumber) || d(t.railwayCompany) || d(t.group) || d(t.livery) || d(t.dccAddress)
          )
        : i(g)
    ),
    z = I(() =>
      i(k)
        ? i(O).filter(
            (t) =>
              d(t.roadNumber) ||
              d(t.railwayCompany) ||
              d(t.type) ||
              d(t.livery) ||
              d(t.serviceLevel)
          )
        : i(O)
    ),
    M = I(() => i(L).length + i(U).length + i(z).length);
  function Y(t, o, N, x) {
    const G = t.description || t.product_code,
      S = o.data.railway?.display ?? null,
      j = o.data.livery ?? null;
    if (o.category === 'Locomotive') {
      const _ = o.data;
      x.locomotives.push({
        id: _.id,
        group: G,
        roadNumber: _.road_number ?? null,
        railwayCompany: S,
        livery: j,
        dccAddress: N
      });
      return;
    }
    if (o.category === 'ElectricMultipleUnit' || o.category === 'Railcar') {
      const _ = o.data;
      x.trains.push({
        id: _.id,
        group: G,
        roadNumber: _.road_number ?? null,
        railwayCompany: S,
        livery: j,
        dccAddress: N
      });
      return;
    }
    if (o.category === 'PassengerCar' || o.category === 'FreightCar') {
      const _ = o.data;
      x.cars.push({
        id: _.id,
        type: _.type_name,
        roadNumber: _.road_number ?? null,
        railwayCompany: S,
        livery: j,
        category: o.category === 'PassengerCar' ? 'passenger' : 'freight',
        serviceLevel: 'service_level' in _ ? (_.service_level ?? null) : null,
        dccAddress: N
      });
    }
  }
  function R(t, o) {
    const N = new Map(o.map((G) => [G.id, G])),
      x = { locomotives: [], trains: [], cars: [] };
    for (const G of t.items) {
      const S = N.get(G.railway_model_id);
      if (S)
        for (const j of G.rolling_stocks) {
          const _ = S.rolling_stocks.find(
            (ae) => ae.data.id === j.rolling_stock_id || ae.data.id === j.id
          );
          if (!_) continue;
          const oe = j.digital?.dcc_address ?? null;
          Y(S, _, oe, x);
        }
    }
    (w(m, x.locomotives, !0), w(g, x.trains, !0), w(O, x.cars, !0));
  }
  async function Q() {
    (w(c, !0), w(v, null), w(m, [], !0), w(g, [], !0), w(O, [], !0));
    try {
      const t = await ke.getDepot();
      if (t.status === 'error') throw new Error(String(t.error ?? 'Failed to load depot'));
      const o = t.data,
        N = Array.from(new Set(o.items.map((G) => G.railway_model_id)));
      if (N.length === 0) return;
      const x = await ke.getRailwayModelsByIds(N);
      if (x.status === 'error') throw new Error(String(x.error ?? 'Failed to load catalog models'));
      R(o, x.data);
    } catch (t) {
      w(v, t instanceof Error ? t.message : 'Unknown error loading depot', !0);
    } finally {
      w(c, !1);
    }
  }
  qe(Q);
  var p = La();
  Ge('s27ve', (t) => {
    Ue(
      (o) => {
        Ve.title = o ?? '';
      },
      [() => Ce()]
    );
  });
  var l = a(p),
    f = a(l),
    A = a(f, !0);
  e(f);
  var D = s(f, 2),
    E = a(D, !0);
  e(D);
  var q = s(D, 2),
    J = a(q, !0);
  (e(q), e(l));
  var $ = s(l, 2),
    K = a($),
    ie = a(K);
  Pe(ie, { size: 18, class: 'text-surface-500' });
  var ee = s(ie, 2);
  (Xe(ee), (ee.__input = (t) => B(t.currentTarget.value)));
  var ue = s(ee, 2);
  {
    var Re = (t) => {
      var o = ya();
      o.__click = C;
      var N = a(o);
      (Ye(N, { size: 16 }), e(o), X((x) => we(o, 'aria-label', x), [() => fe()]), y(t, o));
    };
    se(ue, (t) => {
      i(b) && t(Re);
    });
  }
  (e(K), e($));
  var Ae = s($, 2);
  {
    var De = (t) => {
        var o = ha();
        y(t, o);
      },
      Ee = (t) => {
        var o = me(),
          N = ve(o);
        {
          var x = (S) => {
              var j = wa(),
                _ = a(j),
                oe = a(_, !0);
              e(_);
              var ae = s(_, 2),
                H = a(ae);
              H.__click = Q;
              var V = s(H, 2);
              V.__click = C;
              var W = a(V, !0);
              (e(V),
                e(ae),
                e(j),
                X(
                  (ne) => {
                    (n(oe, i(v)), n(W, ne));
                  },
                  [() => fe()]
                ),
                y(S, j));
            },
            G = (S) => {
              var j = me(),
                _ = ve(j);
              {
                var oe = (H) => {
                    var V = Ca(),
                      W = a(V),
                      ne = a(W, !0);
                    e(W);
                    var de = s(W, 2);
                    de.__click = C;
                    var te = a(de, !0);
                    (e(de),
                      e(V),
                      X(
                        (re, Ie) => {
                          (n(ne, re), n(te, Ie));
                        },
                        [() => aa(), () => fe()]
                      ),
                      y(H, V));
                  },
                  ae = (H) => {
                    var V = ka(),
                      W = a(V);
                    {
                      let te = I(() => ra()),
                        re = I(() => ta());
                      _e(W, {
                        get title() {
                          return i(te);
                        },
                        get items() {
                          return i(L);
                        },
                        get icon() {
                          return ze;
                        },
                        get card() {
                          return pa;
                        },
                        toneClass: 'variant-filled-primary',
                        stickyOffset: F,
                        get emptyMessage() {
                          return i(re);
                        }
                      });
                    }
                    var ne = s(W, 2);
                    {
                      let te = I(() => ia()),
                        re = I(() => sa());
                      _e(ne, {
                        get title() {
                          return i(te);
                        },
                        get items() {
                          return i(U);
                        },
                        get icon() {
                          return Ne;
                        },
                        get card() {
                          return _a;
                        },
                        toneClass: 'variant-filled-secondary',
                        stickyOffset: F,
                        get emptyMessage() {
                          return i(re);
                        }
                      });
                    }
                    var de = s(ne, 2);
                    {
                      let te = I(() => oa()),
                        re = I(() => na());
                      _e(de, {
                        get title() {
                          return i(te);
                        },
                        get items() {
                          return i(z);
                        },
                        get icon() {
                          return Se;
                        },
                        get card() {
                          return xa;
                        },
                        toneClass: 'variant-filled-surface',
                        stickyOffset: F,
                        get emptyMessage() {
                          return i(re);
                        }
                      });
                    }
                    (e(V), y(H, V));
                  };
                se(
                  _,
                  (H) => {
                    i(M) === 0 ? H(oe) : H(ae, !1);
                  },
                  !0
                );
              }
              y(S, j);
            };
          se(
            N,
            (S) => {
              i(v) ? S(x) : S(G, !1);
            },
            !0
          );
        }
        y(t, o);
      };
    se(Ae, (t) => {
      i(c) ? t(De) : t(Ee, !1);
    });
  }
  (e(p),
    X(
      (t, o, N, x) => {
        (n(A, t), n(E, o), n(J, N), we(ee, 'placeholder', x), Qe(ee, i(b)));
      },
      [() => He(), () => Ce(), () => $e(), () => ea()]
    ),
    y(P, p),
    ce());
}
Le(['input', 'click']);
export { Ra as component };
