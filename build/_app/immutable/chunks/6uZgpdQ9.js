import { g as k } from './8AwhvTxk.js';
import {
  ao as W,
  ap as G,
  k as K,
  l as X,
  aq as Z,
  m as f,
  z as H,
  u as V,
  ar as T,
  n as Q,
  as as O,
  A as U,
  B as Y
} from './C2P8ifMu.js';
import {
  p as tt,
  q as et,
  i as b,
  t as nt,
  v as rt,
  x as w,
  y as st,
  z as ot,
  w as I,
  I as $,
  B as ct,
  f as at
} from './Cxbe8PBw.js';
const ut = () => 'Settings',
  it = () => 'Impostazioni',
  $t = (t = {}, n = {}) => ((n.locale ?? k()) === 'en' ? ut() : it()),
  lt = () => 'v.',
  ft = () => 'v.',
  Mt = (t = {}, n = {}) => ((n.locale ?? k()) === 'en' ? lt() : ft()),
  pt = () => 'Search...',
  gt = () => 'Cerca...',
  jt = (t = {}, n = {}) => ((n.locale ?? k()) === 'en' ? pt() : gt()),
  dt = () => 'Search...',
  _t = () => 'Cerca...',
  Vt = (t = {}, n = {}) => ((n.locale ?? k()) === 'en' ? dt() : _t()),
  ht = () => 'Start typing to search',
  mt = () => 'Inizia a digitare per cercare',
  qt = (t = {}, n = {}) => ((n.locale ?? k()) === 'en' ? ht() : mt());
function At(t) {
  const n = Symbol();
  return {
    key: n,
    consume() {
      return G(n) || t;
    },
    provide(e) {
      return W(n, e);
    }
  };
}
function yt(t) {
  return new Proxy(
    {},
    {
      get(n, e) {
        return e === 'style' ? (o) => t({ style: o }).style : t;
      }
    }
  );
}
var Pt = () => (t) => Array.from(new Set(t));
const L = {
  className: 'class',
  defaultChecked: 'checked',
  defaultValue: 'value',
  htmlFor: 'for',
  onBlur: 'onfocusout',
  onChange: 'oninput',
  onFocus: 'onfocusin',
  onDoubleClick: 'ondblclick'
};
function J(t) {
  let n = '';
  for (let e in t) {
    const o = t[e];
    o != null &&
      (e.startsWith('--') || (e = e.replace(/[A-Z]/g, (p) => `-${p.toLowerCase()}`)),
      (n += `${e}:${o};`));
  }
  return n;
}
const xt = new Set(
  'viewBox,className,preserveAspectRatio,fillRule,clipPath,clipRule,strokeWidth,strokeLinecap,strokeLinejoin,strokeDasharray,strokeDashoffset,strokeMiterlimit'.split(
    ','
  )
);
function St(t) {
  return t in L ? L[t] : xt.has(t) ? t : t.toLowerCase();
}
function vt(t, n) {
  return t === 'style' && typeof n == 'object' ? J(n) : n;
}
const Dt = yt((t) => {
    const n = {};
    for (const e in t) n[St(e)] = vt(e, t[e]);
    return n;
  }),
  bt = /((?:--)?(?:\w+-?)+)\s*:\s*([^;]*)/g,
  Ct = (t) => {
    const n = {};
    let e;
    for (; (e = bt.exec(t)); ) n[e[1]] = e[2];
    return n;
  };
function Ft(...t) {
  const n = [];
  for (const o of t) o && 'class' in o && o.class != null && n.push(o.class);
  const e = tt(...t);
  return (
    n.length > 0 && (e.class = n.length === 1 ? n[0] : n),
    'style' in e && (typeof e.style == 'string' && (e.style = Ct(e.style)), (e.style = J(e.style))),
    e
  );
}
function R(t) {
  const n = t().defaultValue ?? t().value,
    e = t().isEqual ?? Object.is;
  let o = K(X(n));
  const p = V(() => t().value !== void 0);
  let c = { current: Z(() => f(o)) },
    d = { current: void 0 };
  H(() => {
    const u = f(p) ? t().value : f(o);
    ((c = { current: u }), (d = { current: u }));
  });
  const S = (u) => {
    const g = b(u) ? u(c.current) : u,
      h = d.current;
    (t().debug && console.log(`[bindable > ${t().debug}] setValue`, { next: g, prev: h }),
      f(p) || Q(o, g, !0),
      e(g, h) || t().onChange?.(g, h));
  };
  function _() {
    return f(p) ? t().value : f(o);
  }
  return {
    initial: n,
    ref: c,
    get: _,
    set(u) {
      (t().sync ? T : et)(() => S(u));
    },
    invoke(u, g) {
      t().onChange?.(u, g);
    },
    hash(u) {
      return t().hash?.(u) ?? String(u);
    }
  };
}
R.cleanup = (t) => {
  O(() => t());
};
R.ref = (t) => {
  let n = t;
  return {
    get: () => n,
    set: (e) => {
      n = e;
    }
  };
};
function wt(t) {
  const n = { current: t };
  return {
    get(e) {
      return n.current[e];
    },
    set(e, o) {
      n.current[e] = o;
    }
  };
}
const M = (t) => (typeof t == 'function' ? t() : t),
  kt = (t, n) => {
    let e = [],
      o = !0;
    U(() => {
      if (o) {
        ((e = t.map((c) => M(c))), (o = !1));
        return;
      }
      let p = !1;
      for (let c = 0; c < t.length; c++)
        if (!nt(e[c], M(t[c]))) {
          p = !0;
          break;
        }
      p && ((e = t.map((c) => M(c))), n());
    });
  };
