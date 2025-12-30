import { T as r, H as t, P as o, o as n, Z as c, F as i, q as l } from './C2P8ifMu.js';
const a = { steam: l, diesel: i, electric: c, passenger: n, freight: o, heart: t, default: r };
function p(e) {
  const s = e.toLowerCase();
  return a[s] ?? a.default;
}
export { p as r };
