const __vite__mapDeps = (
  i,
  m = __vite__mapDeps,
  d = m.f ||
    (m.f = [
      '../nodes/0.D9LH-Nwr.js',
      '../chunks/PPVm8Dsz.js',
      '../chunks/C2P8ifMu.js',
      '../chunks/Ce2uwuEM.js',
      '../chunks/D0U87bhI.js',
      '../chunks/Da5IZt4M.js',
      '../chunks/Cas8_-DH.js',
      '../chunks/8AwhvTxk.js',
      '../chunks/fcFX1gix.js',
      '../chunks/B5syxjUO.js',
      '../chunks/DCbz5KVu.js',
      '../chunks/Cai0cCrU.js',
      '../chunks/6uZgpdQ9.js',
      '../chunks/Cxbe8PBw.js',
      '../chunks/DAlnca8K.js',
      '../chunks/B6BiQSwf.js',
      '../chunks/DxQretAU.js',
      '../chunks/zzNGAeR9.js',
      '../chunks/mPlcS5K-.js',
      '../chunks/BFM2npA3.js',
      '../assets/0.BSWQjA37.css',
      '../nodes/1.C5cr4cd-.js',
      '../nodes/2.BYGjSIcc.js',
      '../chunks/DdIlMJnB.js',
      '../chunks/DCW-6SkD.js',
      '../nodes/3.CQbfRTZQ.js',
      '../chunks/wxoaVA6c.js',
      '../chunks/BUXTf0nH.js',
      '../assets/3.0OI2aY77.css',
      '../nodes/4.Bo_XCLGS.js',
      '../nodes/5.D2P2LPQx.js',
      '../nodes/6.CpG4EF_T.js',
      '../nodes/7.CSkPQsMX.js'
    ])
) => i.map((i) => d[i]);
import { _ as a } from '../chunks/PPVm8Dsz.js';
import {
  p as M,
  y as g,
  z as q,
  A as F,
  B as H,
  m,
  k as p,
  n as E,
  C as N,
  f as V,
  a as l,
  D as h,
  s as Q,
  b as c,
  c as S,
  E as b,
  G as P,
  I as O,
  u as y,
  d as U,
  r as W,
  J as X,
  t as Y,
  e as Z,
  K as $
} from '../chunks/C2P8ifMu.js';
const it = {};
var tt = V(
    '<div id="svelte-announcer" aria-live="assertive" aria-atomic="true" style="position: absolute; left: 0; top: 0; clip: rect(0 0 0 0); clip-path: inset(50%); overflow: hidden; white-space: nowrap; width: 1px; height: 1px"><!></div>'
  ),
  et = V('<!> <!>', 1);
