import { g as d } from './8AwhvTxk.js';
const P = () => 'Saving...',
  x = () => 'Salvataggio...',
  Rt = (t = {}, e = {}) => ((e.locale ?? d()) === 'en' ? P() : x()),
  j = () => 'Saved',
  $ = () => 'Salvato',
  Ft = (t = {}, e = {}) => ((e.locale ?? d()) === 'en' ? j() : $()),
  k = () => 'Something went wrong',
  D = () => 'Qualcosa è andato storto',
  Pt = (t = {}, e = {}) => ((e.locale ?? d()) === 'en' ? k() : D()),
  b = () => 'Retry',
  q = () => 'Riprova',
  xt = (t = {}, e = {}) => ((e.locale ?? d()) === 'en' ? b() : q());
var C = Object.defineProperty,
  E = (t) => {
    throw TypeError(t);
  },
  L = (t, e, r) =>
    e in t ? C(t, e, { enumerable: !0, configurable: !0, writable: !0, value: r }) : (t[e] = r),
  s = (t, e, r) => L(t, typeof e != 'symbol' ? e + '' : e, r),
  K = (t, e, r) => e.has(t) || E('Cannot ' + r),
  g = (t, e, r) => (K(t, e, 'read from private field'), e.get(t)),
  G = (t, e, r) =>
    e.has(t)
      ? E('Cannot add the same private member more than once')
      : e instanceof WeakSet
        ? e.add(t)
        : e.set(t, r);
function jt(t) {
  return t == null ? [] : Array.isArray(t) ? t : [t];
}
var $t = (t) => t[0],
  kt = (t) => t[t.length - 1],
  Dt = (t, ...e) => t.concat(e),
  bt = (t, ...e) => t.filter((r) => !e.includes(r)),
  y = (t) => t?.constructor.name === 'Array',
  B = (t, e) => {
    if (t.length !== e.length) return !1;
    for (let r = 0; r < t.length; r++) if (!M(t[r], e[r])) return !1;
    return !0;
  },
  M = (t, e) => {
    if (Object.is(t, e)) return !0;
    if ((t == null && e != null) || (t != null && e == null)) return !1;
    if (typeof t?.isEqual == 'function' && typeof e?.isEqual == 'function') return t.isEqual(e);
    if (typeof t == 'function' && typeof e == 'function') return t.toString() === e.toString();
    if (y(t) && y(e)) return B(Array.from(t), Array.from(e));
    if (typeof t != 'object' || typeof e != 'object') return !1;
    const r = Object.keys(e ?? Object.create(null)),
      n = r.length;
    for (let o = 0; o < n; o++) if (!Reflect.has(t, r[o])) return !1;
    for (let o = 0; o < n; o++) {
      const i = r[o];
      if (!M(t[i], e[i])) return !1;
    }
    return !0;
  },
  W = (t) => t != null && typeof t == 'object',
  p = (t) => typeof t == 'string',
  qt = (t) => typeof t == 'function',
  z = (t, e) => Object.prototype.hasOwnProperty.call(t, e),
  U = (t) => Object.prototype.toString.call(t),
  S = Function.prototype.toString,
  Z = S.call(Object),
  V = (t) => {
    if (!W(t) || U(t) != '[object Object]' || J(t)) return !1;
    const e = Object.getPrototypeOf(t);
    if (e === null) return !0;
    const r = z(e, 'constructor') && e.constructor;
    return typeof r == 'function' && r instanceof r && S.call(r) == Z;
  },
  X = (t) => typeof t == 'object' && t !== null && '$$typeof' in t && 'props' in t,
  H = (t) => typeof t == 'object' && t !== null && '__v_isVNode' in t,
  J = (t) => X(t) || H(t),
  Ct = (t, ...e) => (typeof t == 'function' ? t(...e) : t) ?? void 0,
  Lt = (t) => t(),
  Q =
    (...t) =>
    (...e) => {
      t.forEach(function (r) {
        r?.(...e);
      });
    },
  Kt = (() => {
    let t = 0;
    return () => (t++, t.toString(36));
  })();