function j(t) {
  return b(t) ? t() : t;
}
function It(t, n) {
  const e = V(() => {
      const { id: r, ids: s, getRootNode: i } = j(n);
      return ct({ id: r, ids: s, getRootNode: i });
    }),
    o = (...r) => {
      t.debug && console.log(...r);
    },
    p = V(() => t.props?.({ props: at(j(n)), scope: f(e) }) ?? j(n)),
    c = zt(() => f(p)),
    d = t.context?.({
      prop: c,
      bindable: R,
      get scope() {
        return f(e);
      },
      flush: B,
      getContext() {
        return S;
      },
      getComputed() {
        return N;
      },
      getRefs() {
        return E;
      },
      getEvent() {
        return z();
      }
    }),
    S = {
      get(r) {
        return d?.[r].get();
      },
      set(r, s) {
        d?.[r].set(s);
      },
      initial(r) {
        return d?.[r].initial;
      },
      hash(r) {
        const s = d?.[r].get();
        return d?.[r].hash(s);
      }
    };
  let _ = new Map(),
    u = { current: null },
    g = { current: null },
    h = { current: { type: '' } };
  const z = () => ({
      ...h.current,
      current() {
        return h.current;
      },
      previous() {
        return g.current;
      }
    }),
    q = () => ({
      ...y,
      hasTag(r) {
        const s = y.get();
        return !!t.states[s]?.tags?.includes(r);
      },
      matches(...r) {
        const s = y.get();
        return r.includes(s);
      }
    }),
    E = wt(t.refs?.({ prop: c, context: S }) ?? {}),
    m = () => ({
      state: q(),
      context: S,
      event: z(),
      prop: c,
      send: F,
      action: v,
      guard: A,
      track: kt,
      refs: E,
      computed: N,
      flush: B,
      scope: f(e),
      choose: D
    }),
    v = (r) => {
      const s = b(r) ? r(m()) : r;
      if (!s) return;
      const i = s.map((a) => {
        const l = t.implementations?.actions?.[a];
        return (l || I(`[zag-js] No implementation found for action "${JSON.stringify(a)}"`), l);
      });
      for (const a of i) a?.(m());
    },
    A = (r) => (b(r) ? r(m()) : t.implementations?.guards?.[r](m())),
    P = (r) => {
      const s = b(r) ? r(m()) : r;
      if (!s) return;
      const i = s.map((l) => {
          const x = t.implementations?.effects?.[l];
          return (x || I(`[zag-js] No implementation found for effect "${JSON.stringify(l)}"`), x);
        }),
        a = [];
      for (const l of i) {
        const x = l?.(m());
        x && a.push(x);
      }
      return () => a.forEach((l) => l?.());
    },
    D = (r) =>
      st(r).find((s) => {
        let i = !s.guard;
        return (ot(s.guard) ? (i = !!A(s.guard)) : b(s.guard) && (i = s.guard(m())), i);
      }),
    N = (r) => {
      rt(t.computed, () => '[zag-js] No computed object found on machine');
      const s = t.computed[r];
      return s({ context: S, event: z(), prop: c, refs: E, scope: f(e), computed: N });
    },
    y = R(() => ({
      defaultValue: t.initialState({ prop: c }),
      onChange(r, s) {
        (s && (_.get(s)?.(), _.delete(s)), s && v(t.states[s]?.exit), v(u.current?.actions));
        const i = P(t.states[r]?.effects);
        if ((i && _.set(r, i), s === $)) {
          v(t.entry);
          const a = P(t.effects);
          a && _.set($, a);
        }
        v(t.states[r]?.entry);
      }
    }));
  let C = w.NotStarted;
  (Y(() => {
    const r = C === w.Started;
    ((C = w.Started), o(r ? 'rehydrating...' : 'initializing...'), y.invoke(y.initial, $));
  }),
    O(() => {
      (o('unmounting...'),
        (C = w.Stopped),
        _.forEach((r) => r?.()),
        (_ = new Map()),
        (u.current = null),
        v(t.exit));
    }));
  const F = (r) => {
    if (C !== w.Started) return;
    ((g.current = h.current), (h.current = r));
    let s = y.get();
    const i = t.states[s].on?.[r.type] ?? t.on?.[r.type],
      a = D(i);
    if (!a) return;
    u.current = a;
    const l = a.target ?? s;
    o('transition', r.type, a.target || s, `(${a.actions})`);
    const x = l !== s;
    x ? y.set(l) : a.reenter && !x ? y.invoke(s, s) : v(a.actions);
  };
  return (
    t.watch?.(m()),
    {
      get state() {
        return q();
      },
      send: F,
      context: S,
      prop: c,
      get scope() {
        return f(e);
      },
      refs: E,
      computed: N,
      get event() {
        return z();
      },
      getStatus: () => C
    }
  );
}
function zt(t) {
  return function (e) {
    return t()[e];
  };
}
function B(t) {
  T(() => {
    queueMicrotask(() => t());
  });
}
export { $t as a, Mt as b, At as c, jt as d, Vt as e, qt as f, Pt as g, Ft as m, Dt as n, It as u };