function rt(r, t) {
  M(t, !0);
  let o = g(t, 'components', 23, () => []),
    f = g(t, 'data_0', 3, null),
    A = g(t, 'data_1', 3, null);
  (q(() => t.stores.page.set(t.page)),
    F(() => {
      (t.stores, t.page, t.constructors, o(), t.form, f(), A(), t.stores.page.notify());
    }));
  let v = p(!1),
    D = p(!1),
    I = p(null);
  H(() => {
    const e = t.stores.page.subscribe(() => {
      m(v) &&
        (E(D, !0),
        N().then(() => {
          E(I, document.title || 'untitled page', !0);
        }));
    });
    return (E(v, !0), e);
  });
  const x = y(() => t.constructors[1]);
  var R = et(),
    L = l(R);
  {
    var j = (e) => {
        const s = y(() => t.constructors[0]);
        var n = b(),
          d = l(n);
        (P(
          d,
          () => m(s),
          (i, _) => {
            O(
              _(i, {
                get data() {
                  return f();
                },
                get form() {
                  return t.form;
                },
                get params() {
                  return t.page.params;
                },
                children: (u, ot) => {
                  var T = b(),
                    B = l(T);
                  (P(
                    B,
                    () => m(x),
                    (G, J) => {
                      O(
                        J(G, {
                          get data() {
                            return A();
                          },
                          get form() {
                            return t.form;
                          },
                          get params() {
                            return t.page.params;
                          }
                        }),
                        (K) => (o()[1] = K),
                        () => o()?.[1]
                      );
                    }
                  ),
                    c(u, T));
                },
                $$slots: { default: !0 }
              }),
              (u) => (o()[0] = u),
              () => o()?.[0]
            );
          }
        ),
          c(e, n));
      },
      k = (e) => {
        const s = y(() => t.constructors[0]);
        var n = b(),
          d = l(n);
        (P(
          d,
          () => m(s),
          (i, _) => {
            O(
              _(i, {
                get data() {
                  return f();
                },
                get form() {
                  return t.form;
                },
                get params() {
                  return t.page.params;
                }
              }),
              (u) => (o()[0] = u),
              () => o()?.[0]
            );
          }
        ),
          c(e, n));
      };
    h(L, (e) => {
      t.constructors[1] ? e(j) : e(k, !1);
    });
  }
  var C = Q(L, 2);
  {
    var z = (e) => {
      var s = tt(),
        n = U(s);
      {
        var d = (i) => {
          var _ = X();
          (Y(() => Z(_, m(I))), c(i, _));
        };
        h(n, (i) => {
          m(D) && i(d);
        });
      }
      (W(s), c(e, s));
    };
    h(C, (e) => {
      m(v) && e(z);
    });
  }
  (c(r, R), S());
}
const mt = $(rt),
  _t = [
    () =>
      a(
        () => import('../nodes/0.D9LH-Nwr.js'),
        __vite__mapDeps([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]),
        import.meta.url
      ),
    () =>
      a(() => import('../nodes/1.C5cr4cd-.js'), __vite__mapDeps([21, 2, 3, 4, 5]), import.meta.url),
    () =>
      a(
        () => import('../nodes/2.BYGjSIcc.js'),
        __vite__mapDeps([22, 2, 4, 5, 6, 7, 8, 23, 24, 19, 1, 15, 13]),
        import.meta.url
      ),
    () =>
      a(
        () => import('../nodes/3.CQbfRTZQ.js'),
        __vite__mapDeps([25, 2, 17, 18, 7, 8, 9, 10, 11, 12, 13, 23, 26, 24, 27, 28]),
        import.meta.url
      ),
    () =>
      a(() => import('../nodes/4.Bo_XCLGS.js'), __vite__mapDeps([29, 2, 6, 5]), import.meta.url),
    () =>
      a(
        () => import('../nodes/5.D2P2LPQx.js'),
        __vite__mapDeps([30, 2, 7, 9, 27, 14, 15, 13, 16, 17, 18]),
        import.meta.url
      ),
    () =>
      a(
        () => import('../nodes/6.CpG4EF_T.js'),
        __vite__mapDeps([31, 2, 7, 11, 26, 24, 17, 18]),
        import.meta.url
      ),
    () =>
      a(
        () => import('../nodes/7.CSkPQsMX.js'),
        __vite__mapDeps([32, 2, 7, 10, 19, 1, 15, 13, 16]),
        import.meta.url
      )
  ],
  ct = [],
  dt = {
    '/': [2],
    '/catalogue/new-model': [3],
    '/error': [4],
    '/my-collection': [5],
    '/my-depot': [6],
    '/my-wishlists': [7]
  },
  w = {
    handleError: ({ error: r }) => {
      console.error(r);
    },
    reroute: () => {},
    transport: {}
  },
  at = Object.fromEntries(Object.entries(w.transport).map(([r, t]) => [r, t.decode])),
  ut = Object.fromEntries(Object.entries(w.transport).map(([r, t]) => [r, t.encode])),
  lt = !1,
  ft = (r, t) => at[r](t);
export {
  ft as decode,
  at as decoders,
  dt as dictionary,
  ut as encoders,
  lt as hash,
  w as hooks,
  it as matchers,
  _t as nodes,
  mt as root,
  ct as server_loads
};