function Y(t) {
  if (!V(t) || t === void 0) return t;
  const e = Reflect.ownKeys(t).filter((n) => typeof n == 'string'),
    r = {};
  for (const n of e) {
    const o = t[n];
    o !== void 0 && (r[n] = Y(o));
  }
  return r;
}
function tt(t, e) {
  const r = {},
    n = {},
    o = new Set(e),
    i = Reflect.ownKeys(t);
  for (const a of i) o.has(a) ? (n[a] = t[a]) : (r[a] = t[a]);
  return [n, r];
}
var Gt = (t) =>
    function (r) {
      return tt(r, t);
    },
  u = () => performance.now(),
  f,
  et = class {
    constructor(t) {
      ((this.onTick = t),
        s(this, 'frameId', null),
        s(this, 'pausedAtMs', null),
        s(this, 'context'),
        s(this, 'cancelFrame', () => {
          this.frameId !== null && (cancelAnimationFrame(this.frameId), (this.frameId = null));
        }),
        s(this, 'setStartMs', (e) => {
          this.context.startMs = e;
        }),
        s(this, 'start', () => {
          if (this.frameId !== null) return;
          const e = u();
          (this.pausedAtMs !== null
            ? ((this.context.startMs += e - this.pausedAtMs), (this.pausedAtMs = null))
            : (this.context.startMs = e),
            (this.frameId = requestAnimationFrame(g(this, f))));
        }),
        s(this, 'pause', () => {
          this.frameId !== null && (this.cancelFrame(), (this.pausedAtMs = u()));
        }),
        s(this, 'stop', () => {
          this.frameId !== null && (this.cancelFrame(), (this.pausedAtMs = null));
        }),
        G(this, f, (e) => {
          if (
            ((this.context.now = e),
            (this.context.deltaMs = e - this.context.startMs),
            this.onTick(this.context) === !1)
          ) {
            this.stop();
            return;
          }
          this.frameId = requestAnimationFrame(g(this, f));
        }),
        (this.context = { now: 0, startMs: u(), deltaMs: 0 }));
    }
    get elapsedMs() {
      return this.pausedAtMs !== null
        ? this.pausedAtMs - this.context.startMs
        : u() - this.context.startMs;
    }
  };
f = new WeakMap();
function Bt(t, e) {
  const r = new et(({ deltaMs: n }) => {
    if (n >= e) return (t(), !1);
  });
  return (r.start(), () => r.stop());
}
function Wt(...t) {
  (t.length === 1 ? t[0] : t[1], t.length === 2 && t[0]);
}
function zt(t, e) {
  if (t == null) throw new Error(e());
}
function Ut(t, e, r) {
  let n = [];
  for (const o of e) t[o] == null && n.push(o);
  if (n.length > 0) throw new Error(`[zag-js${` > ${r}`}] missing required props: ${n.join(', ')}`);
}
var rt = Object.defineProperty,
  nt = (t, e, r) =>
    e in t ? rt(t, e, { enumerable: !0, configurable: !0, writable: !0, value: r }) : (t[e] = r),
  m = (t, e, r) => nt(t, typeof e != 'symbol' ? e + '' : e, r),
  h = (t) => typeof t == 'object' && t !== null,
  Zt = 2147483647,
  Vt = (t) => (t ? '' : void 0),
  ot = 1,
  st = 9,
  it = 11,
  _ = (t) => h(t) && t.nodeType === ot && typeof t.nodeName == 'string',
  at = (t) => h(t) && t.nodeType === st,
  ct = (t) => h(t) && t === t.window,
  ut = (t) => h(t) && t.nodeType !== void 0,
  lt = (t) => ut(t) && t.nodeType === it && 'host' in t;
