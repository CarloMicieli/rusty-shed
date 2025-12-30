const f = ['en', 'it'],
  g = 'PARAGLIDE_LOCALE';
const m = ['cookie', 'globalVariable', 'baseLocale'];
globalThis.__paraglide = {};
let c,
  u = !1,
  w = () => {
    let e;
    for (const o of m) {
      if (o === 'cookie') e = C();
      else if (o === 'baseLocale') e = 'en';
      else if (o === 'globalVariable' && c !== void 0) e = c;
      else if (h(o) && i.has(o)) {
        const t = i.get(o);
        if (t) {
          const a = t.getLocale();
          if (a instanceof Promise) continue;
          e = a;
        }
      }
      if (e !== void 0) {
        const t = y(e);
        return (u || ((c = t), (u = !0), b(t, { reload: !1 })), t);
      }
    }
    throw new Error(
      'No locale found. Read the docs https://inlang.com/m/gerre34r/library-inlang-paraglideJs/errors#no-locale-found'
    );
  };
const p = (e) => {
  window.location.reload();
};
let b = (e, o) => {
  const t = { reload: !0, ...o };
  let a;
  try {
    a = w();
  } catch {}
  const l = [];
  for (const n of m)
    if (n === 'globalVariable') c = e;
    else if (n === 'cookie') {
      if (typeof document > 'u' || typeof window > 'u') continue;
      const r = `${g}=${e}; path=/; max-age=34560000`;
      document.cookie = r;
    } else {
      if (n === 'baseLocale') continue;
      if (h(n) && i.has(n)) {
        const r = i.get(n);
        if (r) {
          let s = r.setLocale(e);
          s instanceof Promise &&
            ((s = s.catch((L) => {
              throw new Error(`Custom strategy "${n}" setLocale failed.`, { cause: L });
            })),
            l.push(s));
        }
      }
    }
  const d = () => {
    t.reload && window.location && e !== a && p();
  };
  if (l.length)
    return Promise.all(l).then(() => {
      d();
    });
  d();
};
function k(e) {
  return typeof e != 'string' ? !1 : e ? f.some((o) => o.toLowerCase() === e.toLowerCase()) : !1;
}
function y(e) {
  if (typeof e != 'string') throw new Error(`Invalid locale: ${e}. Expected a string.`);
  const o = e.toLowerCase(),
    t = f.find((a) => a.toLowerCase() === o);
  if (!t) throw new Error(`Invalid locale: ${e}. Expected one of: ${f.join(', ')}`);
  return t;
}
function C() {
  if (typeof document > 'u' || !document.cookie) return;
  const o = document.cookie.match(new RegExp(`(^| )${g}=([^;]+)`))?.[2];
  if (k(o)) return o;
}
const i = new Map();
function h(e) {
  return typeof e == 'string' && /^custom-[A-Za-z0-9_-]+$/.test(e);
}
export { w as g };