function ft(t) {
  if (!t) return !1;
  const e = t.getRootNode();
  return I(e) === t;
}
function Xt(t, e) {
  if (!t || !e || !_(t) || !_(e)) return !1;
  const r = e.getRootNode?.();
  if (t === e || t.contains(e)) return !0;
  if (r && lt(r)) {
    let n = e;
    for (; n; ) {
      if (t === n) return !0;
      n = n.parentNode || n.host;
    }
  }
  return !1;
}
function dt(t) {
  return at(t) ? t : ct(t) ? t.document : (t?.ownerDocument ?? document);
}
function I(t) {
  let e = t.activeElement;
  for (; e?.shadowRoot; ) {
    const r = e.shadowRoot.activeElement;
    if (!r || r === e) break;
    e = r;
  }
  return e;
}
var O = () => typeof document < 'u';
function ht() {
  return navigator.userAgentData?.platform ?? navigator.platform;
}
var v = (t) => O() && t.test(ht()),
  pt = (t) => O() && t.test(navigator.vendor),
  mt = () => v(/^iPhone/i),
  vt = () => v(/^iPad/i) || (N() && navigator.maxTouchPoints > 1),
  gt = () => mt() || vt(),
  yt = () => N() || gt(),
  N = () => v(/^Mac/i),
  Ht = () => yt() && pt(/apple/i),
  _t = {
    Up: 'ArrowUp',
    Down: 'ArrowDown',
    Esc: 'Escape',
    ' ': 'Space',
    ',': 'Comma',
    Left: 'ArrowLeft',
    Right: 'ArrowRight'
  },
  A = { ArrowLeft: 'ArrowRight', ArrowRight: 'ArrowLeft' };
function Jt(t, e = {}) {
  const { dir: r = 'ltr', orientation: n = 'horizontal' } = e;
  let o = t.key;
  return ((o = _t[o] ?? o), r === 'rtl' && n === 'horizontal' && o in A && (o = A[o]), o);
}
var Qt = (t, e, r, n) => {
    const o = typeof t == 'function' ? t() : t;
    return (
      o?.addEventListener(e, r, n),
      () => {
        o?.removeEventListener(e, r, n);
      }
    );
  },
  At = class T {
    constructor() {
      (m(this, 'id', null),
        m(this, 'fn_cleanup'),
        m(this, 'cleanup', () => {
          this.cancel();
        }));
    }
    static create() {
      return new T();
    }
    request(e) {
      (this.cancel(),
        (this.id = globalThis.requestAnimationFrame(() => {
          ((this.id = null), (this.fn_cleanup = e?.()));
        })));
    }
    cancel() {
      (this.id !== null && (globalThis.cancelAnimationFrame(this.id), (this.id = null)),
        this.fn_cleanup?.(),
        (this.fn_cleanup = void 0));
    }
    isActive() {
      return this.id !== null;
    }
  };
function Yt(t) {
  const e = At.create();
  return (e.request(t), e.cleanup);
}
function te(t, e) {
  return Array.from(t?.querySelectorAll(e) ?? []);
}
var R = (t) => t.id;
function wt(t, e, r = R) {
  return t.find((n) => r(n) === e);
}
function F(t, e, r = R) {
  const n = wt(t, e, r);
  return n ? t.indexOf(n) : -1;
}
function ee(t, e, r = !0) {
  let n = F(t, e);
  return ((n = r ? (n + 1) % t.length : Math.min(n + 1, t.length - 1)), t[n]);
}
function re(t, e, r = !0) {
  let n = F(t, e);
  return n === -1
    ? r
      ? t[t.length - 1]
      : null
    : ((n = r ? (n - 1 + t.length) % t.length : Math.max(0, n - 1)), t[n]);
}
var Et = (...t) =>
    t
      .map((e) => e?.trim?.())
      .filter(Boolean)
      .join(' '),
  Mt = /((?:--)?(?:\w+-?)+)\s*:\s*([^;]*)/g,
  w = (t) => {
    const e = {};
    let r;
    for (; (r = Mt.exec(t)); ) e[r[1]] = r[2];
    return e;
  },
  St = (t, e) => {
    if (p(t)) {
      if (p(e)) return `${t};${e}`;
      t = w(t);
    } else p(e) && (e = w(e));
    return Object.assign({}, t ?? {}, e ?? {});
  };
function ne(...t) {
  let e = {};
  for (let r of t) {
    if (!r) continue;
    for (let o in e) {
      if (o.startsWith('on') && typeof e[o] == 'function' && typeof r[o] == 'function') {
        e[o] = Q(r[o], e[o]);
        continue;
      }
      if (o === 'className' || o === 'class') {
        e[o] = Et(e[o], r[o]);
        continue;
      }
      if (o === 'style') {
        e[o] = St(e[o], r[o]);
        continue;
      }
      e[o] = r[o] !== void 0 ? r[o] : e[o];
    }
    for (let o in r) e[o] === void 0 && (e[o] = r[o]);
    const n = Object.getOwnPropertySymbols(r);
    for (let o of n) e[o] = r[o];
  }
  return e;
}
function It() {
  return {
    and: (...t) =>
      function (r) {
        return t.every((n) => r.guard(n));
      },
    or: (...t) =>
      function (r) {
        return t.some((n) => r.guard(n));
      },
    not: (t) =>
      function (r) {
        return !r.guard(t);
      }
  };
}
function oe(t) {
  return t;
}
function se() {
  return {
    guards: It(),
    createMachine: (t) => t,
    choose: (t) =>
      function ({ choose: r }) {
        return r(t)?.actions;
      }
  };
}
var Ot = ((t) => (
    (t.NotStarted = 'Not Started'),
    (t.Started = 'Started'),
    (t.Stopped = 'Stopped'),
    t
  ))(Ot || {}),
  ie = '__init__';
function ae(t) {
  const e = () => t.getRootNode?.() ?? document,
    r = () => dt(e());
  return {
    ...t,
    getRootNode: e,
    getDoc: r,
    getWin: () => r().defaultView ?? window,
    getActiveElement: () => I(e()),
    isActiveElement: ft,
    getById: (a) => e().getElementById(a)
  };
}
var l = (t, e = []) => ({
    parts: (...r) => {
      if (Nt(e)) return l(t, r);
      throw new Error(
        'createAnatomy().parts(...) should only be called once. Did you mean to use .extendWith(...) ?'
      );
    },
    extendWith: (...r) => l(t, [...e, ...r]),
    omit: (...r) =>
      l(
        t,
        e.filter((n) => !r.includes(n))
      ),
    rename: (r) => l(r, e),
    keys: () => e,
    build: () =>
      [...new Set(e)].reduce(
        (r, n) =>
          Object.assign(r, {
            [n]: {
              selector: [
                `&[data-scope="${c(t)}"][data-part="${c(n)}"]`,
                `& [data-scope="${c(t)}"][data-part="${c(n)}"]`
              ].join(', '),
              attrs: { 'data-scope': c(t), 'data-part': c(n) }
            }
          }),
        {}
      )
  }),
  c = (t) =>
    t
      .replace(/([A-Z])([A-Z])/g, '$1-$2')
      .replace(/([a-z])([A-Z])/g, '$1-$2')
      .replace(/[\s_]+/g, '-')
      .toLowerCase(),
  Nt = (t) => t.length === 0;
export {
  At as A,
  ae as B,
  re as C,
  ee as D,
  kt as E,
  $t as F,
  Dt as G,
  bt as H,
  ie as I,
  te as J,
  Gt as K,
  Jt as L,
  Zt as M,
  Ht as N,
  Pt as a,
  Rt as b,
  xt as c,
  Ft as d,
  Xt as e,
  Y as f,
  Ct as g,
  Qt as h,
  qt as i,
  l as j,
  oe as k,
  It as l,
  Bt as m,
  Ut as n,
  Vt as o,
  ne as p,
  Lt as q,
  Yt as r,
  se as s,
  M as t,
  Kt as u,
  zt as v,
  Wt as w,
  Ot as x,
  jt as y,
  p as z
};
