import {
  p as ze,
  E as me,
  a as Z,
  D as Ie,
  b as f,
  c as $e,
  m as _,
  u as y,
  au as ke,
  f as b,
  av as He,
  d as $,
  a9 as he,
  r as O,
  aw as Je,
  ax as We,
  aH as Xa,
  aA as Ns,
  y as et,
  i as Ds,
  aI as Zs,
  t as oe,
  s as v,
  aq as xs,
  aJ as Ca,
  j as tt,
  e as ie,
  L as Ya,
  R as q,
  V as Q,
  N as Nt,
  ab as Ls,
  aK as ka,
  k as $t,
  l as Ot,
  n as dt,
  U as Ms,
  W as Vs,
  X as js,
  aa as Fs,
  aj as Ks
} from '../chunks/C2P8ifMu.js';
import { commands as Us } from '../chunks/zzNGAeR9.js';
import { g as p } from '../chunks/8AwhvTxk.js';
import { b as Mt, a as Vt } from '../chunks/fcFX1gix.js';
import {
  f as jt,
  d as Ft,
  b as Kt,
  c as Ut,
  a as Gt,
  e as Bt,
  s as Gs
} from '../chunks/B5syxjUO.js';
import { a as Ht } from '../chunks/DCbz5KVu.js';
import { a as Jt } from '../chunks/Cai0cCrU.js';
import {
  f as Wt,
  e as qt,
  d as Xt,
  a as Yt,
  b as Qt,
  c as Qa,
  m as qe,
  g as es,
  u as Bs,
  n as Hs
} from '../chunks/6uZgpdQ9.js';
import {
  I as er,
  J as tr,
  h as rr,
  O as nr,
  K as or,
  L as ar,
  g as sr,
  f as ir,
  Q as cr,
  H as lr,
  E as ur,
  G as _r,
  F as dr,
  D as pr,
  b as mr,
  e as fr,
  d as gr,
  a as hr,
  c as vr,
  A as yr,
  z as br,
  B as wr,
  C as Ir,
  P as Er,
  N as Sr,
  M as Cr,
  y as kr,
  r as Ar,
  j as Tr,
  i as zr,
  v as $r,
  n as Or,
  o as Pr,
  u as Rr,
  m as Nr,
  p as Dr,
  q as Zr,
  t as xr,
  s as Lr,
  k as Mr,
  x as Vr,
  l as jr,
  w as Fr
} from '../chunks/DdIlMJnB.js';
import {
  s as Kr,
  l as Ur,
  c as Gr,
  r as Br,
  n as Hr,
  p as Jr,
  b as Wr,
  f as qr,
  o as Xr,
  m as Yr,
  d as Qr,
  e as en,
  k as tn,
  h as rn,
  j as nn,
  i as on,
  q as an,
  g as sn,
  a as cn
} from '../chunks/wxoaVA6c.js';
import { d as ln } from '../chunks/DCW-6SkD.js';
import {
  k as un,
  h as _n,
  a as dn,
  i as pn,
  j as mn,
  g as fn,
  d as gn,
  e as hn,
  c as vn,
  l as yn,
  b as bn,
  f as wn
} from '../chunks/BUXTf0nH.js';
import {
  a as In,
  b as En,
  c as Sn,
  d as Cn,
  k as Js,
  j as Ws,
  l as qs,
  w as Xs,
  C as Ys,
  D as Qs,
  E as ei,
  F as ti,
  G as ri,
  H as ni,
  J as oi,
  K as ts,
  L as ai,
  N as si,
  o as Fe
} from '../chunks/Cxbe8PBw.js';
const ii = (e) => `Hello world ${e?.username}`,
  ci = (e) => `Ciao mondo ${e?.username}`,
  kn = (e, t = {}) => ((t.locale ?? p()) === 'en' ? ii(e) : ci(e)),
  li = () => 'Edit item',
  ui = () => 'Modifica elemento',
  An = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? li() : ui()),
  _i = () => 'Undo',
  di = () => 'Annulla',
  Tn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? _i() : di()),
  pi = () => 'Add New Railway Model',
  mi = () => 'Aggiungi nuovo modello ferroviario',
  zn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? pi() : mi()),
  fi = () => 'Basic Information',
  gi = () => 'Informazioni di base',
  $n = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? fi() : gi()),
  hi = () => 'Manufacturer',
  vi = () => 'Produttore',
  On = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? hi() : vi()),
  yi = () => 'Product Code',
  bi = () => 'Codice prodotto',
  Pn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? yi() : bi()),
  wi = () => 'e.g., 37858',
  Ii = () => 'es. 37858',
  Rn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? wi() : Ii()),
  Ei = () => 'Description',
  Si = () => 'Descrizione',
  Nn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Ei() : Si()),
  Ci = () => 'e.g., Class 218 Diesel Locomotive',
  ki = () => 'es. Locomotiva diesel BR 218',
  Dn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Ci() : ki()),
  Ai = () => 'Category',
  Ti = () => 'Categoria',
  Zn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Ai() : Ti()),
  zi = () => 'Scale',
  $i = () => 'Scala',
  xn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? zi() : $i()),
  Oi = () => 'Power Method',
  Pi = () => 'Alimentazione',
  Ln = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Oi() : Pi()),
  Ri = () => 'Epoch',
  Ni = () => 'Epoca',
  Mn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Ri() : Ni()),
  Di = () => '-- Select --',
  Zi = () => '-- Seleziona --',
  Vn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Di() : Zi()),
  xi = () => 'Delivery & Availability',
  Li = () => 'Consegna e disponibilità',
  jn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? xi() : Li()),
  Mi = () => 'Delivery Date',
  Vi = () => 'Data di consegna',
  Fn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Mi() : Vi()),
  ji = () => '2025, 2025/06, or 2025/Q2',
  Fi = () => '2025, 2025/06 o 2025/T2',
  Kn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? ji() : Fi()),
  Ki = () => 'Availability Status',
  Ui = () => 'Stato disponibilità',
  Un = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Ki() : Ui()),
  Gi = () => 'Additional Details',
  Bi = () => 'Dettagli aggiuntivi',
  Gn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Gi() : Bi()),
  Hi = () => 'Optional notes about this model...',
  Ji = () => 'Note facoltative su questo modello...',
  Bn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Hi() : Ji()),
  Wi = () => 'Rolling Stock',
  qi = () => 'Materiale rotabile',
  Hn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Wi() : qi()),
  Xi = () => 'Railway Company',
  Yi = () => 'Impresa ferroviaria',
  Jn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Xi() : Yi()),
  Qi = () => 'Rolling Stock Category',
  ec = () => 'Categoria materiale rotabile',
  Wn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Qi() : ec()),
  tc = () => 'Livery',
  rc = () => 'Livrea',
  qn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? tc() : rc()),
  nc = () => 'e.g., Deutsche Bahn AG',
  oc = () => 'es. Deutsche Bahn AG',
  Xn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? nc() : oc()),
  ac = () => 'Class Name',
  sc = () => 'Classificazione',
  Yn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? ac() : sc()),
  ic = () => 'Road Number',
  cc = () => 'Numero di servizio',
  Qn = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? ic() : cc()),
  lc = () => 'Series',
  uc = () => 'Serie',
  eo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? lc() : uc()),
  _c = () => 'Depot',
  dc = () => 'Deposito',
  to = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? _c() : dc()),
  pc = () => 'Type',
  mc = () => 'Tipo',
  ro = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? pc() : mc()),
  fc = () => 'Type Name',
  gc = () => 'Tipo',
  no = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? fc() : gc()),
  hc = () => 'Passenger Car Type',
  vc = () => 'Tipo carrozza passeggeri',
  oo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? hc() : vc()),
  yc = () => 'Freight Car Type',
  bc = () => 'Tipo carro merci',
  ao = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? yc() : bc()),
  wc = () => 'EMU Type',
  Ic = () => 'Tipo elettrotreno',
  so = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? wc() : Ic()),
  Ec = () => 'Is Dummy',
  Sc = () => 'Modello fittizio',
  io = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Ec() : Sc()),
  Cc = () => 'Technical Details',
  kc = () => 'Dettagli tecnici',
  co = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Cc() : kc()),
  Ac = () => 'Control',
  Tc = () => 'Controllo',
  lo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Ac() : Tc()),
  zc = () => 'DCC Interface',
  $c = () => 'Interfaccia DCC',
  uo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? zc() : $c()),
  Oc = () => 'Service Level',
  Pc = () => 'Classe di servizio',
  _o = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Oc() : Pc()),
  Rc = () => 'Duplicate',
  Nc = () => 'Duplica',
  po = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Rc() : Nc()),
  Dc = () => 'Delete',
  Zc = () => 'Elimina',
  mo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Dc() : Zc()),
  xc = () => 'Add Rolling Stock',
  Lc = () => 'Aggiungi materiale rotabile',
  fo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? xc() : Lc()),
  Mc = () => 'Create Railway Model',
  Vc = () => 'Crea modello ferroviario',
  go = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Mc() : Vc()),
  jc = () => 'Cancel',
  Fc = () => 'Annulla',
  ho = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? jc() : Fc()),
  Kc = () => 'Locomotives',
  Uc = () => 'Locomotive',
  vo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Kc() : Uc()),
  Gc = () => 'Train sets',
  Bc = () => 'Coffret treno',
  yo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Gc() : Bc()),
  Hc = () => 'Starter sets',
  Jc = () => 'Starter set',
  bo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Hc() : Jc()),
  Wc = () => 'Freight cars',
  qc = () => 'Carri merci',
  wo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Wc() : qc()),
  Xc = () => 'Passenger cars',
  Yc = () => 'Carrozze passeggeri',
  Io = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Xc() : Yc()),
  Qc = () => 'Electric multiple units',
  el = () => 'Elettrotreni',
  Eo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Qc() : el()),
  tl = () => 'Railcars',
  rl = () => 'Automotrici',
  So = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? tl() : rl()),
  nl = () => 'Announced',
  ol = () => 'Annunciato',
  Co = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? nl() : ol()),
  al = () => 'Available',
  sl = () => 'Disponibile',
  ko = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? al() : sl()),
  il = () => 'Cancelled',
  cl = () => 'Cancellato',
  Ao = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? il() : cl()),
  ll = () => 'Discontinued',
  ul = () => 'Fuori produzione',
  To = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? ll() : ul()),
  _l = () => 'Steam locomotive',
  dl = () => 'Locomotiva a vapore',
  zo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? _l() : dl()),
  pl = () => 'Diesel locomotive',
  ml = () => 'Locomotiva diesel',
  $o = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? pl() : ml()),
  fl = () => 'Electric locomotive',
  gl = () => 'Locomotiva elettrica',
  Oo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? fl() : gl()),
  hl = () => 'Baggage car',
  vl = () => 'Vagone bagagliaio',
  Po = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? hl() : vl()),
  yl = () => 'Buffet car',
  bl = () => 'Vagone buffet',
  Ro = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? yl() : bl()),
  wl = () => 'Combine car',
  Il = () => 'Vagone combinato',
  No = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? wl() : Il()),
  El = () => 'Compartment coach',
  Sl = () => 'Carrozza a scompartimenti',
  Do = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? El() : Sl()),
  Cl = () => 'Dining car',
  kl = () => 'Vagone ristorante',
  Zo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Cl() : kl()),
  Al = () => 'Double-decker',
  Tl = () => 'Vagone a due piani',
  xo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Al() : Tl()),
  zl = () => 'Dome car',
  $l = () => 'Carrozza panoramica',
  Lo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? zl() : $l()),
  Ol = () => 'Driving trailer',
  Pl = () => 'Semipilota',
  Mo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Ol() : Pl()),
  Rl = () => 'Lounge',
  Nl = () => 'Vagone lounge',
  Vo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Rl() : Nl()),
  Dl = () => 'Observation car',
  Zl = () => 'Vagone osservazione',
  jo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Dl() : Zl()),
  xl = () => 'Open coach',
  Ll = () => 'Carrozza a salone',
  Fo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? xl() : Ll()),
  Ml = () => 'Railway post office',
  Vl = () => 'Ufficio postale ferroviario',
  Ko = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Ml() : Vl()),
  jl = () => 'Sleeping car',
  Fl = () => 'Vagone letto',
  Uo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? jl() : Fl()),
  Kl = () => 'Sleeperette',
  Ul = () => 'Cuccette',
  Go = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Kl() : Ul()),
  Gl = () => 'Auto transport cars',
  Bl = () => 'Carri trasporto auto',
  Bo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Gl() : Bl()),
  Hl = () => 'Brake wagon',
  Jl = () => 'Carro freno',
  Ho = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Hl() : Jl()),
  Wl = () => 'Container cars',
  ql = () => 'Porta container',
  Jo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Wl() : ql()),
  Xl = () => 'Covered freight cars',
  Yl = () => 'Carri chiusi',
  Wo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Xl() : Yl()),
  Ql = () => 'Deep well flat cars',
  eu = () => 'Carri tasca',
  qo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Ql() : eu()),
  tu = () => 'Dump cars',
  ru = () => 'Carri ribaltabili',
  Xo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? tu() : ru()),
  nu = () => 'Gondola',
  ou = () => 'Gondola',
  Yo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? nu() : ou()),
  au = () => 'Heavy goods wagons',
  su = () => 'Carri per carichi pesanti',
  Qo = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? au() : su()),
  iu = () => 'Hinged cover wagons',
  cu = () => 'Carri a tetto incernierato',
  ea = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? iu() : cu()),
  lu = () => 'Hopper wagon',
  uu = () => 'Carri tramoggia',
  ta = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? lu() : uu()),
  _u = () => 'Refrigerator cars',
  du = () => 'Carri frigoriferi',
  ra = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? _u() : du()),
  pu = () => 'Silo container cars',
  mu = () => 'Carri silo',
  na = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? pu() : mu()),
  fu = () => 'Slide tarpaulin wagon',
  gu = () => 'Carro a telone scorrevole',
  oa = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? fu() : gu()),
  hu = () => 'Sliding wall boxcars',
  vu = () => 'Carri a pareti scorrevoli',
  aa = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? hu() : vu()),
  yu = () => 'Special transport',
  bu = () => 'Trasporti speciali',
  sa = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? yu() : bu()),
  wu = () => 'Stake wagons',
  Iu = () => 'Carri a sponde',
  ia = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? wu() : Iu()),
  Eu = () => 'Swing roof wagon',
  Su = () => 'Carri a tetto basculante',
  ca = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Eu() : Su()),
  Cu = () => 'Tank cars',
  ku = () => 'Carri cisterna',
  la = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Cu() : ku()),
  Au = () => 'Telescope hood wagons',
  Tu = () => 'Carri a mantice telescopico',
  ua = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Au() : Tu()),
  zu = () => 'Driving car',
  $u = () => 'Carrozza pilota',
  _a = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? zu() : $u()),
  Ou = () => 'High-speed train',
  Pu = () => 'Treno ad alta velocità',
  da = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Ou() : Pu()),
  Ru = () => 'Motor car',
  Nu = () => 'Carrozza motrice',
  pa = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Ru() : Nu()),
  Du = () => 'Power car',
  Zu = () => 'Carrozza di potenza',
  ma = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Du() : Zu()),
  xu = () => 'Trailer car',
  Lu = () => 'Carrozza rimorchiata',
  fa = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? xu() : Lu()),
  Mu = () => 'Train set',
  Vu = () => 'Convoglio',
  ga = (e = {}, t = {}) => ((t.locale ?? p()) === 'en' ? Mu() : Vu()),
  ju = Object.freeze(
    Object.defineProperty(
      {
        __proto__: null,
        actions_add_railway_model: er,
        actions_add_wishlist_item: tr,
        app_collection: jt,
        app_dashboard: Mt,
        app_depot: Jt,
        app_name: Vt,
        app_search_instruction: Wt,
        app_search_mobile_placeholder: qt,
        app_search_placeholder: Xt,
        app_settings: Yt,
        app_version_prefix: Qt,
        app_wishlists: Ht,
        collection_add_first: un,
        collection_add_item: _n,
        collection_clear_filters: dn,
        collection_confirm_delete: pn,
        collection_delete_item: mn,
        collection_edit_item: An,
        collection_empty_caption: fn,
        collection_filter_scales: gn,
        collection_filter_tags: hn,
        collection_filters_title: vn,
        collection_no_results: yn,
        collection_search_placeholder: bn,
        collection_tag_diesel: Ft,
        collection_tag_electric: Kt,
        collection_tag_freight: Ut,
        collection_tag_passenger: Gt,
        collection_tag_steam: Bt,
        collection_title: wn,
        collection_toast_error: In,
        collection_toast_loading: En,
        collection_toast_retry: Sn,
        collection_toast_success: Cn,
        collection_toast_undo: Tn,
        constants_availability_status_announced: Co,
        constants_availability_status_available: ko,
        constants_availability_status_cancelled: Ao,
        constants_availability_status_discontinued: To,
        constants_categories_electric_multiple_units: Eo,
        constants_categories_freight_cars: wo,
        constants_categories_locomotives: vo,
        constants_categories_passenger_cars: Io,
        constants_categories_railcars: So,
        constants_categories_starter_sets: bo,
        constants_categories_train_sets: yo,
        constants_electric_multiple_unit_types_driving_car: _a,
        constants_electric_multiple_unit_types_high_speed_train: da,
        constants_electric_multiple_unit_types_motor_car: pa,
        constants_electric_multiple_unit_types_power_car: ma,
        constants_electric_multiple_unit_types_trailer_car: fa,
        constants_electric_multiple_unit_types_train_set: ga,
        constants_freight_car_types_auto_transport_cars: Bo,
        constants_freight_car_types_brake_wagon: Ho,
        constants_freight_car_types_container_cars: Jo,
        constants_freight_car_types_covered_freight_cars: Wo,
        constants_freight_car_types_deep_well_flat_cars: qo,
        constants_freight_car_types_dump_cars: Xo,
        constants_freight_car_types_gondola: Yo,
        constants_freight_car_types_heavy_goods_wagons: Qo,
        constants_freight_car_types_hinged_cover_wagons: ea,
        constants_freight_car_types_hopper_wagon: ta,
        constants_freight_car_types_refrigerator_cars: ra,
        constants_freight_car_types_silo_container_cars: na,
        constants_freight_car_types_slide_tarpaulin_wagon: oa,
        constants_freight_car_types_sliding_wall_boxcars: aa,
        constants_freight_car_types_special_transport: sa,
        constants_freight_car_types_stake_wagons: ia,
        constants_freight_car_types_swing_roof_wagon: ca,
        constants_freight_car_types_tank_cars: la,
        constants_freight_car_types_telescope_hood_wagons: ua,
        constants_locomotive_types_diesel_locomotive: $o,
        constants_locomotive_types_electric_locomotive: Oo,
        constants_locomotive_types_steam_locomotive: zo,
        constants_passenger_car_types_baggage_car: Po,
        constants_passenger_car_types_buffet_car: Ro,
        constants_passenger_car_types_combine_car: No,
        constants_passenger_car_types_compartment_coach: Do,
        constants_passenger_car_types_dining_car: Zo,
        constants_passenger_car_types_dome_car: Lo,
        constants_passenger_car_types_double_decker: xo,
        constants_passenger_car_types_driving_trailer: Mo,
        constants_passenger_car_types_lounge: Vo,
        constants_passenger_car_types_observation: jo,
        constants_passenger_car_types_open_coach: Fo,
        constants_passenger_car_types_railway_post_office: Ko,
        constants_passenger_car_types_sleeperette: Go,
        constants_passenger_car_types_sleeping_car: Uo,
        dashboard_add_first_model: rr,
        dashboard_due: nr,
        dashboard_due_soon: or,
        dashboard_empty_depot: ar,
        dashboard_empty_depot_message: sr,
        dashboard_empty_depot_title: ir,
        dashboard_empty_recent: cr,
        dashboard_quick_actions: lr,
        dashboard_recently_added: ur,
        dashboard_the_depot: _r,
        dashboard_view_all: dr,
        dashboard_yard_statistics: pr,
        depot_cars_title: Kr,
        depot_category: mr,
        depot_clear_search: Ur,
        depot_company: ln,
        depot_dcc_address: Gr,
        depot_description: fr,
        depot_empty_cars: Br,
        depot_empty_locomotives: Hr,
        depot_empty_trains: Jr,
        depot_group: Wr,
        depot_livery: qr,
        depot_locomotives_title: Xr,
        depot_manufacturer: gr,
        depot_no_results: Yr,
        depot_overflow_note: Qr,
        depot_product_code: hr,
        depot_road_number: en,
        depot_scale: vr,
        depot_search_placeholder: tn,
        depot_service_level: rn,
        depot_subtitle: nn,
        depot_title: on,
        depot_trains_title: an,
        depot_type: sn,
        depot_view_all: cn,
        errors_dashboard_message: yr,
        errors_dashboard_title: br,
        errors_retry_page: wr,
        errors_return_dashboard: Ir,
        example_message: kn,
        form_new_model_add_rolling_stock: fo,
        form_new_model_additional_details: Gn,
        form_new_model_availability_status: Un,
        form_new_model_basic_info: $n,
        form_new_model_cancel: ho,
        form_new_model_category: Zn,
        form_new_model_class_name: Yn,
        form_new_model_control: lo,
        form_new_model_create: go,
        form_new_model_dcc_interface: uo,
        form_new_model_delete: mo,
        form_new_model_delivery_availability: jn,
        form_new_model_delivery_date: Fn,
        form_new_model_delivery_date_placeholder: Kn,
        form_new_model_depot: to,
        form_new_model_description: Nn,
        form_new_model_description_placeholder: Dn,
        form_new_model_details_placeholder: Bn,
        form_new_model_duplicate: po,
        form_new_model_emu_type: so,
        form_new_model_epoch: Mn,
        form_new_model_freight_car_type: ao,
        form_new_model_is_dummy: io,
        form_new_model_livery: qn,
        form_new_model_livery_placeholder: Xn,
        form_new_model_manufacturer: On,
        form_new_model_passenger_car_type: oo,
        form_new_model_power_method: Ln,
        form_new_model_product_code: Pn,
        form_new_model_product_code_placeholder: Rn,
        form_new_model_railway_company: Jn,
        form_new_model_road_number: Qn,
        form_new_model_rolling_stock: Hn,
        form_new_model_rolling_stock_category: Wn,
        form_new_model_scale: xn,
        form_new_model_select_placeholder: Vn,
        form_new_model_series: eo,
        form_new_model_service_level: _o,
        form_new_model_technical_details: co,
        form_new_model_title: zn,
        form_new_model_type: ro,
        form_new_model_type_name: no,
        stats_maintenance_alerts: Er,
        stats_rolling_stocks: Sr,
        stats_total_collection_value: Cr,
        wishlist_modal_add_failed: kr,
        wishlist_modal_cancel: Ar,
        wishlist_modal_choose_or_create: Tr,
        wishlist_modal_close: zr,
        wishlist_modal_create_failed: $r,
        wishlist_modal_item_id_label: Or,
        wishlist_modal_item_id_placeholder: Pr,
        wishlist_modal_missing_model: Rr,
        wishlist_modal_new_list_placeholder: Nr,
        wishlist_modal_notes_label: Dr,
        wishlist_modal_notes_placeholder: Zr,
        wishlist_modal_save: xr,
        wishlist_modal_saving: Lr,
        wishlist_modal_select_list: Mr,
        wishlist_modal_select_list_error: Vr,
        wishlist_modal_select_placeholder: jr,
        wishlist_modal_title: Fr
      },
      Symbol.toStringTag,
      { value: 'Module' }
    )
  ),
  Fu = Object.freeze(
    Object.defineProperty(
      {
        __proto__: null,
        actions_add_railway_model: er,
        actions_add_wishlist_item: tr,
        app_collection: jt,
        app_dashboard: Mt,
        app_depot: Jt,
        app_name: Vt,
        app_search_instruction: Wt,
        app_search_mobile_placeholder: qt,
        app_search_placeholder: Xt,
        app_settings: Yt,
        app_version_prefix: Qt,
        app_wishlists: Ht,
        collection_add_first: un,
        collection_add_item: _n,
        collection_clear_filters: dn,
        collection_confirm_delete: pn,
        collection_delete_item: mn,
        collection_edit_item: An,
        collection_empty_caption: fn,
        collection_filter_scales: gn,
        collection_filter_tags: hn,
        collection_filters_title: vn,
        collection_no_results: yn,
        collection_search_placeholder: bn,
        collection_tag_diesel: Ft,
        collection_tag_electric: Kt,
        collection_tag_freight: Ut,
        collection_tag_passenger: Gt,
        collection_tag_steam: Bt,
        collection_title: wn,
        collection_toast_error: In,
        collection_toast_loading: En,
        collection_toast_retry: Sn,
        collection_toast_success: Cn,
        collection_toast_undo: Tn,
        constants_availability_status_announced: Co,
        constants_availability_status_available: ko,
        constants_availability_status_cancelled: Ao,
        constants_availability_status_discontinued: To,
        constants_categories_electric_multiple_units: Eo,
        constants_categories_freight_cars: wo,
        constants_categories_locomotives: vo,
        constants_categories_passenger_cars: Io,
        constants_categories_railcars: So,
        constants_categories_starter_sets: bo,
        constants_categories_train_sets: yo,
        constants_electric_multiple_unit_types_driving_car: _a,
        constants_electric_multiple_unit_types_high_speed_train: da,
        constants_electric_multiple_unit_types_motor_car: pa,
        constants_electric_multiple_unit_types_power_car: ma,
        constants_electric_multiple_unit_types_trailer_car: fa,
        constants_electric_multiple_unit_types_train_set: ga,
        constants_freight_car_types_auto_transport_cars: Bo,
        constants_freight_car_types_brake_wagon: Ho,
        constants_freight_car_types_container_cars: Jo,
        constants_freight_car_types_covered_freight_cars: Wo,
        constants_freight_car_types_deep_well_flat_cars: qo,
        constants_freight_car_types_dump_cars: Xo,
        constants_freight_car_types_gondola: Yo,
        constants_freight_car_types_heavy_goods_wagons: Qo,
        constants_freight_car_types_hinged_cover_wagons: ea,
        constants_freight_car_types_hopper_wagon: ta,
        constants_freight_car_types_refrigerator_cars: ra,
        constants_freight_car_types_silo_container_cars: na,
        constants_freight_car_types_slide_tarpaulin_wagon: oa,
        constants_freight_car_types_sliding_wall_boxcars: aa,
        constants_freight_car_types_special_transport: sa,
        constants_freight_car_types_stake_wagons: ia,
        constants_freight_car_types_swing_roof_wagon: ca,
        constants_freight_car_types_tank_cars: la,
        constants_freight_car_types_telescope_hood_wagons: ua,
        constants_locomotive_types_diesel_locomotive: $o,
        constants_locomotive_types_electric_locomotive: Oo,
        constants_locomotive_types_steam_locomotive: zo,
        constants_passenger_car_types_baggage_car: Po,
        constants_passenger_car_types_buffet_car: Ro,
        constants_passenger_car_types_combine_car: No,
        constants_passenger_car_types_compartment_coach: Do,
        constants_passenger_car_types_dining_car: Zo,
        constants_passenger_car_types_dome_car: Lo,
        constants_passenger_car_types_double_decker: xo,
        constants_passenger_car_types_driving_trailer: Mo,
        constants_passenger_car_types_lounge: Vo,
        constants_passenger_car_types_observation: jo,
        constants_passenger_car_types_open_coach: Fo,
        constants_passenger_car_types_railway_post_office: Ko,
        constants_passenger_car_types_sleeperette: Go,
        constants_passenger_car_types_sleeping_car: Uo,
        dashboard_add_first_model: rr,
        dashboard_due: nr,
        dashboard_due_soon: or,
        dashboard_empty_depot: ar,
        dashboard_empty_depot_message: sr,
        dashboard_empty_depot_title: ir,
        dashboard_empty_recent: cr,
        dashboard_quick_actions: lr,
        dashboard_recently_added: ur,
        dashboard_the_depot: _r,
        dashboard_view_all: dr,
        dashboard_yard_statistics: pr,
        depot_cars_title: Kr,
        depot_category: mr,
        depot_clear_search: Ur,
        depot_company: ln,
        depot_dcc_address: Gr,
        depot_description: fr,
        depot_empty_cars: Br,
        depot_empty_locomotives: Hr,
        depot_empty_trains: Jr,
        depot_group: Wr,
        depot_livery: qr,
        depot_locomotives_title: Xr,
        depot_manufacturer: gr,
        depot_no_results: Yr,
        depot_overflow_note: Qr,
        depot_product_code: hr,
        depot_road_number: en,
        depot_scale: vr,
        depot_search_placeholder: tn,
        depot_service_level: rn,
        depot_subtitle: nn,
        depot_title: on,
        depot_trains_title: an,
        depot_type: sn,
        depot_view_all: cn,
        errors_dashboard_message: yr,
        errors_dashboard_title: br,
        errors_retry_page: wr,
        errors_return_dashboard: Ir,
        example_message: kn,
        form_new_model_add_rolling_stock: fo,
        form_new_model_additional_details: Gn,
        form_new_model_availability_status: Un,
        form_new_model_basic_info: $n,
        form_new_model_cancel: ho,
        form_new_model_category: Zn,
        form_new_model_class_name: Yn,
        form_new_model_control: lo,
        form_new_model_create: go,
        form_new_model_dcc_interface: uo,
        form_new_model_delete: mo,
        form_new_model_delivery_availability: jn,
        form_new_model_delivery_date: Fn,
        form_new_model_delivery_date_placeholder: Kn,
        form_new_model_depot: to,
        form_new_model_description: Nn,
        form_new_model_description_placeholder: Dn,
        form_new_model_details_placeholder: Bn,
        form_new_model_duplicate: po,
        form_new_model_emu_type: so,
        form_new_model_epoch: Mn,
        form_new_model_freight_car_type: ao,
        form_new_model_is_dummy: io,
        form_new_model_livery: qn,
        form_new_model_livery_placeholder: Xn,
        form_new_model_manufacturer: On,
        form_new_model_passenger_car_type: oo,
        form_new_model_power_method: Ln,
        form_new_model_product_code: Pn,
        form_new_model_product_code_placeholder: Rn,
        form_new_model_railway_company: Jn,
        form_new_model_road_number: Qn,
        form_new_model_rolling_stock: Hn,
        form_new_model_rolling_stock_category: Wn,
        form_new_model_scale: xn,
        form_new_model_select_placeholder: Vn,
        form_new_model_series: eo,
        form_new_model_service_level: _o,
        form_new_model_technical_details: co,
        form_new_model_title: zn,
        form_new_model_type: ro,
        form_new_model_type_name: no,
        m: ju,
        stats_maintenance_alerts: Er,
        stats_rolling_stocks: Sr,
        stats_total_collection_value: Cr,
        wishlist_modal_add_failed: kr,
        wishlist_modal_cancel: Ar,
        wishlist_modal_choose_or_create: Tr,
        wishlist_modal_close: zr,
        wishlist_modal_create_failed: $r,
        wishlist_modal_item_id_label: Or,
        wishlist_modal_item_id_placeholder: Pr,
        wishlist_modal_missing_model: Rr,
        wishlist_modal_new_list_placeholder: Nr,
        wishlist_modal_notes_label: Dr,
        wishlist_modal_notes_placeholder: Zr,
        wishlist_modal_save: xr,
        wishlist_modal_saving: Lr,
        wishlist_modal_select_list: Mr,
        wishlist_modal_select_list_error: Vr,
        wishlist_modal_select_placeholder: jr,
        wishlist_modal_title: Fr
      },
      Symbol.toStringTag,
      { value: 'Module' }
    )
  ),
  Ku = Object.freeze(
    Object.defineProperty(
      {
        __proto__: null,
        actions_add_railway_model: er,
        actions_add_wishlist_item: tr,
        app_collection: jt,
        app_dashboard: Mt,
        app_depot: Jt,
        app_name: Vt,
        app_search_instruction: Wt,
        app_search_mobile_placeholder: qt,
        app_search_placeholder: Xt,
        app_settings: Yt,
        app_version_prefix: Qt,
        app_wishlists: Ht,
        collection_add_first: un,
        collection_add_item: _n,
        collection_clear_filters: dn,
        collection_confirm_delete: pn,
        collection_delete_item: mn,
        collection_edit_item: An,
        collection_empty_caption: fn,
        collection_filter_scales: gn,
        collection_filter_tags: hn,
        collection_filters_title: vn,
        collection_no_results: yn,
        collection_search_placeholder: bn,
        collection_tag_diesel: Ft,
        collection_tag_electric: Kt,
        collection_tag_freight: Ut,
        collection_tag_passenger: Gt,
        collection_tag_steam: Bt,
        collection_title: wn,
        collection_toast_error: In,
        collection_toast_loading: En,
        collection_toast_retry: Sn,
        collection_toast_success: Cn,
        collection_toast_undo: Tn,
        constants_availability_status_announced: Co,
        constants_availability_status_available: ko,
        constants_availability_status_cancelled: Ao,
        constants_availability_status_discontinued: To,
        constants_categories_electric_multiple_units: Eo,
        constants_categories_freight_cars: wo,
        constants_categories_locomotives: vo,
        constants_categories_passenger_cars: Io,
        constants_categories_railcars: So,
        constants_categories_starter_sets: bo,
        constants_categories_train_sets: yo,
        constants_electric_multiple_unit_types_driving_car: _a,
        constants_electric_multiple_unit_types_high_speed_train: da,
        constants_electric_multiple_unit_types_motor_car: pa,
        constants_electric_multiple_unit_types_power_car: ma,
        constants_electric_multiple_unit_types_trailer_car: fa,
        constants_electric_multiple_unit_types_train_set: ga,
        constants_freight_car_types_auto_transport_cars: Bo,
        constants_freight_car_types_brake_wagon: Ho,
        constants_freight_car_types_container_cars: Jo,
        constants_freight_car_types_covered_freight_cars: Wo,
        constants_freight_car_types_deep_well_flat_cars: qo,
        constants_freight_car_types_dump_cars: Xo,
        constants_freight_car_types_gondola: Yo,
        constants_freight_car_types_heavy_goods_wagons: Qo,
        constants_freight_car_types_hinged_cover_wagons: ea,
        constants_freight_car_types_hopper_wagon: ta,
        constants_freight_car_types_refrigerator_cars: ra,
        constants_freight_car_types_silo_container_cars: na,
        constants_freight_car_types_slide_tarpaulin_wagon: oa,
        constants_freight_car_types_sliding_wall_boxcars: aa,
        constants_freight_car_types_special_transport: sa,
        constants_freight_car_types_stake_wagons: ia,
        constants_freight_car_types_swing_roof_wagon: ca,
        constants_freight_car_types_tank_cars: la,
        constants_freight_car_types_telescope_hood_wagons: ua,
        constants_locomotive_types_diesel_locomotive: $o,
        constants_locomotive_types_electric_locomotive: Oo,
        constants_locomotive_types_steam_locomotive: zo,
        constants_passenger_car_types_baggage_car: Po,
        constants_passenger_car_types_buffet_car: Ro,
        constants_passenger_car_types_combine_car: No,
        constants_passenger_car_types_compartment_coach: Do,
        constants_passenger_car_types_dining_car: Zo,
        constants_passenger_car_types_dome_car: Lo,
        constants_passenger_car_types_double_decker: xo,
        constants_passenger_car_types_driving_trailer: Mo,
        constants_passenger_car_types_lounge: Vo,
        constants_passenger_car_types_observation: jo,
        constants_passenger_car_types_open_coach: Fo,
        constants_passenger_car_types_railway_post_office: Ko,
        constants_passenger_car_types_sleeperette: Go,
        constants_passenger_car_types_sleeping_car: Uo,
        dashboard_add_first_model: rr,
        dashboard_due: nr,
        dashboard_due_soon: or,
        dashboard_empty_depot: ar,
        dashboard_empty_depot_message: sr,
        dashboard_empty_depot_title: ir,
        dashboard_empty_recent: cr,
        dashboard_quick_actions: lr,
        dashboard_recently_added: ur,
        dashboard_the_depot: _r,
        dashboard_view_all: dr,
        dashboard_yard_statistics: pr,
        depot_cars_title: Kr,
        depot_category: mr,
        depot_clear_search: Ur,
        depot_company: ln,
        depot_dcc_address: Gr,
        depot_description: fr,
        depot_empty_cars: Br,
        depot_empty_locomotives: Hr,
        depot_empty_trains: Jr,
        depot_group: Wr,
        depot_livery: qr,
        depot_locomotives_title: Xr,
        depot_manufacturer: gr,
        depot_no_results: Yr,
        depot_overflow_note: Qr,
        depot_product_code: hr,
        depot_road_number: en,
        depot_scale: vr,
        depot_search_placeholder: tn,
        depot_service_level: rn,
        depot_subtitle: nn,
        depot_title: on,
        depot_trains_title: an,
        depot_type: sn,
        depot_view_all: cn,
        errors_dashboard_message: yr,
        errors_dashboard_title: br,
        errors_retry_page: wr,
        errors_return_dashboard: Ir,
        example_message: kn,
        form_new_model_add_rolling_stock: fo,
        form_new_model_additional_details: Gn,
        form_new_model_availability_status: Un,
        form_new_model_basic_info: $n,
        form_new_model_cancel: ho,
        form_new_model_category: Zn,
        form_new_model_class_name: Yn,
        form_new_model_control: lo,
        form_new_model_create: go,
        form_new_model_dcc_interface: uo,
        form_new_model_delete: mo,
        form_new_model_delivery_availability: jn,
        form_new_model_delivery_date: Fn,
        form_new_model_delivery_date_placeholder: Kn,
        form_new_model_depot: to,
        form_new_model_description: Nn,
        form_new_model_description_placeholder: Dn,
        form_new_model_details_placeholder: Bn,
        form_new_model_duplicate: po,
        form_new_model_emu_type: so,
        form_new_model_epoch: Mn,
        form_new_model_freight_car_type: ao,
        form_new_model_is_dummy: io,
        form_new_model_livery: qn,
        form_new_model_livery_placeholder: Xn,
        form_new_model_manufacturer: On,
        form_new_model_passenger_car_type: oo,
        form_new_model_power_method: Ln,
        form_new_model_product_code: Pn,
        form_new_model_product_code_placeholder: Rn,
        form_new_model_railway_company: Jn,
        form_new_model_road_number: Qn,
        form_new_model_rolling_stock: Hn,
        form_new_model_rolling_stock_category: Wn,
        form_new_model_scale: xn,
        form_new_model_select_placeholder: Vn,
        form_new_model_series: eo,
        form_new_model_service_level: _o,
        form_new_model_technical_details: co,
        form_new_model_title: zn,
        form_new_model_type: ro,
        form_new_model_type_name: no,
        m: Fu,
        stats_maintenance_alerts: Er,
        stats_rolling_stocks: Sr,
        stats_total_collection_value: Cr,
        wishlist_modal_add_failed: kr,
        wishlist_modal_cancel: Ar,
        wishlist_modal_choose_or_create: Tr,
        wishlist_modal_close: zr,
        wishlist_modal_create_failed: $r,
        wishlist_modal_item_id_label: Or,
        wishlist_modal_item_id_placeholder: Pr,
        wishlist_modal_missing_model: Rr,
        wishlist_modal_new_list_placeholder: Nr,
        wishlist_modal_notes_label: Dr,
        wishlist_modal_notes_placeholder: Zr,
        wishlist_modal_save: xr,
        wishlist_modal_saving: Lr,
        wishlist_modal_select_list: Mr,
        wishlist_modal_select_list_error: Vr,
        wishlist_modal_select_placeholder: jr,
        wishlist_modal_title: Fr
      },
      Symbol.toStringTag,
      { value: 'Module' }
    )
  ),
  wt = Qa(),
  xe = Qa();
var Uu = b('<div><!></div>');
function Gu(e, t) {
  ze(t, !0);
  const r = We(t, ['$$slots', '$$events', '$$legacy']),
    n = xe.consume(),
    o = wt.consume(),
    a = y(() => t.element),
    s = y(() => t.children),
    i = y(() => Je(r, ['element', 'children'])),
    c = y(() => qe(n().getItemContentProps(o()), _(i)));
  var u = me(),
    g = Z(u);
  {
    var C = (m) => {
        var k = me(),
          j = Z(k);
        (ke(
          j,
          () => _(a),
          () => _(c)
        ),
          f(m, k));
      },
      l = (m) => {
        var k = Uu();
        He(k, () => ({ ..._(c) }));
        var j = $(k);
        (ke(j, () => _(s) ?? he), O(k), f(m, k));
      };
    Ie(g, (m) => {
      _(a) ? m(C) : m(l, !1);
    });
  }
  (f(e, u), $e());
}
var Bu = b('<div><!></div>');
function Hu(e, t) {
  ze(t, !0);
  const r = We(t, ['$$slots', '$$events', '$$legacy']),
    n = xe.consume(),
    o = wt.consume(),
    a = y(() => t.element),
    s = y(() => t.children),
    i = y(() => Je(r, ['element', 'children'])),
    c = y(() => qe(n().getItemIndicatorProps(o()), _(i)));
  var u = me(),
    g = Z(u);
  {
    var C = (m) => {
        var k = me(),
          j = Z(k);
        (ke(
          j,
          () => _(a),
          () => _(c)
        ),
          f(m, k));
      },
      l = (m) => {
        var k = Bu();
        He(k, () => ({ ..._(c) }));
        var j = $(k);
        (ke(j, () => _(s) ?? he), O(k), f(m, k));
      };
    Ie(g, (m) => {
      _(a) ? m(C) : m(l, !1);
    });
  }
  (f(e, u), $e());
}
var Ju = b('<button><!></button>');
function Wu(e, t) {
  ze(t, !0);
  const r = We(t, ['$$slots', '$$events', '$$legacy']),
    n = xe.consume(),
    o = wt.consume(),
    a = y(() => t.element),
    s = y(() => t.children),
    i = y(() => Je(r, ['element', 'children'])),
    c = y(() => qe(n().getItemTriggerProps(o()), _(i)));
  var u = me(),
    g = Z(u);
  {
    var C = (m) => {
        var k = me(),
          j = Z(k);
        (ke(
          j,
          () => _(a),
          () => _(c)
        ),
          f(m, k));
      },
      l = (m) => {
        var k = Ju();
        He(k, () => ({ ..._(c) }));
        var j = $(k);
        (ke(j, () => _(s) ?? he), O(k), f(m, k));
      };
    Ie(g, (m) => {
      _(a) ? m(C) : m(l, !1);
    });
  }
  (f(e, u), $e());
}
var qu = Ws('accordion').parts('root', 'item', 'itemTrigger', 'itemContent', 'itemIndicator'),
  Qe = qu.build(),
  gt = (e) => e.ids?.root ?? `accordion:${e.id}`,
  Xu = (e, t) => e.ids?.item?.(t) ?? `accordion:${e.id}:item:${t}`,
  Aa = (e, t) => e.ids?.itemContent?.(t) ?? `accordion:${e.id}:content:${t}`,
  ht = (e, t) => e.ids?.itemTrigger?.(t) ?? `accordion:${e.id}:trigger:${t}`,
  Yu = (e) => e.getById(gt(e)),
  It = (e) => {
    const r = `[aria-controls][data-ownedby='${CSS.escape(gt(e))}']:not([disabled])`;
    return oi(Yu(e), r);
  },
  Qu = (e) => ti(It(e)),
  e_ = (e) => ei(It(e)),
  t_ = (e, t) => Qs(It(e), ht(e, t)),
  r_ = (e, t) => Ys(It(e), ht(e, t));
function n_(e, t) {
  const { send: r, context: n, prop: o, scope: a, computed: s } = e,
    i = n.get('focusedValue'),
    c = n.get('value'),
    u = o('multiple');
  function g(l) {
    let m = l;
    (!u && m.length > 1 && (m = [m[0]]), r({ type: 'VALUE.SET', value: m }));
  }
  function C(l) {
    return {
      expanded: c.includes(l.value),
      focused: i === l.value,
      disabled: !!(l.disabled ?? o('disabled'))
    };
  }
  return {
    focusedValue: i,
    value: c,
    setValue: g,
    getItemState: C,
    getRootProps() {
      return t.element({
        ...Qe.root.attrs,
        dir: o('dir'),
        id: gt(a),
        'data-orientation': o('orientation')
      });
    },
    getItemProps(l) {
      const m = C(l);
      return t.element({
        ...Qe.item.attrs,
        dir: o('dir'),
        id: Xu(a, l.value),
        'data-state': m.expanded ? 'open' : 'closed',
        'data-focus': Fe(m.focused),
        'data-disabled': Fe(m.disabled),
        'data-orientation': o('orientation')
      });
    },
    getItemContentProps(l) {
      const m = C(l);
      return t.element({
        ...Qe.itemContent.attrs,
        dir: o('dir'),
        role: 'region',
        id: Aa(a, l.value),
        'aria-labelledby': ht(a, l.value),
        hidden: !m.expanded,
        'data-state': m.expanded ? 'open' : 'closed',
        'data-disabled': Fe(m.disabled),
        'data-focus': Fe(m.focused),
        'data-orientation': o('orientation')
      });
    },
    getItemIndicatorProps(l) {
      const m = C(l);
      return t.element({
        ...Qe.itemIndicator.attrs,
        dir: o('dir'),
        'aria-hidden': !0,
        'data-state': m.expanded ? 'open' : 'closed',
        'data-disabled': Fe(m.disabled),
        'data-focus': Fe(m.focused),
        'data-orientation': o('orientation')
      });
    },
    getItemTriggerProps(l) {
      const { value: m } = l,
        k = C(l);
      return t.button({
        ...Qe.itemTrigger.attrs,
        type: 'button',
        dir: o('dir'),
        id: ht(a, m),
        'aria-controls': Aa(a, m),
        'aria-expanded': k.expanded,
        disabled: k.disabled,
        'data-orientation': o('orientation'),
        'aria-disabled': k.disabled,
        'data-state': k.expanded ? 'open' : 'closed',
        'data-ownedby': gt(a),
        onFocus() {
          k.disabled || r({ type: 'TRIGGER.FOCUS', value: m });
        },
        onBlur() {
          k.disabled || r({ type: 'TRIGGER.BLUR' });
        },
        onClick(j) {
          k.disabled || (si() && j.currentTarget.focus(), r({ type: 'TRIGGER.CLICK', value: m }));
        },
        onKeyDown(j) {
          if (j.defaultPrevented || k.disabled) return;
          const pe = {
              ArrowDown() {
                s('isHorizontal') || r({ type: 'GOTO.NEXT', value: m });
              },
              ArrowUp() {
                s('isHorizontal') || r({ type: 'GOTO.PREV', value: m });
              },
              ArrowRight() {
                s('isHorizontal') && r({ type: 'GOTO.NEXT', value: m });
              },
              ArrowLeft() {
                s('isHorizontal') && r({ type: 'GOTO.PREV', value: m });
              },
              Home() {
                r({ type: 'GOTO.FIRST', value: m });
              },
              End() {
                r({ type: 'GOTO.LAST', value: m });
              }
            },
            se = ai(j, { dir: o('dir'), orientation: o('orientation') }),
            R = pe[se];
          R && (R(j), j.preventDefault());
        }
      });
    }
  };
}
var { and: o_, not: a_ } = qs(),
  s_ = Js({
    props({ props: e }) {
      return { collapsible: !1, multiple: !1, orientation: 'vertical', defaultValue: [], ...e };
    },
    initialState() {
      return 'idle';
    },
    context({ prop: e, bindable: t }) {
      return {
        focusedValue: t(() => ({
          defaultValue: null,
          sync: !0,
          onChange(r) {
            e('onFocusChange')?.({ value: r });
          }
        })),
        value: t(() => ({
          defaultValue: e('defaultValue'),
          value: e('value'),
          onChange(r) {
            e('onValueChange')?.({ value: r });
          }
        }))
      };
    },
    computed: { isHorizontal: ({ prop: e }) => e('orientation') === 'horizontal' },
    on: { 'VALUE.SET': { actions: ['setValue'] } },
    states: {
      idle: { on: { 'TRIGGER.FOCUS': { target: 'focused', actions: ['setFocusedValue'] } } },
      focused: {
        on: {
          'GOTO.NEXT': { actions: ['focusNextTrigger'] },
          'GOTO.PREV': { actions: ['focusPrevTrigger'] },
          'TRIGGER.CLICK': [
            { guard: o_('isExpanded', 'canToggle'), actions: ['collapse'] },
            { guard: a_('isExpanded'), actions: ['expand'] }
          ],
          'GOTO.FIRST': { actions: ['focusFirstTrigger'] },
          'GOTO.LAST': { actions: ['focusLastTrigger'] },
          'TRIGGER.BLUR': { target: 'idle', actions: ['clearFocusedValue'] }
        }
      }
    },
    implementations: {
      guards: {
        canToggle: ({ prop: e }) => !!e('collapsible') || !!e('multiple'),
        isExpanded: ({ context: e, event: t }) => e.get('value').includes(t.value)
      },
      actions: {
        collapse({ context: e, prop: t, event: r }) {
          const n = t('multiple') ? ni(e.get('value'), r.value) : [];
          e.set('value', n);
        },
        expand({ context: e, prop: t, event: r }) {
          const n = t('multiple') ? ri(e.get('value'), r.value) : [r.value];
          e.set('value', n);
        },
        focusFirstTrigger({ scope: e }) {
          Qu(e)?.focus();
        },
        focusLastTrigger({ scope: e }) {
          e_(e)?.focus();
        },
        focusNextTrigger({ context: e, scope: t }) {
          const r = e.get('focusedValue');
          if (!r) return;
          t_(t, r)?.focus();
        },
        focusPrevTrigger({ context: e, scope: t }) {
          const r = e.get('focusedValue');
          if (!r) return;
          r_(t, r)?.focus();
        },
        setFocusedValue({ context: e, event: t }) {
          e.set('focusedValue', t.value);
        },
        clearFocusedValue({ context: e }) {
          e.set('focusedValue', null);
        },
        setValue({ context: e, event: t }) {
          e.set('value', t.value);
        },
        coarseValue({ context: e, prop: t }) {
          !t('multiple') &&
            e.get('value').length > 1 &&
            (Xs('The value of accordion should be a single value when multiple is false.'),
            e.set('value', [e.get('value')[0]]));
        }
      }
    }
  }),
  i_ = es()([
    'collapsible',
    'dir',
    'disabled',
    'getRootNode',
    'id',
    'ids',
    'multiple',
    'onFocusChange',
    'onValueChange',
    'orientation',
    'value',
    'defaultValue'
  ]),
  c_ = ts(i_),
  l_ = es()(['value', 'disabled']),
  u_ = ts(l_),
  __ = b('<div><!></div>');
function d_(e, t) {
  ze(t, !0);
  const r = We(t, ['$$slots', '$$events', '$$legacy']),
    n = xe.consume(),
    o = y(() => u_(r)),
    a = y(() => Xa(_(o), 2)),
    s = y(() => _(a)[0]),
    i = y(() => _(a)[1]),
    c = y(() => _(i).element),
    u = y(() => _(i).children),
    g = y(() => Je(_(i), ['element', 'children'])),
    C = y(() => qe(n().getItemProps(_(s)), _(g)));
  wt.provide(() => _(s));
  var l = me(),
    m = Z(l);
  {
    var k = (pe) => {
        var se = me(),
          R = Z(se);
        (ke(
          R,
          () => _(c),
          () => _(C)
        ),
          f(pe, se));
      },
      j = (pe) => {
        var se = __();
        He(se, () => ({ ..._(C) }));
        var R = $(se);
        (ke(R, () => _(u) ?? he), O(se), f(pe, se));
      };
    Ie(m, (pe) => {
      _(c) ? pe(k) : pe(j, !1);
    });
  }
  (f(e, l), $e());
}
function p_(e, t) {
  ze(t, !0);
  const r = xe.consume(),
    n = y(() => t.children);
  var o = me(),
    a = Z(o);
  (ke(
    a,
    () => _(n),
    () => r
  ),
    f(e, o),
    $e());
}
var m_ = b('<div><!></div>');
function f_(e, t) {
  ze(t, !0);
  const r = We(t, ['$$slots', '$$events', '$$legacy']),
    n = y(() => t.element),
    o = y(() => t.children),
    a = y(() => t.value),
    s = y(() => Je(r, ['element', 'children', 'value'])),
    i = y(() => qe(_(a)().getRootProps(), _(s)));
  xe.provide(() => _(a)());
  var c = me(),
    u = Z(c);
  {
    var g = (l) => {
        var m = me(),
          k = Z(m);
        (ke(
          k,
          () => _(n),
          () => _(i)
        ),
          f(l, m));
      },
      C = (l) => {
        var m = m_();
        He(m, () => ({ ..._(i) }));
        var k = $(m);
        (ke(k, () => _(o) ?? he), O(m), f(l, m));
      };
    Ie(u, (l) => {
      _(n) ? l(g) : l(C, !1);
    });
  }
  (f(e, c), $e());
}
function g_(e) {
  const t = Bs(s_, e),
    r = y(() => n_(t, Hs));
  return () => _(r);
}
var h_ = b('<div><!></div>');
function v_(e, t) {
  const r = Ns();
  ze(t, !0);
  const n = We(t, ['$$slots', '$$events', '$$legacy']),
    o = y(() => c_(n)),
    a = y(() => Xa(_(o), 2)),
    s = y(() => _(a)[0]),
    i = y(() => _(a)[1]),
    c = y(() => _(i).element),
    u = y(() => _(i).children),
    g = y(() => Je(_(i), ['element', 'children'])),
    C = g_(() => ({ ..._(s), id: r })),
    l = y(() => qe(C().getRootProps(), _(g)));
  xe.provide(() => C());
  var m = me(),
    k = Z(m);
  {
    var j = (se) => {
        var R = me(),
          le = Z(R);
        (ke(
          le,
          () => _(c),
          () => _(l)
        ),
          f(se, R));
      },
      pe = (se) => {
        var R = h_();
        He(R, () => ({ ..._(l) }));
        var le = $(R);
        (ke(le, () => _(u) ?? he), O(R), f(se, R));
      };
    Ie(k, (se) => {
      _(c) ? se(j) : se(pe, !1);
    });
  }
  (f(e, m), $e());
}
const Ce = Object.assign(v_, {
  Provider: f_,
  Context: p_,
  Item: d_,
  ItemTrigger: Wu,
  ItemIndicator: Hu,
  ItemContent: Gu
});
function d(e, t, r) {
  function n(i, c) {
    if (
      (i._zod ||
        Object.defineProperty(i, '_zod', {
          value: { def: c, constr: s, traits: new Set() },
          enumerable: !1
        }),
      i._zod.traits.has(e))
    )
      return;
    (i._zod.traits.add(e), t(i, c));
    const u = s.prototype,
      g = Object.keys(u);
    for (let C = 0; C < g.length; C++) {
      const l = g[C];
      l in i || (i[l] = u[l].bind(i));
    }
  }
  const o = r?.Parent ?? Object;
  class a extends o {}
  Object.defineProperty(a, 'name', { value: e });
  function s(i) {
    var c;
    const u = r?.Parent ? new a() : this;
    (n(u, i), (c = u._zod).deferred ?? (c.deferred = []));
    for (const g of u._zod.deferred) g();
    return u;
  }
  return (
    Object.defineProperty(s, 'init', { value: n }),
    Object.defineProperty(s, Symbol.hasInstance, {
      value: (i) => (r?.Parent && i instanceof r.Parent ? !0 : i?._zod?.traits?.has(e))
    }),
    Object.defineProperty(s, 'name', { value: e }),
    s
  );
}
class Ge extends Error {
  constructor() {
    super('Encountered Promise during synchronous parse. Use .parseAsync() instead.');
  }
}
class rs extends Error {
  constructor(t) {
    (super(`Encountered unidirectional transform during encode: ${t}`),
      (this.name = 'ZodEncodeError'));
  }
}
const ns = {};
function De(e) {
  return ns;
}
function os(e) {
  const t = Object.values(e).filter((n) => typeof n == 'number');
  return Object.entries(e)
    .filter(([n, o]) => t.indexOf(+n) === -1)
    .map(([n, o]) => o);
}
function Dt(e, t) {
  return typeof t == 'bigint' ? t.toString() : t;
}
function Et(e) {
  return {
    get value() {
      {
        const t = e();
        return (Object.defineProperty(this, 'value', { value: t }), t);
      }
    }
  };
}
function ha(e) {
  return e == null;
}
function va(e) {
  const t = e.startsWith('^') ? 1 : 0,
    r = e.endsWith('$') ? e.length - 1 : e.length;
  return e.slice(t, r);
}
function y_(e, t) {
  const r = (e.toString().split('.')[1] || '').length,
    n = t.toString();
  let o = (n.split('.')[1] || '').length;
  if (o === 0 && /\d?e-\d?/.test(n)) {
    const c = n.match(/\d?e-(\d?)/);
    c?.[1] && (o = Number.parseInt(c[1]));
  }
  const a = r > o ? r : o,
    s = Number.parseInt(e.toFixed(a).replace('.', '')),
    i = Number.parseInt(t.toFixed(a).replace('.', ''));
  return (s % i) / 10 ** a;
}
const Ta = Symbol('evaluating');
function J(e, t, r) {
  let n;
  Object.defineProperty(e, t, {
    get() {
      if (n !== Ta) return (n === void 0 && ((n = Ta), (n = r())), n);
    },
    set(o) {
      Object.defineProperty(e, t, { value: o });
    },
    configurable: !0
  });
}
function Le(e, t, r) {
  Object.defineProperty(e, t, { value: r, writable: !0, enumerable: !0, configurable: !0 });
}
function Me(...e) {
  const t = {};
  for (const r of e) {
    const n = Object.getOwnPropertyDescriptors(r);
    Object.assign(t, n);
  }
  return Object.defineProperties({}, t);
}
function za(e) {
  return JSON.stringify(e);
}
function b_(e) {
  return e
    .toLowerCase()
    .trim()
    .replace(/[^\w\s-]/g, '')
    .replace(/[\s_-]+/g, '-')
    .replace(/^-+|-+$/g, '');
}
const as = 'captureStackTrace' in Error ? Error.captureStackTrace : (...e) => {};
function nt(e) {
  return typeof e == 'object' && e !== null && !Array.isArray(e);
}
const w_ = Et(() => {
  if (typeof navigator < 'u' && navigator?.userAgent?.includes('Cloudflare')) return !1;
  try {
    const e = Function;
    return (new e(''), !0);
  } catch {
    return !1;
  }
});
function ot(e) {
  if (nt(e) === !1) return !1;
  const t = e.constructor;
  if (t === void 0 || typeof t != 'function') return !0;
  const r = t.prototype;
  return !(nt(r) === !1 || Object.prototype.hasOwnProperty.call(r, 'isPrototypeOf') === !1);
}
function ss(e) {
  return ot(e) ? { ...e } : Array.isArray(e) ? [...e] : e;
}
const I_ = new Set(['string', 'number', 'symbol']);
function Be(e) {
  return e.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}
function Oe(e, t, r) {
  const n = new e._zod.constr(t ?? e._zod.def);
  return ((!t || r?.parent) && (n._zod.parent = e), n);
}
function T(e) {
  const t = e;
  if (!t) return {};
  if (typeof t == 'string') return { error: () => t };
  if (t?.message !== void 0) {
    if (t?.error !== void 0) throw new Error('Cannot specify both `message` and `error` params');
    t.error = t.message;
  }
  return (delete t.message, typeof t.error == 'string' ? { ...t, error: () => t.error } : t);
}
function E_(e) {
  return Object.keys(e).filter(
    (t) => e[t]._zod.optin === 'optional' && e[t]._zod.optout === 'optional'
  );
}
const S_ = {
  safeint: [Number.MIN_SAFE_INTEGER, Number.MAX_SAFE_INTEGER],
  int32: [-2147483648, 2147483647],
  uint32: [0, 4294967295],
  float32: [-34028234663852886e22, 34028234663852886e22],
  float64: [-Number.MAX_VALUE, Number.MAX_VALUE]
};
function C_(e, t) {
  const r = e._zod.def,
    n = Me(e._zod.def, {
      get shape() {
        const o = {};
        for (const a in t) {
          if (!(a in r.shape)) throw new Error(`Unrecognized key: "${a}"`);
          t[a] && (o[a] = r.shape[a]);
        }
        return (Le(this, 'shape', o), o);
      },
      checks: []
    });
  return Oe(e, n);
}
function k_(e, t) {
  const r = e._zod.def,
    n = Me(e._zod.def, {
      get shape() {
        const o = { ...e._zod.def.shape };
        for (const a in t) {
          if (!(a in r.shape)) throw new Error(`Unrecognized key: "${a}"`);
          t[a] && delete o[a];
        }
        return (Le(this, 'shape', o), o);
      },
      checks: []
    });
  return Oe(e, n);
}
function A_(e, t) {
  if (!ot(t)) throw new Error('Invalid input to extend: expected a plain object');
  const r = e._zod.def.checks;
  if (r && r.length > 0)
    throw new Error(
      'Object schemas containing refinements cannot be extended. Use `.safeExtend()` instead.'
    );
  const o = Me(e._zod.def, {
    get shape() {
      const a = { ...e._zod.def.shape, ...t };
      return (Le(this, 'shape', a), a);
    },
    checks: []
  });
  return Oe(e, o);
}
function T_(e, t) {
  if (!ot(t)) throw new Error('Invalid input to safeExtend: expected a plain object');
  const r = {
    ...e._zod.def,
    get shape() {
      const n = { ...e._zod.def.shape, ...t };
      return (Le(this, 'shape', n), n);
    },
    checks: e._zod.def.checks
  };
  return Oe(e, r);
}
function z_(e, t) {
  const r = Me(e._zod.def, {
    get shape() {
      const n = { ...e._zod.def.shape, ...t._zod.def.shape };
      return (Le(this, 'shape', n), n);
    },
    get catchall() {
      return t._zod.def.catchall;
    },
    checks: []
  });
  return Oe(e, r);
}
function $_(e, t, r) {
  const n = Me(t._zod.def, {
    get shape() {
      const o = t._zod.def.shape,
        a = { ...o };
      if (r)
        for (const s in r) {
          if (!(s in o)) throw new Error(`Unrecognized key: "${s}"`);
          r[s] && (a[s] = e ? new e({ type: 'optional', innerType: o[s] }) : o[s]);
        }
      else for (const s in o) a[s] = e ? new e({ type: 'optional', innerType: o[s] }) : o[s];
      return (Le(this, 'shape', a), a);
    },
    checks: []
  });
  return Oe(t, n);
}
function O_(e, t, r) {
  const n = Me(t._zod.def, {
    get shape() {
      const o = t._zod.def.shape,
        a = { ...o };
      if (r)
        for (const s in r) {
          if (!(s in a)) throw new Error(`Unrecognized key: "${s}"`);
          r[s] && (a[s] = new e({ type: 'nonoptional', innerType: o[s] }));
        }
      else for (const s in o) a[s] = new e({ type: 'nonoptional', innerType: o[s] });
      return (Le(this, 'shape', a), a);
    },
    checks: []
  });
  return Oe(t, n);
}
function Ke(e, t = 0) {
  if (e.aborted === !0) return !0;
  for (let r = t; r < e.issues.length; r++) if (e.issues[r]?.continue !== !0) return !0;
  return !1;
}
function is(e, t) {
  return t.map((r) => {
    var n;
    return ((n = r).path ?? (n.path = []), r.path.unshift(e), r);
  });
}
function pt(e) {
  return typeof e == 'string' ? e : e?.message;
}
function Ze(e, t, r) {
  const n = { ...e, path: e.path ?? [] };
  if (!e.message) {
    const o =
      pt(e.inst?._zod.def?.error?.(e)) ??
      pt(t?.error?.(e)) ??
      pt(r.customError?.(e)) ??
      pt(r.localeError?.(e)) ??
      'Invalid input';
    n.message = o;
  }
  return (delete n.inst, delete n.continue, t?.reportInput || delete n.input, n);
}
function ya(e) {
  return Array.isArray(e) ? 'array' : typeof e == 'string' ? 'string' : 'unknown';
}
function at(...e) {
  const [t, r, n] = e;
  return typeof t == 'string' ? { message: t, code: 'custom', input: r, inst: n } : { ...t };
}
const cs = (e, t) => {
    ((e.name = '$ZodError'),
      Object.defineProperty(e, '_zod', { value: e._zod, enumerable: !1 }),
      Object.defineProperty(e, 'issues', { value: t, enumerable: !1 }),
      (e.message = JSON.stringify(t, Dt, 2)),
      Object.defineProperty(e, 'toString', { value: () => e.message, enumerable: !1 }));
  },
  ls = d('$ZodError', cs),
  us = d('$ZodError', cs, { Parent: Error });
function P_(e, t = (r) => r.message) {
  const r = {},
    n = [];
  for (const o of e.issues)
    o.path.length > 0
      ? ((r[o.path[0]] = r[o.path[0]] || []), r[o.path[0]].push(t(o)))
      : n.push(t(o));
  return { formErrors: n, fieldErrors: r };
}
function R_(e, t = (r) => r.message) {
  const r = { _errors: [] },
    n = (o) => {
      for (const a of o.issues)
        if (a.code === 'invalid_union' && a.errors.length) a.errors.map((s) => n({ issues: s }));
        else if (a.code === 'invalid_key') n({ issues: a.issues });
        else if (a.code === 'invalid_element') n({ issues: a.issues });
        else if (a.path.length === 0) r._errors.push(t(a));
        else {
          let s = r,
            i = 0;
          for (; i < a.path.length; ) {
            const c = a.path[i];
            (i === a.path.length - 1
              ? ((s[c] = s[c] || { _errors: [] }), s[c]._errors.push(t(a)))
              : (s[c] = s[c] || { _errors: [] }),
              (s = s[c]),
              i++);
          }
        }
    };
  return (n(e), r);
}
const ba = (e) => (t, r, n, o) => {
    const a = n ? Object.assign(n, { async: !1 }) : { async: !1 },
      s = t._zod.run({ value: r, issues: [] }, a);
    if (s instanceof Promise) throw new Ge();
    if (s.issues.length) {
      const i = new (o?.Err ?? e)(s.issues.map((c) => Ze(c, a, De())));
      throw (as(i, o?.callee), i);
    }
    return s.value;
  },
  wa = (e) => async (t, r, n, o) => {
    const a = n ? Object.assign(n, { async: !0 }) : { async: !0 };
    let s = t._zod.run({ value: r, issues: [] }, a);
    if ((s instanceof Promise && (s = await s), s.issues.length)) {
      const i = new (o?.Err ?? e)(s.issues.map((c) => Ze(c, a, De())));
      throw (as(i, o?.callee), i);
    }
    return s.value;
  },
  St = (e) => (t, r, n) => {
    const o = n ? { ...n, async: !1 } : { async: !1 },
      a = t._zod.run({ value: r, issues: [] }, o);
    if (a instanceof Promise) throw new Ge();
    return a.issues.length
      ? { success: !1, error: new (e ?? ls)(a.issues.map((s) => Ze(s, o, De()))) }
      : { success: !0, data: a.value };
  },
  N_ = St(us),
  Ct = (e) => async (t, r, n) => {
    const o = n ? Object.assign(n, { async: !0 }) : { async: !0 };
    let a = t._zod.run({ value: r, issues: [] }, o);
    return (
      a instanceof Promise && (a = await a),
      a.issues.length
        ? { success: !1, error: new e(a.issues.map((s) => Ze(s, o, De()))) }
        : { success: !0, data: a.value }
    );
  },
  D_ = Ct(us),
  Z_ = (e) => (t, r, n) => {
    const o = n ? Object.assign(n, { direction: 'backward' }) : { direction: 'backward' };
    return ba(e)(t, r, o);
  },
  x_ = (e) => (t, r, n) => ba(e)(t, r, n),
  L_ = (e) => async (t, r, n) => {
    const o = n ? Object.assign(n, { direction: 'backward' }) : { direction: 'backward' };
    return wa(e)(t, r, o);
  },
  M_ = (e) => async (t, r, n) => wa(e)(t, r, n),
  V_ = (e) => (t, r, n) => {
    const o = n ? Object.assign(n, { direction: 'backward' }) : { direction: 'backward' };
    return St(e)(t, r, o);
  },
  j_ = (e) => (t, r, n) => St(e)(t, r, n),
  F_ = (e) => async (t, r, n) => {
    const o = n ? Object.assign(n, { direction: 'backward' }) : { direction: 'backward' };
    return Ct(e)(t, r, o);
  },
  K_ = (e) => async (t, r, n) => Ct(e)(t, r, n),
  U_ = /^[cC][^\s-]{8,}$/,
  G_ = /^[0-9a-z]+$/,
  B_ = /^[0-9A-HJKMNP-TV-Za-hjkmnp-tv-z]{26}$/,
  H_ = /^[0-9a-vA-V]{20}$/,
  J_ = /^[A-Za-z0-9]{27}$/,
  W_ = /^[a-zA-Z0-9_-]{21}$/,
  q_ =
    /^P(?:(\d+W)|(?!.*W)(?=\d|T\d)(\d+Y)?(\d+M)?(\d+D)?(T(?=\d)(\d+H)?(\d+M)?(\d+([.,]\d+)?S)?)?)$/,
  X_ = /^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})$/,
  $a = (e) =>
    e
      ? new RegExp(
          `^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-${e}[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`
        )
      : /^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-8][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}|00000000-0000-0000-0000-000000000000|ffffffff-ffff-ffff-ffff-ffffffffffff)$/,
  Y_ =
    /^(?!\.)(?!.*\.\.)([A-Za-z0-9_'+\-\.]*)[A-Za-z0-9_+-]@([A-Za-z0-9][A-Za-z0-9\-]*\.)+[A-Za-z]{2,}$/,
  Q_ = '^(\\p{Extended_Pictographic}|\\p{Emoji_Component})+$';
function ed() {
  return new RegExp(Q_, 'u');
}
const td =
    /^(?:(?:25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9])\.){3}(?:25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9])$/,
  rd =
    /^(([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:))$/,
  nd =
    /^((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9])\.){3}(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9])\/([0-9]|[1-2][0-9]|3[0-2])$/,
  od =
    /^(([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}|::|([0-9a-fA-F]{1,4})?::([0-9a-fA-F]{1,4}:?){0,6})\/(12[0-8]|1[01][0-9]|[1-9]?[0-9])$/,
  ad = /^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$/,
  _s = /^[A-Za-z0-9_-]*$/,
  sd = /^\+(?:[0-9]){6,14}[0-9]$/,
  ds =
    '(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))',
  id = new RegExp(`^${ds}$`);
function ps(e) {
  const t = '(?:[01]\\d|2[0-3]):[0-5]\\d';
  return typeof e.precision == 'number'
    ? e.precision === -1
      ? `${t}`
      : e.precision === 0
        ? `${t}:[0-5]\\d`
        : `${t}:[0-5]\\d\\.\\d{${e.precision}}`
    : `${t}(?::[0-5]\\d(?:\\.\\d+)?)?`;
}
function cd(e) {
  return new RegExp(`^${ps(e)}$`);
}
function ld(e) {
  const t = ps({ precision: e.precision }),
    r = ['Z'];
  (e.local && r.push(''), e.offset && r.push('([+-](?:[01]\\d|2[0-3]):[0-5]\\d)'));
  const n = `${t}(?:${r.join('|')})`;
  return new RegExp(`^${ds}T(?:${n})$`);
}
const ud = (e) => {
    const t = e ? `[\\s\\S]{${e?.minimum ?? 0},${e?.maximum ?? ''}}` : '[\\s\\S]*';
    return new RegExp(`^${t}$`);
  },
  _d = /^-?\d+$/,
  dd = /^-?\d+(?:\.\d+)?/,
  pd = /^(?:true|false)$/i,
  md = /^[^A-Z]*$/,
  fd = /^[^a-z]*$/,
  Ee = d('$ZodCheck', (e, t) => {
    var r;
    (e._zod ?? (e._zod = {}), (e._zod.def = t), (r = e._zod).onattach ?? (r.onattach = []));
  }),
  ms = { number: 'number', bigint: 'bigint', object: 'date' },
  fs = d('$ZodCheckLessThan', (e, t) => {
    Ee.init(e, t);
    const r = ms[typeof t.value];
    (e._zod.onattach.push((n) => {
      const o = n._zod.bag,
        a = (t.inclusive ? o.maximum : o.exclusiveMaximum) ?? Number.POSITIVE_INFINITY;
      t.value < a && (t.inclusive ? (o.maximum = t.value) : (o.exclusiveMaximum = t.value));
    }),
      (e._zod.check = (n) => {
        (t.inclusive ? n.value <= t.value : n.value < t.value) ||
          n.issues.push({
            origin: r,
            code: 'too_big',
            maximum: t.value,
            input: n.value,
            inclusive: t.inclusive,
            inst: e,
            continue: !t.abort
          });
      }));
  }),
  gs = d('$ZodCheckGreaterThan', (e, t) => {
    Ee.init(e, t);
    const r = ms[typeof t.value];
    (e._zod.onattach.push((n) => {
      const o = n._zod.bag,
        a = (t.inclusive ? o.minimum : o.exclusiveMinimum) ?? Number.NEGATIVE_INFINITY;
      t.value > a && (t.inclusive ? (o.minimum = t.value) : (o.exclusiveMinimum = t.value));
    }),
      (e._zod.check = (n) => {
        (t.inclusive ? n.value >= t.value : n.value > t.value) ||
          n.issues.push({
            origin: r,
            code: 'too_small',
            minimum: t.value,
            input: n.value,
            inclusive: t.inclusive,
            inst: e,
            continue: !t.abort
          });
      }));
  }),
  gd = d('$ZodCheckMultipleOf', (e, t) => {
    (Ee.init(e, t),
      e._zod.onattach.push((r) => {
        var n;
        (n = r._zod.bag).multipleOf ?? (n.multipleOf = t.value);
      }),
      (e._zod.check = (r) => {
        if (typeof r.value != typeof t.value)
          throw new Error('Cannot mix number and bigint in multiple_of check.');
        (typeof r.value == 'bigint'
          ? r.value % t.value === BigInt(0)
          : y_(r.value, t.value) === 0) ||
          r.issues.push({
            origin: typeof r.value,
            code: 'not_multiple_of',
            divisor: t.value,
            input: r.value,
            inst: e,
            continue: !t.abort
          });
      }));
  }),
  hd = d('$ZodCheckNumberFormat', (e, t) => {
    (Ee.init(e, t), (t.format = t.format || 'float64'));
    const r = t.format?.includes('int'),
      n = r ? 'int' : 'number',
      [o, a] = S_[t.format];
    (e._zod.onattach.push((s) => {
      const i = s._zod.bag;
      ((i.format = t.format), (i.minimum = o), (i.maximum = a), r && (i.pattern = _d));
    }),
      (e._zod.check = (s) => {
        const i = s.value;
        if (r) {
          if (!Number.isInteger(i)) {
            s.issues.push({
              expected: n,
              format: t.format,
              code: 'invalid_type',
              continue: !1,
              input: i,
              inst: e
            });
            return;
          }
          if (!Number.isSafeInteger(i)) {
            i > 0
              ? s.issues.push({
                  input: i,
                  code: 'too_big',
                  maximum: Number.MAX_SAFE_INTEGER,
                  note: 'Integers must be within the safe integer range.',
                  inst: e,
                  origin: n,
                  continue: !t.abort
                })
              : s.issues.push({
                  input: i,
                  code: 'too_small',
                  minimum: Number.MIN_SAFE_INTEGER,
                  note: 'Integers must be within the safe integer range.',
                  inst: e,
                  origin: n,
                  continue: !t.abort
                });
            return;
          }
        }
        (i < o &&
          s.issues.push({
            origin: 'number',
            input: i,
            code: 'too_small',
            minimum: o,
            inclusive: !0,
            inst: e,
            continue: !t.abort
          }),
          i > a &&
            s.issues.push({ origin: 'number', input: i, code: 'too_big', maximum: a, inst: e }));
      }));
  }),
  vd = d('$ZodCheckMaxLength', (e, t) => {
    var r;
    (Ee.init(e, t),
      (r = e._zod.def).when ??
        (r.when = (n) => {
          const o = n.value;
          return !ha(o) && o.length !== void 0;
        }),
      e._zod.onattach.push((n) => {
        const o = n._zod.bag.maximum ?? Number.POSITIVE_INFINITY;
        t.maximum < o && (n._zod.bag.maximum = t.maximum);
      }),
      (e._zod.check = (n) => {
        const o = n.value;
        if (o.length <= t.maximum) return;
        const s = ya(o);
        n.issues.push({
          origin: s,
          code: 'too_big',
          maximum: t.maximum,
          inclusive: !0,
          input: o,
          inst: e,
          continue: !t.abort
        });
      }));
  }),
  yd = d('$ZodCheckMinLength', (e, t) => {
    var r;
    (Ee.init(e, t),
      (r = e._zod.def).when ??
        (r.when = (n) => {
          const o = n.value;
          return !ha(o) && o.length !== void 0;
        }),
      e._zod.onattach.push((n) => {
        const o = n._zod.bag.minimum ?? Number.NEGATIVE_INFINITY;
        t.minimum > o && (n._zod.bag.minimum = t.minimum);
      }),
      (e._zod.check = (n) => {
        const o = n.value;
        if (o.length >= t.minimum) return;
        const s = ya(o);
        n.issues.push({
          origin: s,
          code: 'too_small',
          minimum: t.minimum,
          inclusive: !0,
          input: o,
          inst: e,
          continue: !t.abort
        });
      }));
  }),
  bd = d('$ZodCheckLengthEquals', (e, t) => {
    var r;
    (Ee.init(e, t),
      (r = e._zod.def).when ??
        (r.when = (n) => {
          const o = n.value;
          return !ha(o) && o.length !== void 0;
        }),
      e._zod.onattach.push((n) => {
        const o = n._zod.bag;
        ((o.minimum = t.length), (o.maximum = t.length), (o.length = t.length));
      }),
      (e._zod.check = (n) => {
        const o = n.value,
          a = o.length;
        if (a === t.length) return;
        const s = ya(o),
          i = a > t.length;
        n.issues.push({
          origin: s,
          ...(i
            ? { code: 'too_big', maximum: t.length }
            : { code: 'too_small', minimum: t.length }),
          inclusive: !0,
          exact: !0,
          input: n.value,
          inst: e,
          continue: !t.abort
        });
      }));
  }),
  kt = d('$ZodCheckStringFormat', (e, t) => {
    var r, n;
    (Ee.init(e, t),
      e._zod.onattach.push((o) => {
        const a = o._zod.bag;
        ((a.format = t.format),
          t.pattern && (a.patterns ?? (a.patterns = new Set()), a.patterns.add(t.pattern)));
      }),
      t.pattern
        ? ((r = e._zod).check ??
          (r.check = (o) => {
            ((t.pattern.lastIndex = 0),
              !t.pattern.test(o.value) &&
                o.issues.push({
                  origin: 'string',
                  code: 'invalid_format',
                  format: t.format,
                  input: o.value,
                  ...(t.pattern ? { pattern: t.pattern.toString() } : {}),
                  inst: e,
                  continue: !t.abort
                }));
          }))
        : ((n = e._zod).check ?? (n.check = () => {})));
  }),
  wd = d('$ZodCheckRegex', (e, t) => {
    (kt.init(e, t),
      (e._zod.check = (r) => {
        ((t.pattern.lastIndex = 0),
          !t.pattern.test(r.value) &&
            r.issues.push({
              origin: 'string',
              code: 'invalid_format',
              format: 'regex',
              input: r.value,
              pattern: t.pattern.toString(),
              inst: e,
              continue: !t.abort
            }));
      }));
  }),
  Id = d('$ZodCheckLowerCase', (e, t) => {
    (t.pattern ?? (t.pattern = md), kt.init(e, t));
  }),
  Ed = d('$ZodCheckUpperCase', (e, t) => {
    (t.pattern ?? (t.pattern = fd), kt.init(e, t));
  }),
  Sd = d('$ZodCheckIncludes', (e, t) => {
    Ee.init(e, t);
    const r = Be(t.includes),
      n = new RegExp(typeof t.position == 'number' ? `^.{${t.position}}${r}` : r);
    ((t.pattern = n),
      e._zod.onattach.push((o) => {
        const a = o._zod.bag;
        (a.patterns ?? (a.patterns = new Set()), a.patterns.add(n));
      }),
      (e._zod.check = (o) => {
        o.value.includes(t.includes, t.position) ||
          o.issues.push({
            origin: 'string',
            code: 'invalid_format',
            format: 'includes',
            includes: t.includes,
            input: o.value,
            inst: e,
            continue: !t.abort
          });
      }));
  }),
  Cd = d('$ZodCheckStartsWith', (e, t) => {
    Ee.init(e, t);
    const r = new RegExp(`^${Be(t.prefix)}.*`);
    (t.pattern ?? (t.pattern = r),
      e._zod.onattach.push((n) => {
        const o = n._zod.bag;
        (o.patterns ?? (o.patterns = new Set()), o.patterns.add(r));
      }),
      (e._zod.check = (n) => {
        n.value.startsWith(t.prefix) ||
          n.issues.push({
            origin: 'string',
            code: 'invalid_format',
            format: 'starts_with',
            prefix: t.prefix,
            input: n.value,
            inst: e,
            continue: !t.abort
          });
      }));
  }),
  kd = d('$ZodCheckEndsWith', (e, t) => {
    Ee.init(e, t);
    const r = new RegExp(`.*${Be(t.suffix)}$`);
    (t.pattern ?? (t.pattern = r),
      e._zod.onattach.push((n) => {
        const o = n._zod.bag;
        (o.patterns ?? (o.patterns = new Set()), o.patterns.add(r));
      }),
      (e._zod.check = (n) => {
        n.value.endsWith(t.suffix) ||
          n.issues.push({
            origin: 'string',
            code: 'invalid_format',
            format: 'ends_with',
            suffix: t.suffix,
            input: n.value,
            inst: e,
            continue: !t.abort
          });
      }));
  }),
  Ad = d('$ZodCheckOverwrite', (e, t) => {
    (Ee.init(e, t),
      (e._zod.check = (r) => {
        r.value = t.tx(r.value);
      }));
  });
class Td {
  constructor(t = []) {
    ((this.content = []), (this.indent = 0), this && (this.args = t));
  }
  indented(t) {
    ((this.indent += 1), t(this), (this.indent -= 1));
  }
  write(t) {
    if (typeof t == 'function') {
      (t(this, { execution: 'sync' }), t(this, { execution: 'async' }));
      return;
    }
    const n = t
        .split(
          `
`
        )
        .filter((s) => s),
      o = Math.min(...n.map((s) => s.length - s.trimStart().length)),
      a = n.map((s) => s.slice(o)).map((s) => ' '.repeat(this.indent * 2) + s);
    for (const s of a) this.content.push(s);
  }
  compile() {
    const t = Function,
      r = this?.args,
      o = [...(this?.content ?? ['']).map((a) => `  ${a}`)];
    return new t(
      ...r,
      o.join(`
`)
    );
  }
}
const zd = { major: 4, minor: 2, patch: 1 },
  ce = d('$ZodType', (e, t) => {
    var r;
    (e ?? (e = {}), (e._zod.def = t), (e._zod.bag = e._zod.bag || {}), (e._zod.version = zd));
    const n = [...(e._zod.def.checks ?? [])];
    e._zod.traits.has('$ZodCheck') && n.unshift(e);
    for (const o of n) for (const a of o._zod.onattach) a(e);
    if (n.length === 0)
      ((r = e._zod).deferred ?? (r.deferred = []),
        e._zod.deferred?.push(() => {
          e._zod.run = e._zod.parse;
        }));
    else {
      const o = (s, i, c) => {
          let u = Ke(s),
            g;
          for (const C of i) {
            if (C._zod.def.when) {
              if (!C._zod.def.when(s)) continue;
            } else if (u) continue;
            const l = s.issues.length,
              m = C._zod.check(s);
            if (m instanceof Promise && c?.async === !1) throw new Ge();
            if (g || m instanceof Promise)
              g = (g ?? Promise.resolve()).then(async () => {
                (await m, s.issues.length !== l && (u || (u = Ke(s, l))));
              });
            else {
              if (s.issues.length === l) continue;
              u || (u = Ke(s, l));
            }
          }
          return g ? g.then(() => s) : s;
        },
        a = (s, i, c) => {
          if (Ke(s)) return ((s.aborted = !0), s);
          const u = o(i, n, c);
          if (u instanceof Promise) {
            if (c.async === !1) throw new Ge();
            return u.then((g) => e._zod.parse(g, c));
          }
          return e._zod.parse(u, c);
        };
      e._zod.run = (s, i) => {
        if (i.skipChecks) return e._zod.parse(s, i);
        if (i.direction === 'backward') {
          const u = e._zod.parse({ value: s.value, issues: [] }, { ...i, skipChecks: !0 });
          return u instanceof Promise ? u.then((g) => a(g, s, i)) : a(u, s, i);
        }
        const c = e._zod.parse(s, i);
        if (c instanceof Promise) {
          if (i.async === !1) throw new Ge();
          return c.then((u) => o(u, n, i));
        }
        return o(c, n, i);
      };
    }
    e['~standard'] = {
      validate: (o) => {
        try {
          const a = N_(e, o);
          return a.success ? { value: a.data } : { issues: a.error?.issues };
        } catch {
          return D_(e, o).then((s) =>
            s.success ? { value: s.data } : { issues: s.error?.issues }
          );
        }
      },
      vendor: 'zod',
      version: 1
    };
  }),
  Ia = d('$ZodString', (e, t) => {
    (ce.init(e, t),
      (e._zod.pattern = [...(e?._zod.bag?.patterns ?? [])].pop() ?? ud(e._zod.bag)),
      (e._zod.parse = (r, n) => {
        if (t.coerce)
          try {
            r.value = String(r.value);
          } catch {}
        return (
          typeof r.value == 'string' ||
            r.issues.push({ expected: 'string', code: 'invalid_type', input: r.value, inst: e }),
          r
        );
      }));
  }),
  re = d('$ZodStringFormat', (e, t) => {
    (kt.init(e, t), Ia.init(e, t));
  }),
  $d = d('$ZodGUID', (e, t) => {
    (t.pattern ?? (t.pattern = X_), re.init(e, t));
  }),
  Od = d('$ZodUUID', (e, t) => {
    if (t.version) {
      const n = { v1: 1, v2: 2, v3: 3, v4: 4, v5: 5, v6: 6, v7: 7, v8: 8 }[t.version];
      if (n === void 0) throw new Error(`Invalid UUID version: "${t.version}"`);
      t.pattern ?? (t.pattern = $a(n));
    } else t.pattern ?? (t.pattern = $a());
    re.init(e, t);
  }),
  Pd = d('$ZodEmail', (e, t) => {
    (t.pattern ?? (t.pattern = Y_), re.init(e, t));
  }),
  Rd = d('$ZodURL', (e, t) => {
    (re.init(e, t),
      (e._zod.check = (r) => {
        try {
          const n = r.value.trim(),
            o = new URL(n);
          (t.hostname &&
            ((t.hostname.lastIndex = 0),
            t.hostname.test(o.hostname) ||
              r.issues.push({
                code: 'invalid_format',
                format: 'url',
                note: 'Invalid hostname',
                pattern: t.hostname.source,
                input: r.value,
                inst: e,
                continue: !t.abort
              })),
            t.protocol &&
              ((t.protocol.lastIndex = 0),
              t.protocol.test(o.protocol.endsWith(':') ? o.protocol.slice(0, -1) : o.protocol) ||
                r.issues.push({
                  code: 'invalid_format',
                  format: 'url',
                  note: 'Invalid protocol',
                  pattern: t.protocol.source,
                  input: r.value,
                  inst: e,
                  continue: !t.abort
                })),
            t.normalize ? (r.value = o.href) : (r.value = n));
          return;
        } catch {
          r.issues.push({
            code: 'invalid_format',
            format: 'url',
            input: r.value,
            inst: e,
            continue: !t.abort
          });
        }
      }));
  }),
  Nd = d('$ZodEmoji', (e, t) => {
    (t.pattern ?? (t.pattern = ed()), re.init(e, t));
  }),
  Dd = d('$ZodNanoID', (e, t) => {
    (t.pattern ?? (t.pattern = W_), re.init(e, t));
  }),
  Zd = d('$ZodCUID', (e, t) => {
    (t.pattern ?? (t.pattern = U_), re.init(e, t));
  }),
  xd = d('$ZodCUID2', (e, t) => {
    (t.pattern ?? (t.pattern = G_), re.init(e, t));
  }),
  Ld = d('$ZodULID', (e, t) => {
    (t.pattern ?? (t.pattern = B_), re.init(e, t));
  }),
  Md = d('$ZodXID', (e, t) => {
    (t.pattern ?? (t.pattern = H_), re.init(e, t));
  }),
  Vd = d('$ZodKSUID', (e, t) => {
    (t.pattern ?? (t.pattern = J_), re.init(e, t));
  }),
  jd = d('$ZodISODateTime', (e, t) => {
    (t.pattern ?? (t.pattern = ld(t)), re.init(e, t));
  }),
  Fd = d('$ZodISODate', (e, t) => {
    (t.pattern ?? (t.pattern = id), re.init(e, t));
  }),
  Kd = d('$ZodISOTime', (e, t) => {
    (t.pattern ?? (t.pattern = cd(t)), re.init(e, t));
  }),
  Ud = d('$ZodISODuration', (e, t) => {
    (t.pattern ?? (t.pattern = q_), re.init(e, t));
  }),
  Gd = d('$ZodIPv4', (e, t) => {
    (t.pattern ?? (t.pattern = td), re.init(e, t), (e._zod.bag.format = 'ipv4'));
  }),
  Bd = d('$ZodIPv6', (e, t) => {
    (t.pattern ?? (t.pattern = rd),
      re.init(e, t),
      (e._zod.bag.format = 'ipv6'),
      (e._zod.check = (r) => {
        try {
          new URL(`http://[${r.value}]`);
        } catch {
          r.issues.push({
            code: 'invalid_format',
            format: 'ipv6',
            input: r.value,
            inst: e,
            continue: !t.abort
          });
        }
      }));
  }),
  Hd = d('$ZodCIDRv4', (e, t) => {
    (t.pattern ?? (t.pattern = nd), re.init(e, t));
  }),
  Jd = d('$ZodCIDRv6', (e, t) => {
    (t.pattern ?? (t.pattern = od),
      re.init(e, t),
      (e._zod.check = (r) => {
        const n = r.value.split('/');
        try {
          if (n.length !== 2) throw new Error();
          const [o, a] = n;
          if (!a) throw new Error();
          const s = Number(a);
          if (`${s}` !== a) throw new Error();
          if (s < 0 || s > 128) throw new Error();
          new URL(`http://[${o}]`);
        } catch {
          r.issues.push({
            code: 'invalid_format',
            format: 'cidrv6',
            input: r.value,
            inst: e,
            continue: !t.abort
          });
        }
      }));
  });
function hs(e) {
  if (e === '') return !0;
  if (e.length % 4 !== 0) return !1;
  try {
    return (atob(e), !0);
  } catch {
    return !1;
  }
}
const Wd = d('$ZodBase64', (e, t) => {
  (t.pattern ?? (t.pattern = ad),
    re.init(e, t),
    (e._zod.bag.contentEncoding = 'base64'),
    (e._zod.check = (r) => {
      hs(r.value) ||
        r.issues.push({
          code: 'invalid_format',
          format: 'base64',
          input: r.value,
          inst: e,
          continue: !t.abort
        });
    }));
});
function qd(e) {
  if (!_s.test(e)) return !1;
  const t = e.replace(/[-_]/g, (n) => (n === '-' ? '+' : '/')),
    r = t.padEnd(Math.ceil(t.length / 4) * 4, '=');
  return hs(r);
}
const Xd = d('$ZodBase64URL', (e, t) => {
    (t.pattern ?? (t.pattern = _s),
      re.init(e, t),
      (e._zod.bag.contentEncoding = 'base64url'),
      (e._zod.check = (r) => {
        qd(r.value) ||
          r.issues.push({
            code: 'invalid_format',
            format: 'base64url',
            input: r.value,
            inst: e,
            continue: !t.abort
          });
      }));
  }),
  Yd = d('$ZodE164', (e, t) => {
    (t.pattern ?? (t.pattern = sd), re.init(e, t));
  });
function Qd(e, t = null) {
  try {
    const r = e.split('.');
    if (r.length !== 3) return !1;
    const [n] = r;
    if (!n) return !1;
    const o = JSON.parse(atob(n));
    return !(('typ' in o && o?.typ !== 'JWT') || !o.alg || (t && (!('alg' in o) || o.alg !== t)));
  } catch {
    return !1;
  }
}
const ep = d('$ZodJWT', (e, t) => {
    (re.init(e, t),
      (e._zod.check = (r) => {
        Qd(r.value, t.alg) ||
          r.issues.push({
            code: 'invalid_format',
            format: 'jwt',
            input: r.value,
            inst: e,
            continue: !t.abort
          });
      }));
  }),
  vs = d('$ZodNumber', (e, t) => {
    (ce.init(e, t),
      (e._zod.pattern = e._zod.bag.pattern ?? dd),
      (e._zod.parse = (r, n) => {
        if (t.coerce)
          try {
            r.value = Number(r.value);
          } catch {}
        const o = r.value;
        if (typeof o == 'number' && !Number.isNaN(o) && Number.isFinite(o)) return r;
        const a =
          typeof o == 'number'
            ? Number.isNaN(o)
              ? 'NaN'
              : Number.isFinite(o)
                ? void 0
                : 'Infinity'
            : void 0;
        return (
          r.issues.push({
            expected: 'number',
            code: 'invalid_type',
            input: o,
            inst: e,
            ...(a ? { received: a } : {})
          }),
          r
        );
      }));
  }),
  tp = d('$ZodNumberFormat', (e, t) => {
    (hd.init(e, t), vs.init(e, t));
  }),
  rp = d('$ZodBoolean', (e, t) => {
    (ce.init(e, t),
      (e._zod.pattern = pd),
      (e._zod.parse = (r, n) => {
        if (t.coerce)
          try {
            r.value = !!r.value;
          } catch {}
        const o = r.value;
        return (
          typeof o == 'boolean' ||
            r.issues.push({ expected: 'boolean', code: 'invalid_type', input: o, inst: e }),
          r
        );
      }));
  }),
  np = d('$ZodUnknown', (e, t) => {
    (ce.init(e, t), (e._zod.parse = (r) => r));
  }),
  op = d('$ZodNever', (e, t) => {
    (ce.init(e, t),
      (e._zod.parse = (r, n) => (
        r.issues.push({ expected: 'never', code: 'invalid_type', input: r.value, inst: e }),
        r
      )));
  });
function Oa(e, t, r) {
  (e.issues.length && t.issues.push(...is(r, e.issues)), (t.value[r] = e.value));
}
const ap = d('$ZodArray', (e, t) => {
  (ce.init(e, t),
    (e._zod.parse = (r, n) => {
      const o = r.value;
      if (!Array.isArray(o))
        return (r.issues.push({ expected: 'array', code: 'invalid_type', input: o, inst: e }), r);
      r.value = Array(o.length);
      const a = [];
      for (let s = 0; s < o.length; s++) {
        const i = o[s],
          c = t.element._zod.run({ value: i, issues: [] }, n);
        c instanceof Promise ? a.push(c.then((u) => Oa(u, r, s))) : Oa(c, r, s);
      }
      return a.length ? Promise.all(a).then(() => r) : r;
    }));
});
function vt(e, t, r, n) {
  (e.issues.length && t.issues.push(...is(r, e.issues)),
    e.value === void 0 ? r in n && (t.value[r] = void 0) : (t.value[r] = e.value));
}
function ys(e) {
  const t = Object.keys(e.shape);
  for (const n of t)
    if (!e.shape?.[n]?._zod?.traits?.has('$ZodType'))
      throw new Error(`Invalid element at key "${n}": expected a Zod schema`);
  const r = E_(e.shape);
  return { ...e, keys: t, keySet: new Set(t), numKeys: t.length, optionalKeys: new Set(r) };
}
function bs(e, t, r, n, o, a) {
  const s = [],
    i = o.keySet,
    c = o.catchall._zod,
    u = c.def.type;
  for (const g in t) {
    if (i.has(g)) continue;
    if (u === 'never') {
      s.push(g);
      continue;
    }
    const C = c.run({ value: t[g], issues: [] }, n);
    C instanceof Promise ? e.push(C.then((l) => vt(l, r, g, t))) : vt(C, r, g, t);
  }
  return (
    s.length && r.issues.push({ code: 'unrecognized_keys', keys: s, input: t, inst: a }),
    e.length ? Promise.all(e).then(() => r) : r
  );
}
const sp = d('$ZodObject', (e, t) => {
    if ((ce.init(e, t), !Object.getOwnPropertyDescriptor(t, 'shape')?.get)) {
      const i = t.shape;
      Object.defineProperty(t, 'shape', {
        get: () => {
          const c = { ...i };
          return (Object.defineProperty(t, 'shape', { value: c }), c);
        }
      });
    }
    const n = Et(() => ys(t));
    J(e._zod, 'propValues', () => {
      const i = t.shape,
        c = {};
      for (const u in i) {
        const g = i[u]._zod;
        if (g.values) {
          c[u] ?? (c[u] = new Set());
          for (const C of g.values) c[u].add(C);
        }
      }
      return c;
    });
    const o = nt,
      a = t.catchall;
    let s;
    e._zod.parse = (i, c) => {
      s ?? (s = n.value);
      const u = i.value;
      if (!o(u))
        return (i.issues.push({ expected: 'object', code: 'invalid_type', input: u, inst: e }), i);
      i.value = {};
      const g = [],
        C = s.shape;
      for (const l of s.keys) {
        const k = C[l]._zod.run({ value: u[l], issues: [] }, c);
        k instanceof Promise ? g.push(k.then((j) => vt(j, i, l, u))) : vt(k, i, l, u);
      }
      return a ? bs(g, u, i, c, n.value, e) : g.length ? Promise.all(g).then(() => i) : i;
    };
  }),
  ip = d('$ZodObjectJIT', (e, t) => {
    sp.init(e, t);
    const r = e._zod.parse,
      n = Et(() => ys(t)),
      o = (l) => {
        const m = new Td(['shape', 'payload', 'ctx']),
          k = n.value,
          j = (le) => {
            const ge = za(le);
            return `shape[${ge}]._zod.run({ value: input[${ge}], issues: [] }, ctx)`;
          };
        m.write('const input = payload.value;');
        const pe = Object.create(null);
        let se = 0;
        for (const le of k.keys) pe[le] = `key_${se++}`;
        m.write('const newResult = {};');
        for (const le of k.keys) {
          const ge = pe[le],
            Ae = za(le);
          (m.write(`const ${ge} = ${j(le)};`),
            m.write(`
        if (${ge}.issues.length) {
          payload.issues = payload.issues.concat(${ge}.issues.map(iss => ({
            ...iss,
            path: iss.path ? [${Ae}, ...iss.path] : [${Ae}]
          })));
        }
        
        
        if (${ge}.value === undefined) {
          if (${Ae} in input) {
            newResult[${Ae}] = undefined;
          }
        } else {
          newResult[${Ae}] = ${ge}.value;
        }
        
      `));
        }
        (m.write('payload.value = newResult;'), m.write('return payload;'));
        const R = m.compile();
        return (le, ge) => R(l, le, ge);
      };
    let a;
    const s = nt,
      i = !ns.jitless,
      u = i && w_.value,
      g = t.catchall;
    let C;
    e._zod.parse = (l, m) => {
      C ?? (C = n.value);
      const k = l.value;
      return s(k)
        ? i && u && m?.async === !1 && m.jitless !== !0
          ? (a || (a = o(t.shape)), (l = a(l, m)), g ? bs([], k, l, m, C, e) : l)
          : r(l, m)
        : (l.issues.push({ expected: 'object', code: 'invalid_type', input: k, inst: e }), l);
    };
  });
function Pa(e, t, r, n) {
  for (const a of e) if (a.issues.length === 0) return ((t.value = a.value), t);
  const o = e.filter((a) => !Ke(a));
  return o.length === 1
    ? ((t.value = o[0].value), o[0])
    : (t.issues.push({
        code: 'invalid_union',
        input: t.value,
        inst: r,
        errors: e.map((a) => a.issues.map((s) => Ze(s, n, De())))
      }),
      t);
}
const ws = d('$ZodUnion', (e, t) => {
    (ce.init(e, t),
      J(e._zod, 'optin', () =>
        t.options.some((o) => o._zod.optin === 'optional') ? 'optional' : void 0
      ),
      J(e._zod, 'optout', () =>
        t.options.some((o) => o._zod.optout === 'optional') ? 'optional' : void 0
      ),
      J(e._zod, 'values', () => {
        if (t.options.every((o) => o._zod.values))
          return new Set(t.options.flatMap((o) => Array.from(o._zod.values)));
      }),
      J(e._zod, 'pattern', () => {
        if (t.options.every((o) => o._zod.pattern)) {
          const o = t.options.map((a) => a._zod.pattern);
          return new RegExp(`^(${o.map((a) => va(a.source)).join('|')})$`);
        }
      }));
    const r = t.options.length === 1,
      n = t.options[0]._zod.run;
    e._zod.parse = (o, a) => {
      if (r) return n(o, a);
      let s = !1;
      const i = [];
      for (const c of t.options) {
        const u = c._zod.run({ value: o.value, issues: [] }, a);
        if (u instanceof Promise) (i.push(u), (s = !0));
        else {
          if (u.issues.length === 0) return u;
          i.push(u);
        }
      }
      return s ? Promise.all(i).then((c) => Pa(c, o, e, a)) : Pa(i, o, e, a);
    };
  }),
  cp = d('$ZodDiscriminatedUnion', (e, t) => {
    ((t.inclusive = !1), ws.init(e, t));
    const r = e._zod.parse;
    J(e._zod, 'propValues', () => {
      const o = {};
      for (const a of t.options) {
        const s = a._zod.propValues;
        if (!s || Object.keys(s).length === 0)
          throw new Error(`Invalid discriminated union option at index "${t.options.indexOf(a)}"`);
        for (const [i, c] of Object.entries(s)) {
          o[i] || (o[i] = new Set());
          for (const u of c) o[i].add(u);
        }
      }
      return o;
    });
    const n = Et(() => {
      const o = t.options,
        a = new Map();
      for (const s of o) {
        const i = s._zod.propValues?.[t.discriminator];
        if (!i || i.size === 0)
          throw new Error(`Invalid discriminated union option at index "${t.options.indexOf(s)}"`);
        for (const c of i) {
          if (a.has(c)) throw new Error(`Duplicate discriminator value "${String(c)}"`);
          a.set(c, s);
        }
      }
      return a;
    });
    e._zod.parse = (o, a) => {
      const s = o.value;
      if (!nt(s))
        return (o.issues.push({ code: 'invalid_type', expected: 'object', input: s, inst: e }), o);
      const i = n.value.get(s?.[t.discriminator]);
      return i
        ? i._zod.run(o, a)
        : t.unionFallback
          ? r(o, a)
          : (o.issues.push({
              code: 'invalid_union',
              errors: [],
              note: 'No matching discriminator',
              discriminator: t.discriminator,
              input: s,
              path: [t.discriminator],
              inst: e
            }),
            o);
    };
  }),
  lp = d('$ZodIntersection', (e, t) => {
    (ce.init(e, t),
      (e._zod.parse = (r, n) => {
        const o = r.value,
          a = t.left._zod.run({ value: o, issues: [] }, n),
          s = t.right._zod.run({ value: o, issues: [] }, n);
        return a instanceof Promise || s instanceof Promise
          ? Promise.all([a, s]).then(([c, u]) => Ra(r, c, u))
          : Ra(r, a, s);
      }));
  });
function Zt(e, t) {
  if (e === t) return { valid: !0, data: e };
  if (e instanceof Date && t instanceof Date && +e == +t) return { valid: !0, data: e };
  if (ot(e) && ot(t)) {
    const r = Object.keys(t),
      n = Object.keys(e).filter((a) => r.indexOf(a) !== -1),
      o = { ...e, ...t };
    for (const a of n) {
      const s = Zt(e[a], t[a]);
      if (!s.valid) return { valid: !1, mergeErrorPath: [a, ...s.mergeErrorPath] };
      o[a] = s.data;
    }
    return { valid: !0, data: o };
  }
  if (Array.isArray(e) && Array.isArray(t)) {
    if (e.length !== t.length) return { valid: !1, mergeErrorPath: [] };
    const r = [];
    for (let n = 0; n < e.length; n++) {
      const o = e[n],
        a = t[n],
        s = Zt(o, a);
      if (!s.valid) return { valid: !1, mergeErrorPath: [n, ...s.mergeErrorPath] };
      r.push(s.data);
    }
    return { valid: !0, data: r };
  }
  return { valid: !1, mergeErrorPath: [] };
}
function Ra(e, t, r) {
  if (
    (t.issues.length && e.issues.push(...t.issues),
    r.issues.length && e.issues.push(...r.issues),
    Ke(e))
  )
    return e;
  const n = Zt(t.value, r.value);
  if (!n.valid)
    throw new Error(`Unmergable intersection. Error path: ${JSON.stringify(n.mergeErrorPath)}`);
  return ((e.value = n.data), e);
}
const up = d('$ZodEnum', (e, t) => {
    ce.init(e, t);
    const r = os(t.entries),
      n = new Set(r);
    ((e._zod.values = n),
      (e._zod.pattern = new RegExp(
        `^(${r
          .filter((o) => I_.has(typeof o))
          .map((o) => (typeof o == 'string' ? Be(o) : o.toString()))
          .join('|')})$`
      )),
      (e._zod.parse = (o, a) => {
        const s = o.value;
        return (
          n.has(s) || o.issues.push({ code: 'invalid_value', values: r, input: s, inst: e }),
          o
        );
      }));
  }),
  _p = d('$ZodLiteral', (e, t) => {
    if ((ce.init(e, t), t.values.length === 0))
      throw new Error('Cannot create literal schema with no valid values');
    const r = new Set(t.values);
    ((e._zod.values = r),
      (e._zod.pattern = new RegExp(
        `^(${t.values.map((n) => (typeof n == 'string' ? Be(n) : n ? Be(n.toString()) : String(n))).join('|')})$`
      )),
      (e._zod.parse = (n, o) => {
        const a = n.value;
        return (
          r.has(a) || n.issues.push({ code: 'invalid_value', values: t.values, input: a, inst: e }),
          n
        );
      }));
  }),
  dp = d('$ZodTransform', (e, t) => {
    (ce.init(e, t),
      (e._zod.parse = (r, n) => {
        if (n.direction === 'backward') throw new rs(e.constructor.name);
        const o = t.transform(r.value, r);
        if (n.async)
          return (o instanceof Promise ? o : Promise.resolve(o)).then((s) => ((r.value = s), r));
        if (o instanceof Promise) throw new Ge();
        return ((r.value = o), r);
      }));
  });
function Na(e, t) {
  return e.issues.length && t === void 0 ? { issues: [], value: void 0 } : e;
}
const pp = d('$ZodOptional', (e, t) => {
    (ce.init(e, t),
      (e._zod.optin = 'optional'),
      (e._zod.optout = 'optional'),
      J(e._zod, 'values', () =>
        t.innerType._zod.values ? new Set([...t.innerType._zod.values, void 0]) : void 0
      ),
      J(e._zod, 'pattern', () => {
        const r = t.innerType._zod.pattern;
        return r ? new RegExp(`^(${va(r.source)})?$`) : void 0;
      }),
      (e._zod.parse = (r, n) => {
        if (t.innerType._zod.optin === 'optional') {
          const o = t.innerType._zod.run(r, n);
          return o instanceof Promise ? o.then((a) => Na(a, r.value)) : Na(o, r.value);
        }
        return r.value === void 0 ? r : t.innerType._zod.run(r, n);
      }));
  }),
  mp = d('$ZodNullable', (e, t) => {
    (ce.init(e, t),
      J(e._zod, 'optin', () => t.innerType._zod.optin),
      J(e._zod, 'optout', () => t.innerType._zod.optout),
      J(e._zod, 'pattern', () => {
        const r = t.innerType._zod.pattern;
        return r ? new RegExp(`^(${va(r.source)}|null)$`) : void 0;
      }),
      J(e._zod, 'values', () =>
        t.innerType._zod.values ? new Set([...t.innerType._zod.values, null]) : void 0
      ),
      (e._zod.parse = (r, n) => (r.value === null ? r : t.innerType._zod.run(r, n))));
  }),
  fp = d('$ZodDefault', (e, t) => {
    (ce.init(e, t),
      (e._zod.optin = 'optional'),
      J(e._zod, 'values', () => t.innerType._zod.values),
      (e._zod.parse = (r, n) => {
        if (n.direction === 'backward') return t.innerType._zod.run(r, n);
        if (r.value === void 0) return ((r.value = t.defaultValue), r);
        const o = t.innerType._zod.run(r, n);
        return o instanceof Promise ? o.then((a) => Da(a, t)) : Da(o, t);
      }));
  });
function Da(e, t) {
  return (e.value === void 0 && (e.value = t.defaultValue), e);
}
const gp = d('$ZodPrefault', (e, t) => {
    (ce.init(e, t),
      (e._zod.optin = 'optional'),
      J(e._zod, 'values', () => t.innerType._zod.values),
      (e._zod.parse = (r, n) => (
        n.direction === 'backward' || (r.value === void 0 && (r.value = t.defaultValue)),
        t.innerType._zod.run(r, n)
      )));
  }),
  hp = d('$ZodNonOptional', (e, t) => {
    (ce.init(e, t),
      J(e._zod, 'values', () => {
        const r = t.innerType._zod.values;
        return r ? new Set([...r].filter((n) => n !== void 0)) : void 0;
      }),
      (e._zod.parse = (r, n) => {
        const o = t.innerType._zod.run(r, n);
        return o instanceof Promise ? o.then((a) => Za(a, e)) : Za(o, e);
      }));
  });
function Za(e, t) {
  return (
    !e.issues.length &&
      e.value === void 0 &&
      e.issues.push({ code: 'invalid_type', expected: 'nonoptional', input: e.value, inst: t }),
    e
  );
}
const vp = d('$ZodCatch', (e, t) => {
    (ce.init(e, t),
      J(e._zod, 'optin', () => t.innerType._zod.optin),
      J(e._zod, 'optout', () => t.innerType._zod.optout),
      J(e._zod, 'values', () => t.innerType._zod.values),
      (e._zod.parse = (r, n) => {
        if (n.direction === 'backward') return t.innerType._zod.run(r, n);
        const o = t.innerType._zod.run(r, n);
        return o instanceof Promise
          ? o.then(
              (a) => (
                (r.value = a.value),
                a.issues.length &&
                  ((r.value = t.catchValue({
                    ...r,
                    error: { issues: a.issues.map((s) => Ze(s, n, De())) },
                    input: r.value
                  })),
                  (r.issues = [])),
                r
              )
            )
          : ((r.value = o.value),
            o.issues.length &&
              ((r.value = t.catchValue({
                ...r,
                error: { issues: o.issues.map((a) => Ze(a, n, De())) },
                input: r.value
              })),
              (r.issues = [])),
            r);
      }));
  }),
  yp = d('$ZodPipe', (e, t) => {
    (ce.init(e, t),
      J(e._zod, 'values', () => t.in._zod.values),
      J(e._zod, 'optin', () => t.in._zod.optin),
      J(e._zod, 'optout', () => t.out._zod.optout),
      J(e._zod, 'propValues', () => t.in._zod.propValues),
      (e._zod.parse = (r, n) => {
        if (n.direction === 'backward') {
          const a = t.out._zod.run(r, n);
          return a instanceof Promise ? a.then((s) => mt(s, t.in, n)) : mt(a, t.in, n);
        }
        const o = t.in._zod.run(r, n);
        return o instanceof Promise ? o.then((a) => mt(a, t.out, n)) : mt(o, t.out, n);
      }));
  });
function mt(e, t, r) {
  return e.issues.length
    ? ((e.aborted = !0), e)
    : t._zod.run({ value: e.value, issues: e.issues }, r);
}
const bp = d('$ZodReadonly', (e, t) => {
  (ce.init(e, t),
    J(e._zod, 'propValues', () => t.innerType._zod.propValues),
    J(e._zod, 'values', () => t.innerType._zod.values),
    J(e._zod, 'optin', () => t.innerType?._zod?.optin),
    J(e._zod, 'optout', () => t.innerType?._zod?.optout),
    (e._zod.parse = (r, n) => {
      if (n.direction === 'backward') return t.innerType._zod.run(r, n);
      const o = t.innerType._zod.run(r, n);
      return o instanceof Promise ? o.then(xa) : xa(o);
    }));
});
function xa(e) {
  return ((e.value = Object.freeze(e.value)), e);
}
const wp = d('$ZodCustom', (e, t) => {
  (Ee.init(e, t),
    ce.init(e, t),
    (e._zod.parse = (r, n) => r),
    (e._zod.check = (r) => {
      const n = r.value,
        o = t.fn(n);
      if (o instanceof Promise) return o.then((a) => La(a, r, n, e));
      La(o, r, n, e);
    }));
});
function La(e, t, r, n) {
  if (!e) {
    const o = {
      code: 'custom',
      input: r,
      inst: n,
      path: [...(n._zod.def.path ?? [])],
      continue: !n._zod.def.abort
    };
    (n._zod.def.params && (o.params = n._zod.def.params), t.issues.push(at(o)));
  }
}
var Ma;
class Ip {
  constructor() {
    ((this._map = new WeakMap()), (this._idmap = new Map()));
  }
  add(t, ...r) {
    const n = r[0];
    if ((this._map.set(t, n), n && typeof n == 'object' && 'id' in n)) {
      if (this._idmap.has(n.id)) throw new Error(`ID ${n.id} already exists in the registry`);
      this._idmap.set(n.id, t);
    }
    return this;
  }
  clear() {
    return ((this._map = new WeakMap()), (this._idmap = new Map()), this);
  }
  remove(t) {
    const r = this._map.get(t);
    return (
      r && typeof r == 'object' && 'id' in r && this._idmap.delete(r.id),
      this._map.delete(t),
      this
    );
  }
  get(t) {
    const r = t._zod.parent;
    if (r) {
      const n = { ...(this.get(r) ?? {}) };
      delete n.id;
      const o = { ...n, ...this._map.get(t) };
      return Object.keys(o).length ? o : void 0;
    }
    return this._map.get(t);
  }
  has(t) {
    return this._map.has(t);
  }
}
function Ep() {
  return new Ip();
}
(Ma = globalThis).__zod_globalRegistry ?? (Ma.__zod_globalRegistry = Ep());
const rt = globalThis.__zod_globalRegistry;
function Sp(e, t) {
  return new e({ type: 'string', ...T(t) });
}
function Cp(e, t) {
  return new e({ type: 'string', format: 'email', check: 'string_format', abort: !1, ...T(t) });
}
function Va(e, t) {
  return new e({ type: 'string', format: 'guid', check: 'string_format', abort: !1, ...T(t) });
}
function kp(e, t) {
  return new e({ type: 'string', format: 'uuid', check: 'string_format', abort: !1, ...T(t) });
}
function Ap(e, t) {
  return new e({
    type: 'string',
    format: 'uuid',
    check: 'string_format',
    abort: !1,
    version: 'v4',
    ...T(t)
  });
}
function Tp(e, t) {
  return new e({
    type: 'string',
    format: 'uuid',
    check: 'string_format',
    abort: !1,
    version: 'v6',
    ...T(t)
  });
}
function zp(e, t) {
  return new e({
    type: 'string',
    format: 'uuid',
    check: 'string_format',
    abort: !1,
    version: 'v7',
    ...T(t)
  });
}
function $p(e, t) {
  return new e({ type: 'string', format: 'url', check: 'string_format', abort: !1, ...T(t) });
}
function Op(e, t) {
  return new e({ type: 'string', format: 'emoji', check: 'string_format', abort: !1, ...T(t) });
}
function Pp(e, t) {
  return new e({ type: 'string', format: 'nanoid', check: 'string_format', abort: !1, ...T(t) });
}
function Rp(e, t) {
  return new e({ type: 'string', format: 'cuid', check: 'string_format', abort: !1, ...T(t) });
}
function Np(e, t) {
  return new e({ type: 'string', format: 'cuid2', check: 'string_format', abort: !1, ...T(t) });
}
function Dp(e, t) {
  return new e({ type: 'string', format: 'ulid', check: 'string_format', abort: !1, ...T(t) });
}
function Zp(e, t) {
  return new e({ type: 'string', format: 'xid', check: 'string_format', abort: !1, ...T(t) });
}
function xp(e, t) {
  return new e({ type: 'string', format: 'ksuid', check: 'string_format', abort: !1, ...T(t) });
}
function Lp(e, t) {
  return new e({ type: 'string', format: 'ipv4', check: 'string_format', abort: !1, ...T(t) });
}
function Mp(e, t) {
  return new e({ type: 'string', format: 'ipv6', check: 'string_format', abort: !1, ...T(t) });
}
function Vp(e, t) {
  return new e({ type: 'string', format: 'cidrv4', check: 'string_format', abort: !1, ...T(t) });
}
function jp(e, t) {
  return new e({ type: 'string', format: 'cidrv6', check: 'string_format', abort: !1, ...T(t) });
}
function Fp(e, t) {
  return new e({ type: 'string', format: 'base64', check: 'string_format', abort: !1, ...T(t) });
}
function Kp(e, t) {
  return new e({ type: 'string', format: 'base64url', check: 'string_format', abort: !1, ...T(t) });
}
function Up(e, t) {
  return new e({ type: 'string', format: 'e164', check: 'string_format', abort: !1, ...T(t) });
}
function Gp(e, t) {
  return new e({ type: 'string', format: 'jwt', check: 'string_format', abort: !1, ...T(t) });
}
function Bp(e, t) {
  return new e({
    type: 'string',
    format: 'datetime',
    check: 'string_format',
    offset: !1,
    local: !1,
    precision: null,
    ...T(t)
  });
}
function Hp(e, t) {
  return new e({ type: 'string', format: 'date', check: 'string_format', ...T(t) });
}
function Jp(e, t) {
  return new e({
    type: 'string',
    format: 'time',
    check: 'string_format',
    precision: null,
    ...T(t)
  });
}
function Wp(e, t) {
  return new e({ type: 'string', format: 'duration', check: 'string_format', ...T(t) });
}
function qp(e, t) {
  return new e({ type: 'number', checks: [], ...T(t) });
}
function Xp(e, t) {
  return new e({ type: 'number', check: 'number_format', abort: !1, format: 'safeint', ...T(t) });
}
function Yp(e, t) {
  return new e({ type: 'boolean', ...T(t) });
}
function Qp(e) {
  return new e({ type: 'unknown' });
}
function em(e, t) {
  return new e({ type: 'never', ...T(t) });
}
function ja(e, t) {
  return new fs({ check: 'less_than', ...T(t), value: e, inclusive: !1 });
}
function Pt(e, t) {
  return new fs({ check: 'less_than', ...T(t), value: e, inclusive: !0 });
}
function Fa(e, t) {
  return new gs({ check: 'greater_than', ...T(t), value: e, inclusive: !1 });
}
function Rt(e, t) {
  return new gs({ check: 'greater_than', ...T(t), value: e, inclusive: !0 });
}
function Ka(e, t) {
  return new gd({ check: 'multiple_of', ...T(t), value: e });
}
function Is(e, t) {
  return new vd({ check: 'max_length', ...T(t), maximum: e });
}
function yt(e, t) {
  return new yd({ check: 'min_length', ...T(t), minimum: e });
}
function Es(e, t) {
  return new bd({ check: 'length_equals', ...T(t), length: e });
}
function tm(e, t) {
  return new wd({ check: 'string_format', format: 'regex', ...T(t), pattern: e });
}
function rm(e) {
  return new Id({ check: 'string_format', format: 'lowercase', ...T(e) });
}
function nm(e) {
  return new Ed({ check: 'string_format', format: 'uppercase', ...T(e) });
}
function om(e, t) {
  return new Sd({ check: 'string_format', format: 'includes', ...T(t), includes: e });
}
function am(e, t) {
  return new Cd({ check: 'string_format', format: 'starts_with', ...T(t), prefix: e });
}
function sm(e, t) {
  return new kd({ check: 'string_format', format: 'ends_with', ...T(t), suffix: e });
}
function Xe(e) {
  return new Ad({ check: 'overwrite', tx: e });
}
function im(e) {
  return Xe((t) => t.normalize(e));
}
function cm() {
  return Xe((e) => e.trim());
}
function lm() {
  return Xe((e) => e.toLowerCase());
}
function um() {
  return Xe((e) => e.toUpperCase());
}
function _m() {
  return Xe((e) => b_(e));
}
function dm(e, t, r) {
  return new e({ type: 'array', element: t, ...T(r) });
}
function pm(e, t, r) {
  return new e({ type: 'custom', check: 'custom', fn: t, ...T(r) });
}
function mm(e) {
  const t = fm(
    (r) => (
      (r.addIssue = (n) => {
        if (typeof n == 'string') r.issues.push(at(n, r.value, t._zod.def));
        else {
          const o = n;
          (o.fatal && (o.continue = !1),
            o.code ?? (o.code = 'custom'),
            o.input ?? (o.input = r.value),
            o.inst ?? (o.inst = t),
            o.continue ?? (o.continue = !t._zod.def.abort),
            r.issues.push(at(o)));
        }
      }),
      e(r.value, r)
    )
  );
  return t;
}
function fm(e, t) {
  const r = new Ee({ check: 'custom', ...T(t) });
  return ((r._zod.check = e), r);
}
function Ss(e) {
  let t = e?.target ?? 'draft-2020-12';
  return (
    t === 'draft-4' && (t = 'draft-04'),
    t === 'draft-7' && (t = 'draft-07'),
    {
      processors: e.processors ?? {},
      metadataRegistry: e?.metadata ?? rt,
      target: t,
      unrepresentable: e?.unrepresentable ?? 'throw',
      override: e?.override ?? (() => {}),
      io: e?.io ?? 'output',
      counter: 0,
      seen: new Map(),
      cycles: e?.cycles ?? 'ref',
      reused: e?.reused ?? 'inline',
      external: e?.external ?? void 0
    }
  );
}
function fe(e, t, r = { path: [], schemaPath: [] }) {
  var n;
  const o = e._zod.def,
    a = t.seen.get(e);
  if (a) return (a.count++, r.schemaPath.includes(e) && (a.cycle = r.path), a.schema);
  const s = { schema: {}, count: 1, cycle: void 0, path: r.path };
  t.seen.set(e, s);
  const i = e._zod.toJSONSchema?.();
  if (i) s.schema = i;
  else {
    const g = { ...r, schemaPath: [...r.schemaPath, e], path: r.path },
      C = e._zod.parent;
    if (C) ((s.ref = C), fe(C, t, g), (t.seen.get(C).isParent = !0));
    else if (e._zod.processJSONSchema) e._zod.processJSONSchema(t, s.schema, g);
    else {
      const l = s.schema,
        m = t.processors[o.type];
      if (!m) throw new Error(`[toJSONSchema]: Non-representable type encountered: ${o.type}`);
      m(e, t, l, g);
    }
  }
  const c = t.metadataRegistry.get(e);
  return (
    c && Object.assign(s.schema, c),
    t.io === 'input' && be(e) && (delete s.schema.examples, delete s.schema.default),
    t.io === 'input' &&
      s.schema._prefault &&
      ((n = s.schema).default ?? (n.default = s.schema._prefault)),
    delete s.schema._prefault,
    t.seen.get(e).schema
  );
}
function Cs(e, t) {
  const r = e.seen.get(t);
  if (!r) throw new Error('Unprocessed schema. This is a bug in Zod.');
  const n = (a) => {
      const s = e.target === 'draft-2020-12' ? '$defs' : 'definitions';
      if (e.external) {
        const g = e.external.registry.get(a[0])?.id,
          C = e.external.uri ?? ((m) => m);
        if (g) return { ref: C(g) };
        const l = a[1].defId ?? a[1].schema.id ?? `schema${e.counter++}`;
        return ((a[1].defId = l), { defId: l, ref: `${C('__shared')}#/${s}/${l}` });
      }
      if (a[1] === r) return { ref: '#' };
      const c = `#/${s}/`,
        u = a[1].schema.id ?? `__schema${e.counter++}`;
      return { defId: u, ref: c + u };
    },
    o = (a) => {
      if (a[1].schema.$ref) return;
      const s = a[1],
        { ref: i, defId: c } = n(a);
      ((s.def = { ...s.schema }), c && (s.defId = c));
      const u = s.schema;
      for (const g in u) delete u[g];
      u.$ref = i;
    };
  if (e.cycles === 'throw')
    for (const a of e.seen.entries()) {
      const s = a[1];
      if (s.cycle)
        throw new Error(`Cycle detected: #/${s.cycle?.join('/')}/<root>

Set the \`cycles\` parameter to \`"ref"\` to resolve cyclical schemas with defs.`);
    }
  for (const a of e.seen.entries()) {
    const s = a[1];
    if (t === a[0]) {
      o(a);
      continue;
    }
    if (e.external) {
      const c = e.external.registry.get(a[0])?.id;
      if (t !== a[0] && c) {
        o(a);
        continue;
      }
    }
    if (e.metadataRegistry.get(a[0])?.id) {
      o(a);
      continue;
    }
    if (s.cycle) {
      o(a);
      continue;
    }
    if (s.count > 1 && e.reused === 'ref') {
      o(a);
      continue;
    }
  }
}
function ks(e, t) {
  const r = e.seen.get(t);
  if (!r) throw new Error('Unprocessed schema. This is a bug in Zod.');
  const n = (s) => {
    const i = e.seen.get(s),
      c = i.def ?? i.schema,
      u = { ...c };
    if (i.ref === null) return;
    const g = i.ref;
    if (((i.ref = null), g)) {
      n(g);
      const C = e.seen.get(g).schema;
      C.$ref && (e.target === 'draft-07' || e.target === 'draft-04' || e.target === 'openapi-3.0')
        ? ((c.allOf = c.allOf ?? []), c.allOf.push(C))
        : (Object.assign(c, C), Object.assign(c, u));
    }
    i.isParent || e.override({ zodSchema: s, jsonSchema: c, path: i.path ?? [] });
  };
  for (const s of [...e.seen.entries()].reverse()) n(s[0]);
  const o = {};
  if (
    (e.target === 'draft-2020-12'
      ? (o.$schema = 'https://json-schema.org/draft/2020-12/schema')
      : e.target === 'draft-07'
        ? (o.$schema = 'http://json-schema.org/draft-07/schema#')
        : e.target === 'draft-04'
          ? (o.$schema = 'http://json-schema.org/draft-04/schema#')
          : e.target,
    e.external?.uri)
  ) {
    const s = e.external.registry.get(t)?.id;
    if (!s) throw new Error('Schema is missing an `id` property');
    o.$id = e.external.uri(s);
  }
  Object.assign(o, r.def ?? r.schema);
  const a = e.external?.defs ?? {};
  for (const s of e.seen.entries()) {
    const i = s[1];
    i.def && i.defId && (a[i.defId] = i.def);
  }
  e.external ||
    (Object.keys(a).length > 0 &&
      (e.target === 'draft-2020-12' ? (o.$defs = a) : (o.definitions = a)));
  try {
    const s = JSON.parse(JSON.stringify(o));
    return (
      Object.defineProperty(s, '~standard', {
        value: {
          ...t['~standard'],
          jsonSchema: { input: bt(t, 'input'), output: bt(t, 'output') }
        },
        enumerable: !1,
        writable: !1
      }),
      s
    );
  } catch {
    throw new Error('Error converting schema to JSON.');
  }
}
function be(e, t) {
  const r = t ?? { seen: new Set() };
  if (r.seen.has(e)) return !1;
  r.seen.add(e);
  const n = e._zod.def;
  if (n.type === 'transform') return !0;
  if (n.type === 'array') return be(n.element, r);
  if (n.type === 'set') return be(n.valueType, r);
  if (n.type === 'lazy') return be(n.getter(), r);
  if (
    n.type === 'promise' ||
    n.type === 'optional' ||
    n.type === 'nonoptional' ||
    n.type === 'nullable' ||
    n.type === 'readonly' ||
    n.type === 'default' ||
    n.type === 'prefault'
  )
    return be(n.innerType, r);
  if (n.type === 'intersection') return be(n.left, r) || be(n.right, r);
  if (n.type === 'record' || n.type === 'map') return be(n.keyType, r) || be(n.valueType, r);
  if (n.type === 'pipe') return be(n.in, r) || be(n.out, r);
  if (n.type === 'object') {
    for (const o in n.shape) if (be(n.shape[o], r)) return !0;
    return !1;
  }
  if (n.type === 'union') {
    for (const o of n.options) if (be(o, r)) return !0;
    return !1;
  }
  if (n.type === 'tuple') {
    for (const o of n.items) if (be(o, r)) return !0;
    return !!(n.rest && be(n.rest, r));
  }
  return !1;
}
const gm =
    (e, t = {}) =>
    (r) => {
      const n = Ss({ ...r, processors: t });
      return (fe(e, n), Cs(n, e), ks(n, e));
    },
  bt = (e, t) => (r) => {
    const { libraryOptions: n, target: o } = r ?? {},
      a = Ss({ ...(n ?? {}), target: o, io: t, processors: {} });
    return (fe(e, a), Cs(a, e), ks(a, e));
  },
  hm = { guid: 'uuid', url: 'uri', datetime: 'date-time', json_string: 'json-string', regex: '' },
  vm = (e, t, r, n) => {
    const o = r;
    o.type = 'string';
    const { minimum: a, maximum: s, format: i, patterns: c, contentEncoding: u } = e._zod.bag;
    if (
      (typeof a == 'number' && (o.minLength = a),
      typeof s == 'number' && (o.maxLength = s),
      i && ((o.format = hm[i] ?? i), o.format === '' && delete o.format),
      u && (o.contentEncoding = u),
      c && c.size > 0)
    ) {
      const g = [...c];
      g.length === 1
        ? (o.pattern = g[0].source)
        : g.length > 1 &&
          (o.allOf = [
            ...g.map((C) => ({
              ...(t.target === 'draft-07' || t.target === 'draft-04' || t.target === 'openapi-3.0'
                ? { type: 'string' }
                : {}),
              pattern: C.source
            }))
          ]);
    }
  },
  ym = (e, t, r, n) => {
    const o = r,
      {
        minimum: a,
        maximum: s,
        format: i,
        multipleOf: c,
        exclusiveMaximum: u,
        exclusiveMinimum: g
      } = e._zod.bag;
    (typeof i == 'string' && i.includes('int') ? (o.type = 'integer') : (o.type = 'number'),
      typeof g == 'number' &&
        (t.target === 'draft-04' || t.target === 'openapi-3.0'
          ? ((o.minimum = g), (o.exclusiveMinimum = !0))
          : (o.exclusiveMinimum = g)),
      typeof a == 'number' &&
        ((o.minimum = a),
        typeof g == 'number' &&
          t.target !== 'draft-04' &&
          (g >= a ? delete o.minimum : delete o.exclusiveMinimum)),
      typeof u == 'number' &&
        (t.target === 'draft-04' || t.target === 'openapi-3.0'
          ? ((o.maximum = u), (o.exclusiveMaximum = !0))
          : (o.exclusiveMaximum = u)),
      typeof s == 'number' &&
        ((o.maximum = s),
        typeof u == 'number' &&
          t.target !== 'draft-04' &&
          (u <= s ? delete o.maximum : delete o.exclusiveMaximum)),
      typeof c == 'number' && (o.multipleOf = c));
  },
  bm = (e, t, r, n) => {
    r.type = 'boolean';
  },
  wm = (e, t, r, n) => {
    r.not = {};
  },
  Im = (e, t, r, n) => {},
  Em = (e, t, r, n) => {
    const o = e._zod.def,
      a = os(o.entries);
    (a.every((s) => typeof s == 'number') && (r.type = 'number'),
      a.every((s) => typeof s == 'string') && (r.type = 'string'),
      (r.enum = a));
  },
  Sm = (e, t, r, n) => {
    const o = e._zod.def,
      a = [];
    for (const s of o.values)
      if (s === void 0) {
        if (t.unrepresentable === 'throw')
          throw new Error('Literal `undefined` cannot be represented in JSON Schema');
      } else if (typeof s == 'bigint') {
        if (t.unrepresentable === 'throw')
          throw new Error('BigInt literals cannot be represented in JSON Schema');
        a.push(Number(s));
      } else a.push(s);
    if (a.length !== 0)
      if (a.length === 1) {
        const s = a[0];
        ((r.type = s === null ? 'null' : typeof s),
          t.target === 'draft-04' || t.target === 'openapi-3.0' ? (r.enum = [s]) : (r.const = s));
      } else
        (a.every((s) => typeof s == 'number') && (r.type = 'number'),
          a.every((s) => typeof s == 'string') && (r.type = 'string'),
          a.every((s) => typeof s == 'boolean') && (r.type = 'boolean'),
          a.every((s) => s === null) && (r.type = 'null'),
          (r.enum = a));
  },
  Cm = (e, t, r, n) => {
    if (t.unrepresentable === 'throw')
      throw new Error('Custom types cannot be represented in JSON Schema');
  },
  km = (e, t, r, n) => {
    if (t.unrepresentable === 'throw')
      throw new Error('Transforms cannot be represented in JSON Schema');
  },
  Am = (e, t, r, n) => {
    const o = r,
      a = e._zod.def,
      { minimum: s, maximum: i } = e._zod.bag;
    (typeof s == 'number' && (o.minItems = s),
      typeof i == 'number' && (o.maxItems = i),
      (o.type = 'array'),
      (o.items = fe(a.element, t, { ...n, path: [...n.path, 'items'] })));
  },
  Tm = (e, t, r, n) => {
    const o = r,
      a = e._zod.def;
    ((o.type = 'object'), (o.properties = {}));
    const s = a.shape;
    for (const u in s) o.properties[u] = fe(s[u], t, { ...n, path: [...n.path, 'properties', u] });
    const i = new Set(Object.keys(s)),
      c = new Set(
        [...i].filter((u) => {
          const g = a.shape[u]._zod;
          return t.io === 'input' ? g.optin === void 0 : g.optout === void 0;
        })
      );
    (c.size > 0 && (o.required = Array.from(c)),
      a.catchall?._zod.def.type === 'never'
        ? (o.additionalProperties = !1)
        : a.catchall
          ? a.catchall &&
            (o.additionalProperties = fe(a.catchall, t, {
              ...n,
              path: [...n.path, 'additionalProperties']
            }))
          : t.io === 'output' && (o.additionalProperties = !1));
  },
  zm = (e, t, r, n) => {
    const o = e._zod.def,
      a = o.inclusive === !1,
      s = o.options.map((i, c) => fe(i, t, { ...n, path: [...n.path, a ? 'oneOf' : 'anyOf', c] }));
    a ? (r.oneOf = s) : (r.anyOf = s);
  },
  $m = (e, t, r, n) => {
    const o = e._zod.def,
      a = fe(o.left, t, { ...n, path: [...n.path, 'allOf', 0] }),
      s = fe(o.right, t, { ...n, path: [...n.path, 'allOf', 1] }),
      i = (u) => 'allOf' in u && Object.keys(u).length === 1,
      c = [...(i(a) ? a.allOf : [a]), ...(i(s) ? s.allOf : [s])];
    r.allOf = c;
  },
  Om = (e, t, r, n) => {
    const o = e._zod.def,
      a = fe(o.innerType, t, n),
      s = t.seen.get(e);
    t.target === 'openapi-3.0'
      ? ((s.ref = o.innerType), (r.nullable = !0))
      : (r.anyOf = [a, { type: 'null' }]);
  },
  Pm = (e, t, r, n) => {
    const o = e._zod.def;
    fe(o.innerType, t, n);
    const a = t.seen.get(e);
    a.ref = o.innerType;
  },
  Rm = (e, t, r, n) => {
    const o = e._zod.def;
    fe(o.innerType, t, n);
    const a = t.seen.get(e);
    ((a.ref = o.innerType), (r.default = JSON.parse(JSON.stringify(o.defaultValue))));
  },
  Nm = (e, t, r, n) => {
    const o = e._zod.def;
    fe(o.innerType, t, n);
    const a = t.seen.get(e);
    ((a.ref = o.innerType),
      t.io === 'input' && (r._prefault = JSON.parse(JSON.stringify(o.defaultValue))));
  },
  Dm = (e, t, r, n) => {
    const o = e._zod.def;
    fe(o.innerType, t, n);
    const a = t.seen.get(e);
    a.ref = o.innerType;
    let s;
    try {
      s = o.catchValue(void 0);
    } catch {
      throw new Error('Dynamic catch values are not supported in JSON Schema');
    }
    r.default = s;
  },
  Zm = (e, t, r, n) => {
    const o = e._zod.def,
      a = t.io === 'input' ? (o.in._zod.def.type === 'transform' ? o.out : o.in) : o.out;
    fe(a, t, n);
    const s = t.seen.get(e);
    s.ref = a;
  },
  xm = (e, t, r, n) => {
    const o = e._zod.def;
    fe(o.innerType, t, n);
    const a = t.seen.get(e);
    ((a.ref = o.innerType), (r.readOnly = !0));
  },
  Lm = (e, t, r, n) => {
    const o = e._zod.def;
    fe(o.innerType, t, n);
    const a = t.seen.get(e);
    a.ref = o.innerType;
  },
  Mm = d('ZodISODateTime', (e, t) => {
    (jd.init(e, t), ae.init(e, t));
  });
function Vm(e) {
  return Bp(Mm, e);
}
const jm = d('ZodISODate', (e, t) => {
  (Fd.init(e, t), ae.init(e, t));
});
function Fm(e) {
  return Hp(jm, e);
}
const Km = d('ZodISOTime', (e, t) => {
  (Kd.init(e, t), ae.init(e, t));
});
function Um(e) {
  return Jp(Km, e);
}
const Gm = d('ZodISODuration', (e, t) => {
  (Ud.init(e, t), ae.init(e, t));
});
function Bm(e) {
  return Wp(Gm, e);
}
const Hm = (e, t) => {
    (ls.init(e, t),
      (e.name = 'ZodError'),
      Object.defineProperties(e, {
        format: { value: (r) => R_(e, r) },
        flatten: { value: (r) => P_(e, r) },
        addIssue: {
          value: (r) => {
            (e.issues.push(r), (e.message = JSON.stringify(e.issues, Dt, 2)));
          }
        },
        addIssues: {
          value: (r) => {
            (e.issues.push(...r), (e.message = JSON.stringify(e.issues, Dt, 2)));
          }
        },
        isEmpty: {
          get() {
            return e.issues.length === 0;
          }
        }
      }));
  },
  Te = d('ZodError', Hm, { Parent: Error }),
  Jm = ba(Te),
  Wm = wa(Te),
  qm = St(Te),
  Xm = Ct(Te),
  Ym = Z_(Te),
  Qm = x_(Te),
  ef = L_(Te),
  tf = M_(Te),
  rf = V_(Te),
  nf = j_(Te),
  of = F_(Te),
  af = K_(Te),
  ue = d(
    'ZodType',
    (e, t) => (
      ce.init(e, t),
      Object.assign(e['~standard'], {
        jsonSchema: { input: bt(e, 'input'), output: bt(e, 'output') }
      }),
      (e.toJSONSchema = gm(e, {})),
      (e.def = t),
      (e.type = t.type),
      Object.defineProperty(e, '_def', { value: t }),
      (e.check = (...r) =>
        e.clone(
          Me(t, {
            checks: [
              ...(t.checks ?? []),
              ...r.map((n) =>
                typeof n == 'function'
                  ? { _zod: { check: n, def: { check: 'custom' }, onattach: [] } }
                  : n
              )
            ]
          })
        )),
      (e.clone = (r, n) => Oe(e, r, n)),
      (e.brand = () => e),
      (e.register = (r, n) => (r.add(e, n), e)),
      (e.parse = (r, n) => Jm(e, r, n, { callee: e.parse })),
      (e.safeParse = (r, n) => qm(e, r, n)),
      (e.parseAsync = async (r, n) => Wm(e, r, n, { callee: e.parseAsync })),
      (e.safeParseAsync = async (r, n) => Xm(e, r, n)),
      (e.spa = e.safeParseAsync),
      (e.encode = (r, n) => Ym(e, r, n)),
      (e.decode = (r, n) => Qm(e, r, n)),
      (e.encodeAsync = async (r, n) => ef(e, r, n)),
      (e.decodeAsync = async (r, n) => tf(e, r, n)),
      (e.safeEncode = (r, n) => rf(e, r, n)),
      (e.safeDecode = (r, n) => nf(e, r, n)),
      (e.safeEncodeAsync = async (r, n) => of(e, r, n)),
      (e.safeDecodeAsync = async (r, n) => af(e, r, n)),
      (e.refine = (r, n) => e.check(Yf(r, n))),
      (e.superRefine = (r) => e.check(Qf(r))),
      (e.overwrite = (r) => e.check(Xe(r))),
      (e.optional = () => Ha(e)),
      (e.nullable = () => Ja(e)),
      (e.nullish = () => Ha(Ja(e))),
      (e.nonoptional = (r) => Gf(e, r)),
      (e.array = () => $s(e)),
      (e.or = (r) => Pf([e, r])),
      (e.and = (r) => Zf(e, r)),
      (e.transform = (r) => Wa(e, Mf(r))),
      (e.default = (r) => Ff(e, r)),
      (e.prefault = (r) => Uf(e, r)),
      (e.catch = (r) => Hf(e, r)),
      (e.pipe = (r) => Wa(e, r)),
      (e.readonly = () => qf(e)),
      (e.describe = (r) => {
        const n = e.clone();
        return (rt.add(n, { description: r }), n);
      }),
      Object.defineProperty(e, 'description', {
        get() {
          return rt.get(e)?.description;
        },
        configurable: !0
      }),
      (e.meta = (...r) => {
        if (r.length === 0) return rt.get(e);
        const n = e.clone();
        return (rt.add(n, r[0]), n);
      }),
      (e.isOptional = () => e.safeParse(void 0).success),
      (e.isNullable = () => e.safeParse(null).success),
      e
    )
  ),
  As = d('_ZodString', (e, t) => {
    (Ia.init(e, t), ue.init(e, t), (e._zod.processJSONSchema = (n, o, a) => vm(e, n, o)));
    const r = e._zod.bag;
    ((e.format = r.format ?? null),
      (e.minLength = r.minimum ?? null),
      (e.maxLength = r.maximum ?? null),
      (e.regex = (...n) => e.check(tm(...n))),
      (e.includes = (...n) => e.check(om(...n))),
      (e.startsWith = (...n) => e.check(am(...n))),
      (e.endsWith = (...n) => e.check(sm(...n))),
      (e.min = (...n) => e.check(yt(...n))),
      (e.max = (...n) => e.check(Is(...n))),
      (e.length = (...n) => e.check(Es(...n))),
      (e.nonempty = (...n) => e.check(yt(1, ...n))),
      (e.lowercase = (n) => e.check(rm(n))),
      (e.uppercase = (n) => e.check(nm(n))),
      (e.trim = () => e.check(cm())),
      (e.normalize = (...n) => e.check(im(...n))),
      (e.toLowerCase = () => e.check(lm())),
      (e.toUpperCase = () => e.check(um())),
      (e.slugify = () => e.check(_m())));
  }),
  sf = d('ZodString', (e, t) => {
    (Ia.init(e, t),
      As.init(e, t),
      (e.email = (r) => e.check(Cp(cf, r))),
      (e.url = (r) => e.check($p(lf, r))),
      (e.jwt = (r) => e.check(Gp(Sf, r))),
      (e.emoji = (r) => e.check(Op(uf, r))),
      (e.guid = (r) => e.check(Va(Ua, r))),
      (e.uuid = (r) => e.check(kp(ft, r))),
      (e.uuidv4 = (r) => e.check(Ap(ft, r))),
      (e.uuidv6 = (r) => e.check(Tp(ft, r))),
      (e.uuidv7 = (r) => e.check(zp(ft, r))),
      (e.nanoid = (r) => e.check(Pp(_f, r))),
      (e.guid = (r) => e.check(Va(Ua, r))),
      (e.cuid = (r) => e.check(Rp(df, r))),
      (e.cuid2 = (r) => e.check(Np(pf, r))),
      (e.ulid = (r) => e.check(Dp(mf, r))),
      (e.base64 = (r) => e.check(Fp(wf, r))),
      (e.base64url = (r) => e.check(Kp(If, r))),
      (e.xid = (r) => e.check(Zp(ff, r))),
      (e.ksuid = (r) => e.check(xp(gf, r))),
      (e.ipv4 = (r) => e.check(Lp(hf, r))),
      (e.ipv6 = (r) => e.check(Mp(vf, r))),
      (e.cidrv4 = (r) => e.check(Vp(yf, r))),
      (e.cidrv6 = (r) => e.check(jp(bf, r))),
      (e.e164 = (r) => e.check(Up(Ef, r))),
      (e.datetime = (r) => e.check(Vm(r))),
      (e.date = (r) => e.check(Fm(r))),
      (e.time = (r) => e.check(Um(r))),
      (e.duration = (r) => e.check(Bm(r))));
  });
function G(e) {
  return Sp(sf, e);
}
const ae = d('ZodStringFormat', (e, t) => {
    (re.init(e, t), As.init(e, t));
  }),
  cf = d('ZodEmail', (e, t) => {
    (Pd.init(e, t), ae.init(e, t));
  }),
  Ua = d('ZodGUID', (e, t) => {
    ($d.init(e, t), ae.init(e, t));
  }),
  ft = d('ZodUUID', (e, t) => {
    (Od.init(e, t), ae.init(e, t));
  }),
  lf = d('ZodURL', (e, t) => {
    (Rd.init(e, t), ae.init(e, t));
  }),
  uf = d('ZodEmoji', (e, t) => {
    (Nd.init(e, t), ae.init(e, t));
  }),
  _f = d('ZodNanoID', (e, t) => {
    (Dd.init(e, t), ae.init(e, t));
  }),
  df = d('ZodCUID', (e, t) => {
    (Zd.init(e, t), ae.init(e, t));
  }),
  pf = d('ZodCUID2', (e, t) => {
    (xd.init(e, t), ae.init(e, t));
  }),
  mf = d('ZodULID', (e, t) => {
    (Ld.init(e, t), ae.init(e, t));
  }),
  ff = d('ZodXID', (e, t) => {
    (Md.init(e, t), ae.init(e, t));
  }),
  gf = d('ZodKSUID', (e, t) => {
    (Vd.init(e, t), ae.init(e, t));
  }),
  hf = d('ZodIPv4', (e, t) => {
    (Gd.init(e, t), ae.init(e, t));
  }),
  vf = d('ZodIPv6', (e, t) => {
    (Bd.init(e, t), ae.init(e, t));
  }),
  yf = d('ZodCIDRv4', (e, t) => {
    (Hd.init(e, t), ae.init(e, t));
  }),
  bf = d('ZodCIDRv6', (e, t) => {
    (Jd.init(e, t), ae.init(e, t));
  }),
  wf = d('ZodBase64', (e, t) => {
    (Wd.init(e, t), ae.init(e, t));
  }),
  If = d('ZodBase64URL', (e, t) => {
    (Xd.init(e, t), ae.init(e, t));
  }),
  Ef = d('ZodE164', (e, t) => {
    (Yd.init(e, t), ae.init(e, t));
  }),
  Sf = d('ZodJWT', (e, t) => {
    (ep.init(e, t), ae.init(e, t));
  }),
  Ts = d('ZodNumber', (e, t) => {
    (vs.init(e, t),
      ue.init(e, t),
      (e._zod.processJSONSchema = (n, o, a) => ym(e, n, o)),
      (e.gt = (n, o) => e.check(Fa(n, o))),
      (e.gte = (n, o) => e.check(Rt(n, o))),
      (e.min = (n, o) => e.check(Rt(n, o))),
      (e.lt = (n, o) => e.check(ja(n, o))),
      (e.lte = (n, o) => e.check(Pt(n, o))),
      (e.max = (n, o) => e.check(Pt(n, o))),
      (e.int = (n) => e.check(Ga(n))),
      (e.safe = (n) => e.check(Ga(n))),
      (e.positive = (n) => e.check(Fa(0, n))),
      (e.nonnegative = (n) => e.check(Rt(0, n))),
      (e.negative = (n) => e.check(ja(0, n))),
      (e.nonpositive = (n) => e.check(Pt(0, n))),
      (e.multipleOf = (n, o) => e.check(Ka(n, o))),
      (e.step = (n, o) => e.check(Ka(n, o))),
      (e.finite = () => e));
    const r = e._zod.bag;
    ((e.minValue =
      Math.max(
        r.minimum ?? Number.NEGATIVE_INFINITY,
        r.exclusiveMinimum ?? Number.NEGATIVE_INFINITY
      ) ?? null),
      (e.maxValue =
        Math.min(
          r.maximum ?? Number.POSITIVE_INFINITY,
          r.exclusiveMaximum ?? Number.POSITIVE_INFINITY
        ) ?? null),
      (e.isInt = (r.format ?? '').includes('int') || Number.isSafeInteger(r.multipleOf ?? 0.5)),
      (e.isFinite = !0),
      (e.format = r.format ?? null));
  });
function xt(e) {
  return qp(Ts, e);
}
const Cf = d('ZodNumberFormat', (e, t) => {
  (tp.init(e, t), Ts.init(e, t));
});
function Ga(e) {
  return Xp(Cf, e);
}
const kf = d('ZodBoolean', (e, t) => {
  (rp.init(e, t), ue.init(e, t), (e._zod.processJSONSchema = (r, n, o) => bm(e, r, n)));
});
function zs(e) {
  return Yp(kf, e);
}
const Af = d('ZodUnknown', (e, t) => {
  (np.init(e, t), ue.init(e, t), (e._zod.processJSONSchema = (r, n, o) => Im()));
});
function Ba() {
  return Qp(Af);
}
const Tf = d('ZodNever', (e, t) => {
  (op.init(e, t), ue.init(e, t), (e._zod.processJSONSchema = (r, n, o) => wm(e, r, n)));
});
function zf(e) {
  return em(Tf, e);
}
const $f = d('ZodArray', (e, t) => {
  (ap.init(e, t),
    ue.init(e, t),
    (e._zod.processJSONSchema = (r, n, o) => Am(e, r, n, o)),
    (e.element = t.element),
    (e.min = (r, n) => e.check(yt(r, n))),
    (e.nonempty = (r) => e.check(yt(1, r))),
    (e.max = (r, n) => e.check(Is(r, n))),
    (e.length = (r, n) => e.check(Es(r, n))),
    (e.unwrap = () => e.element));
});
function $s(e, t) {
  return dm($f, e, t);
}
const Of = d('ZodObject', (e, t) => {
  (ip.init(e, t),
    ue.init(e, t),
    (e._zod.processJSONSchema = (r, n, o) => Tm(e, r, n, o)),
    J(e, 'shape', () => t.shape),
    (e.keyof = () => we(Object.keys(e._zod.def.shape))),
    (e.catchall = (r) => e.clone({ ...e._zod.def, catchall: r })),
    (e.passthrough = () => e.clone({ ...e._zod.def, catchall: Ba() })),
    (e.loose = () => e.clone({ ...e._zod.def, catchall: Ba() })),
    (e.strict = () => e.clone({ ...e._zod.def, catchall: zf() })),
    (e.strip = () => e.clone({ ...e._zod.def, catchall: void 0 })),
    (e.extend = (r) => A_(e, r)),
    (e.safeExtend = (r) => T_(e, r)),
    (e.merge = (r) => z_(e, r)),
    (e.pick = (r) => C_(e, r)),
    (e.omit = (r) => k_(e, r)),
    (e.partial = (...r) => $_(Ps, e, r[0])),
    (e.required = (...r) => O_(Rs, e, r[0])));
});
function st(e, t) {
  const r = { type: 'object', shape: e ?? {}, ...T(t) };
  return new Of(r);
}
const Os = d('ZodUnion', (e, t) => {
  (ws.init(e, t),
    ue.init(e, t),
    (e._zod.processJSONSchema = (r, n, o) => zm(e, r, n, o)),
    (e.options = t.options));
});
function Pf(e, t) {
  return new Os({ type: 'union', options: e, ...T(t) });
}
const Rf = d('ZodDiscriminatedUnion', (e, t) => {
  (Os.init(e, t), cp.init(e, t));
});
function Nf(e, t, r) {
  return new Rf({ type: 'union', options: t, discriminator: e, ...T(r) });
}
const Df = d('ZodIntersection', (e, t) => {
  (lp.init(e, t), ue.init(e, t), (e._zod.processJSONSchema = (r, n, o) => $m(e, r, n, o)));
});
function Zf(e, t) {
  return new Df({ type: 'intersection', left: e, right: t });
}
const Lt = d('ZodEnum', (e, t) => {
  (up.init(e, t),
    ue.init(e, t),
    (e._zod.processJSONSchema = (n, o, a) => Em(e, n, o)),
    (e.enum = t.entries),
    (e.options = Object.values(t.entries)));
  const r = new Set(Object.keys(t.entries));
  ((e.extract = (n, o) => {
    const a = {};
    for (const s of n)
      if (r.has(s)) a[s] = t.entries[s];
      else throw new Error(`Key ${s} not found in enum`);
    return new Lt({ ...t, checks: [], ...T(o), entries: a });
  }),
    (e.exclude = (n, o) => {
      const a = { ...t.entries };
      for (const s of n)
        if (r.has(s)) delete a[s];
        else throw new Error(`Key ${s} not found in enum`);
      return new Lt({ ...t, checks: [], ...T(o), entries: a });
    }));
});
function we(e, t) {
  const r = Array.isArray(e) ? Object.fromEntries(e.map((n) => [n, n])) : e;
  return new Lt({ type: 'enum', entries: r, ...T(t) });
}
const xf = d('ZodLiteral', (e, t) => {
  (_p.init(e, t),
    ue.init(e, t),
    (e._zod.processJSONSchema = (r, n, o) => Sm(e, r, n)),
    (e.values = new Set(t.values)),
    Object.defineProperty(e, 'value', {
      get() {
        if (t.values.length > 1)
          throw new Error(
            'This schema contains multiple valid literal values. Use `.values` instead.'
          );
        return t.values[0];
      }
    }));
});
function it(e, t) {
  return new xf({ type: 'literal', values: Array.isArray(e) ? e : [e], ...T(t) });
}
const Lf = d('ZodTransform', (e, t) => {
  (dp.init(e, t),
    ue.init(e, t),
    (e._zod.processJSONSchema = (r, n, o) => km(e, r)),
    (e._zod.parse = (r, n) => {
      if (n.direction === 'backward') throw new rs(e.constructor.name);
      r.addIssue = (a) => {
        if (typeof a == 'string') r.issues.push(at(a, r.value, t));
        else {
          const s = a;
          (s.fatal && (s.continue = !1),
            s.code ?? (s.code = 'custom'),
            s.input ?? (s.input = r.value),
            s.inst ?? (s.inst = e),
            r.issues.push(at(s)));
        }
      };
      const o = t.transform(r.value, r);
      return o instanceof Promise ? o.then((a) => ((r.value = a), r)) : ((r.value = o), r);
    }));
});
function Mf(e) {
  return new Lf({ type: 'transform', transform: e });
}
const Ps = d('ZodOptional', (e, t) => {
  (pp.init(e, t),
    ue.init(e, t),
    (e._zod.processJSONSchema = (r, n, o) => Lm(e, r, n, o)),
    (e.unwrap = () => e._zod.def.innerType));
});
function Ha(e) {
  return new Ps({ type: 'optional', innerType: e });
}
const Vf = d('ZodNullable', (e, t) => {
  (mp.init(e, t),
    ue.init(e, t),
    (e._zod.processJSONSchema = (r, n, o) => Om(e, r, n, o)),
    (e.unwrap = () => e._zod.def.innerType));
});
function Ja(e) {
  return new Vf({ type: 'nullable', innerType: e });
}
const jf = d('ZodDefault', (e, t) => {
  (fp.init(e, t),
    ue.init(e, t),
    (e._zod.processJSONSchema = (r, n, o) => Rm(e, r, n, o)),
    (e.unwrap = () => e._zod.def.innerType),
    (e.removeDefault = e.unwrap));
});
function Ff(e, t) {
  return new jf({
    type: 'default',
    innerType: e,
    get defaultValue() {
      return typeof t == 'function' ? t() : ss(t);
    }
  });
}
const Kf = d('ZodPrefault', (e, t) => {
  (gp.init(e, t),
    ue.init(e, t),
    (e._zod.processJSONSchema = (r, n, o) => Nm(e, r, n, o)),
    (e.unwrap = () => e._zod.def.innerType));
});
function Uf(e, t) {
  return new Kf({
    type: 'prefault',
    innerType: e,
    get defaultValue() {
      return typeof t == 'function' ? t() : ss(t);
    }
  });
}
const Rs = d('ZodNonOptional', (e, t) => {
  (hp.init(e, t),
    ue.init(e, t),
    (e._zod.processJSONSchema = (r, n, o) => Pm(e, r, n, o)),
    (e.unwrap = () => e._zod.def.innerType));
});
function Gf(e, t) {
  return new Rs({ type: 'nonoptional', innerType: e, ...T(t) });
}
const Bf = d('ZodCatch', (e, t) => {
  (vp.init(e, t),
    ue.init(e, t),
    (e._zod.processJSONSchema = (r, n, o) => Dm(e, r, n, o)),
    (e.unwrap = () => e._zod.def.innerType),
    (e.removeCatch = e.unwrap));
});
function Hf(e, t) {
  return new Bf({ type: 'catch', innerType: e, catchValue: typeof t == 'function' ? t : () => t });
}
const Jf = d('ZodPipe', (e, t) => {
  (yp.init(e, t),
    ue.init(e, t),
    (e._zod.processJSONSchema = (r, n, o) => Zm(e, r, n, o)),
    (e.in = t.in),
    (e.out = t.out));
});
function Wa(e, t) {
  return new Jf({ type: 'pipe', in: e, out: t });
}
const Wf = d('ZodReadonly', (e, t) => {
  (bp.init(e, t),
    ue.init(e, t),
    (e._zod.processJSONSchema = (r, n, o) => xm(e, r, n, o)),
    (e.unwrap = () => e._zod.def.innerType));
});
function qf(e) {
  return new Wf({ type: 'readonly', innerType: e });
}
const Xf = d('ZodCustom', (e, t) => {
  (wp.init(e, t), ue.init(e, t), (e._zod.processJSONSchema = (r, n, o) => Cm(e, r)));
});
function Yf(e, t = {}) {
  return pm(Xf, e, t);
}
function Qf(e) {
  return mm(e);
}
const eg = we(['AC', 'DC', 'TRIX_EXPRESS']),
  tg = we(['H0', 'H0m', 'H0e', 'N', 'TT', 'Z', 'G', 'Scale1', 'Scale0', 'Scale00']),
  rg = we([
    'LOCOMOTIVES',
    'TRAIN_SETS',
    'STARTER_SETS',
    'FREIGHT_CARS',
    'PASSENGER_CARS',
    'ELECTRIC_MULTIPLE_UNITS',
    'RAILCARS'
  ]),
  ng = we(['ANNOUNCED', 'AVAILABLE', 'CANCELLED', 'DISCONTINUED']),
  Ea = we(['DCC_READY', 'DCC_FITTED', 'DCC_SOUND', 'NO_DCC']),
  og = we(['STEAM_LOCOMOTIVE', 'DIESEL_LOCOMOTIVE', 'ELECTRIC_LOCOMOTIVE']),
  ag = we([
    'BAGGAGE_CAR',
    'BUFFET_CAR',
    'COMBINE_CAR',
    'COMPARTMENT_COACH',
    'DINING_CAR',
    'DOUBLE_DECKER',
    'DOME_CAR',
    'DRIVING_TRAILER',
    'LOUNGE',
    'OBSERVATION',
    'OPEN_COACH',
    'RAILWAY_POST_OFFICE',
    'SLEEPING_CAR',
    'SLEEPERETTE'
  ]),
  sg = we([
    'AUTO_TRANSPORT_CARS',
    'BRAKE_WAGON',
    'CONTAINER_CARS',
    'COVERED_FREIGHT_CARS',
    'DEEP_WELL_FLAT_CARS',
    'DUMP_CARS',
    'GONDOLA',
    'HEAVY_GOODS_WAGONS',
    'HINGED_COVER_WAGONS',
    'HOPPER_WAGON',
    'REFRIGERATOR_CARS',
    'SILO_CONTAINER_CARS',
    'SLIDE_TARPAULIN_WAGON',
    'SLIDING_WALL_BOXCARS',
    'SPECIAL_TRANSPORT',
    'STAKE_WAGONS',
    'SWING_ROOF_WAGON',
    'TANK_CARS',
    'TELESCOPE_HOOD_WAGONS'
  ]),
  ig = we(['FIRST', 'SECOND', 'THIRD', 'FIRST_SECOND', 'SECOND_THIRD', 'FIRST_SECOND_THIRD']),
  Sa = we([
    'NEM_651',
    'NEM_652',
    'NEM_654',
    'PLUX_8',
    'PLUX_12',
    'PLUX_16',
    'PLUX_22',
    'NEXT_18',
    'NEXT_18_S',
    'MTC_21'
  ]),
  cg = we([
    'DRIVING_CAR',
    'HIGH_SPEED_TRAIN',
    'MOTOR_CAR',
    'POWER_CAR',
    'TRAILER_CAR',
    'TRAIN_SET'
  ]),
  lg = we(['NONE', 'NEM_355', 'NEM_356', 'NEM_357', 'NEM_359', 'NEM_360', 'NEM_362', 'NEM_365']),
  Ue = we(['YES', 'NO', 'NOT_APPLICABLE']),
  qa = we(['PLASTIC', 'METAL_DIE_CAST']),
  ug = st({ socket: lg, close_couplers: Ue.nullable(), digital_shunting: Ue.nullable() }),
  _g = st({
    minimum_radius: xt().positive().nullable(),
    coupling: ug.nullable(),
    flywheel_fitted: Ue.nullable(),
    body_shell: qa.nullable(),
    chassis: qa.nullable(),
    interior_lights: Ue.nullable(),
    lights: Ue.nullable(),
    sprung_buffers: Ue.nullable()
  }),
  dg = st({ millimeters: xt().positive().nullable(), inches: xt().positive().nullable() }),
  ct = st({
    railway_company_id: G().min(1, 'Railway company is required'),
    livery: G().nullable(),
    length_over_buffers: dg.nullable(),
    technical_specifications: _g.nullable()
  }),
  pg = ct.extend({
    category: it('Locomotive'),
    class_name: G().min(1, 'Class name is required for locomotives'),
    road_number: G().min(1, 'Road number is required for locomotives'),
    series: G().nullable(),
    depot: G().nullable(),
    locomotive_type: og,
    is_dummy: zs().default(!1).nullable(),
    control: Ea.nullable(),
    dcc_interface: Sa.nullable()
  }),
  mg = ct.extend({
    category: it('PassengerCar'),
    type_name: G().min(1, 'Type name is required for passenger cars'),
    road_number: G().nullable(),
    series: G().nullable(),
    depot: G().nullable(),
    passenger_car_type: ag,
    service_level: ig.nullable()
  }),
  fg = ct.extend({
    category: it('FreightCar'),
    type_name: G().min(1, 'Type name is required for freight cars'),
    road_number: G().nullable(),
    series: G().nullable(),
    depot: G().nullable(),
    freight_car_type: sg.nullable()
  }),
  gg = ct.extend({
    category: it('Railcar'),
    type_name: G().min(1, 'Type name is required for railcars'),
    road_number: G().nullable(),
    series: G().nullable(),
    depot: G().nullable(),
    control: Ea.nullable(),
    dcc_interface: Sa.nullable()
  }),
  hg = ct.extend({
    category: it('ElectricMultipleUnit'),
    type_name: G().min(1, 'Type name is required for EMUs'),
    road_number: G().nullable(),
    series: G().nullable(),
    depot: G().nullable(),
    electric_multiple_unit_type: cg,
    is_dummy: zs().default(!1).nullable(),
    control: Ea.nullable(),
    dcc_interface: Sa.nullable()
  }),
  vg = Nf('category', [pg, mg, fg, gg, hg]),
  yg = G()
    .regex(/^\d{4}(\/\d{2}|\/Q[1-4])?$/, {
      message: 'Invalid format. Use: 2025, 2025/06, or 2025/Q2'
    })
    .nullable(),
  bg = st({
    manufacturer_id: G().min(1, 'Manufacturer is required'),
    product_code: G().min(1, 'Product code is required'),
    description: G().min(1, 'Description is required'),
    details: G().nullable(),
    power_method: eg,
    scale: tg,
    epoch: G().min(1, 'Epoch is required'),
    category: rg,
    delivery_date: yg,
    availability_status: ng.nullable(),
    rolling_stocks: $s(vg).min(1, 'At least one rolling stock is required')
  }),
  te = {
    title: { id: 'add-new-railway-model', labelKey: 'form_new_model_title' },
    basicInfo: { id: 'basic-information', labelKey: 'form_new_model_basic_info' },
    manufacturer: { id: 'manufacturer', labelKey: 'form_new_model_manufacturer' },
    productCode: { id: 'product-code', labelKey: 'form_new_model_product_code' },
    productCodePlaceholder: {
      id: 'product-code-placeholder',
      labelKey: 'form_new_model_product_code_placeholder'
    },
    description: { id: 'description', labelKey: 'form_new_model_description' },
    descriptionPlaceholder: {
      id: 'description-placeholder',
      labelKey: 'form_new_model_description_placeholder'
    },
    category: { id: 'category', labelKey: 'form_new_model_category' },
    scale: { id: 'scale', labelKey: 'form_new_model_scale' },
    powerMethod: { id: 'power-method', labelKey: 'form_new_model_power_method' },
    epoch: { id: 'epoch', labelKey: 'form_new_model_epoch' },
    selectPlaceholder: { id: 'select-placeholder', labelKey: 'form_new_model_select_placeholder' },
    deliveryAvailability: {
      id: 'delivery-availability',
      labelKey: 'form_new_model_delivery_availability'
    },
    deliveryDate: { id: 'delivery-date', labelKey: 'form_new_model_delivery_date' },
    deliveryDatePlaceholder: {
      id: 'delivery-date-placeholder',
      labelKey: 'form_new_model_delivery_date_placeholder'
    },
    availabilityStatus: {
      id: 'availability-status',
      labelKey: 'form_new_model_availability_status'
    },
    additionalDetails: { id: 'additional-details', labelKey: 'form_new_model_additional_details' },
    detailsPlaceholder: {
      id: 'details-placeholder',
      labelKey: 'form_new_model_details_placeholder'
    },
    rollingStock: { id: 'rolling-stock', labelKey: 'form_new_model_rolling_stock' },
    railwayCompany: { id: 'railway-company', labelKey: 'form_new_model_railway_company' },
    rollingStockCategory: {
      id: 'rolling-stock-category',
      labelKey: 'form_new_model_rolling_stock_category'
    },
    livery: { id: 'livery', labelKey: 'form_new_model_livery' },
    liveryPlaceholder: { id: 'livery-placeholder', labelKey: 'form_new_model_livery_placeholder' },
    className: { id: 'class-name', labelKey: 'form_new_model_class_name' },
    roadNumber: { id: 'road-number', labelKey: 'form_new_model_road_number' },
    series: { id: 'series', labelKey: 'form_new_model_series' },
    depot: { id: 'depot', labelKey: 'form_new_model_depot' },
    type: { id: 'type', labelKey: 'form_new_model_type' },
    typeName: { id: 'type-name', labelKey: 'form_new_model_type_name' },
    passengerCarType: { id: 'passenger-car-type', labelKey: 'form_new_model_passenger_car_type' },
    freightCarType: { id: 'freight-car-type', labelKey: 'form_new_model_freight_car_type' },
    emuType: { id: 'emu-type', labelKey: 'form_new_model_emu_type' },
    isDummy: { id: 'is-dummy', labelKey: 'form_new_model_is_dummy' },
    technicalDetails: { id: 'technical-details', labelKey: 'form_new_model_technical_details' },
    control: { id: 'control', labelKey: 'form_new_model_control' },
    dccInterface: { id: 'dcc-interface', labelKey: 'form_new_model_dcc_interface' },
    serviceLevel: { id: 'service-level', labelKey: 'form_new_model_service_level' },
    duplicate: { id: 'duplicate', labelKey: 'form_new_model_duplicate' },
    delete: { id: 'delete', labelKey: 'form_new_model_delete' },
    addRollingStock: { id: 'add-rolling-stock', labelKey: 'form_new_model_add_rolling_stock' },
    create: { id: 'create-railway-model', labelKey: 'form_new_model_create' },
    cancel: { id: 'cancel', labelKey: 'form_new_model_cancel' }
  };
function wg() {
  return {
    category: '',
    railway_company_id: '',
    class_name: '',
    road_number: '',
    series: null,
    depot: null,
    livery: null,
    locomotive_type: '',
    passenger_car_type: '',
    freight_car_type: '',
    electric_multiple_unit_type: '',
    type_name: '',
    service_level: '',
    is_dummy: !1,
    control: '',
    dcc_interface: '',
    length_over_buffers: null,
    technical_specifications: null
  };
}
function Ig(e) {
  const t = {
    category: e.category,
    railway_company_id: e.railway_company_id,
    livery: e.livery || null,
    length_over_buffers: e.length_over_buffers ?? null,
    technical_specifications: e.technical_specifications ?? null
  };
  switch (e.category) {
    case 'Locomotive':
      return {
        ...t,
        category: 'Locomotive',
        class_name: e.class_name ?? '',
        road_number: e.road_number ?? '',
        series: e.series || null,
        depot: e.depot || null,
        locomotive_type: e.locomotive_type || '',
        is_dummy: e.is_dummy ?? !1,
        control: e.control || null,
        dcc_interface: e.dcc_interface || null
      };
    case 'PassengerCar':
      return {
        ...t,
        category: 'PassengerCar',
        type_name: e.type_name ?? '',
        road_number: e.road_number || null,
        series: e.series || null,
        depot: e.depot || null,
        passenger_car_type: e.passenger_car_type || '',
        service_level: e.service_level || null
      };
    case 'FreightCar':
      return {
        ...t,
        category: 'FreightCar',
        type_name: e.type_name ?? '',
        road_number: e.road_number || null,
        series: e.series || null,
        depot: e.depot || null,
        freight_car_type: e.freight_car_type || null
      };
    case 'Railcar':
      return {
        ...t,
        category: 'Railcar',
        type_name: e.type_name ?? '',
        road_number: e.road_number || null,
        series: e.series || null,
        depot: e.depot || null,
        control: e.control || null,
        dcc_interface: e.dcc_interface || null
      };
    case 'ElectricMultipleUnit':
      return {
        ...t,
        category: 'ElectricMultipleUnit',
        type_name: e.type_name ?? '',
        road_number: e.road_number || null,
        series: e.series || null,
        depot: e.depot || null,
        electric_multiple_unit_type: e.electric_multiple_unit_type || '',
        is_dummy: e.is_dummy ?? !1,
        control: e.control || null,
        dcc_interface: e.dcc_interface || null
      };
    default:
      throw new Error('Invalid rolling stock category');
  }
}
function ee(e) {
  if (e.labelKey) {
    const t = Ku[e.labelKey];
    return typeof t == 'function' ? t() : e.id;
  }
  return e.display ?? e.id;
}
var Eg = b('<span class="text-sm text-error-500"> </span>'),
  Sg = b(
    '<label class="label"><span class="text-sm font-bold tracking-wider text-surface-300 uppercase"> </span> <!> <!></label>'
  );
function H(e, t) {
  ze(t, !1);
  let r = et(t, 'label', 8),
    n = et(t, 'error', 8, void 0),
    o = et(t, 'required', 8, !1),
    a = et(t, 'fieldId', 8, void 0);
  Ds();
  var s = Sg(),
    i = $(s),
    c = $(i);
  O(i);
  var u = v(i, 2);
  Zs(u, t, 'default', {});
  var g = v(u, 2);
  {
    var C = (l) => {
      var m = Eg(),
        k = $(m, !0);
      (O(m), oe(() => ie(k, n())), f(l, m));
    };
    Ie(g, (l) => {
      n() && l(C);
    });
  }
  (O(s),
    oe(
      (l) => {
        (tt(s, 'for', a()), ie(c, `${l ?? ''}${o() ? ' *' : ''}`));
      },
      [() => (Ca(ee), Ca(r()), xs(() => ee(r())))]
    ),
    f(e, s),
    $e());
}
const Cg = JSON.parse(
    '[{"id":"acme","name":"A.C.M.E.","registered_company_name":"Anonima Costruzioni Modellistiche Esatte S.r.l.","status":"ACTIVE","country_code":"IT","website_url":"https://www.acmetreni.com"},{"id":"atm-antonini","name":"ATM Antonini","registered_company_name":"Antonini Modellismo S.r.l.","status":"ACTIVE","country_code":"IT","website_url":"http://www.atmonline.it"},{"id":"accurascale","name":"Accurascale","registered_company_name":"Accurascale Ltd","status":"ACTIVE","country_code":"IE","website_url":"https://www.accurascale.com"},{"id":"aimx-models","name":"AiMX Models","registered_company_name":"AIMX Company Limited","status":"ACTIVE","country_code":"IT","website_url":"https://aimx-hk.com"},{"id":"albert-model","name":"Albert Model","registered_company_name":"Albert Modell Kft.","status":"Active","country_code":"HU","website_url":"https://albertmodell.com"},{"id":"amintiri-feroviare","name":"Amintiri Feroviare","registered_company_name":"Amintiri Feroviare S.R.L.","status":"Active","country_code":"RO","website_url":"https://www.amintiriferoviare.com"},{"id":"arnold","name":"Arnold","registered_company_name":"Hornby Hobbies Ltd","status":"Active","country_code":"GB","website_url":"https://uk.arnoldmodel.com"},{"id":"athearn","name":"Athearn","registered_company_name":"Athearn Trains","status":"ACTIVE","country_code":"US","website_url":"https://www.athearn.com"},{"id":"atlas-model-railroad-co","name":"Atlas Model Railroad Co.","registered_company_name":"Atlas Model Railroad Co. Inc.","status":"ACTIVE","country_code":"US","website_url":"https://shop.atlasrr.com"},{"id":"b-models","name":"B-Models","registered_company_name":"Van Biervliet NV","status":"Active","country_code":"BE","website_url":"https://www.vanbiervliet.com"},{"id":"bachmann-europe","name":"Bachmann Europe","registered_company_name":"Bachmann Europe Plc","status":"ACTIVE","country_code":"GB","website_url":"https://www.bachmann.co.uk"},{"id":"bachmann-industries","name":"Bachmann Industries","registered_company_name":"Bachmann Industries Inc.","status":"ACTIVE","country_code":"US","website_url":"https://www.bachmanntrains.com"},{"id":"bemo","name":"Bemo","registered_company_name":"Bemo Modelleisenbahnen GmbH u. Co. KG","status":"ACTIVE","country_code":"DE","website_url":"https://www.bemo-modellbahn.de"},{"id":"brawa","name":"Brawa","registered_company_name":"Brawa Artur Braun Modellspielwarenfabrik GmbH & Co. KG","status":"ACTIVE","country_code":"DE","website_url":"https://www.brawa.de"},{"id":"broadway-limited-imports","name":"Broadway Limited Imports","registered_company_name":"Broadway Limited Imports LLC","status":"ACTIVE","country_code":"US","website_url":"https://www.broadway-limited.com"},{"id":"dapol","name":"Dapol","registered_company_name":"Dapol Ltd","status":"ACTIVE","country_code":"GB","website_url":"https://www.dapol.co.uk"},{"id":"dekas","name":"Dekas","registered_company_name":"Dekas ApS","status":"Active","country_code":"DK","website_url":"https://dekas.dk"},{"id":"esu","name":"ESU","registered_company_name":"ESU electronic solutions ulm GmbH & Co. KG","status":"ACTIVE","country_code":"DE","website_url":"https://www.esu.eu"},{"id":"electrotren","name":"Electrotren","registered_company_name":"Electrotren S.A.","status":"ACTIVE","country_code":"ES","website_url":"https://www.electrotren.com"},{"id":"exact-trains","name":"Exact Trains","registered_company_name":"Exact-train B.V.","status":"Active","country_code":"NL","website_url":"https://www.exact-train.com"},{"id":"fleischmann","name":"Fleischmann","registered_company_name":"Modelleisenbahn München GmbH","status":"ACTIVE","country_code":"DE","website_url":"https://www.fleischmann.de"},{"id":"heljan","name":"Heljan","registered_company_name":"Heljan A/S","status":"ACTIVE","country_code":"DK","website_url":"https://www.heljan.dk"},{"id":"hornby-railways","name":"Hornby Railways","registered_company_name":"Hornby Hobbies Ltd","status":"ACTIVE","country_code":"GB","website_url":"https://www.hornby.com"},{"id":"intermountain-railway-co","name":"InterMountain Railway Co.","registered_company_name":"InterMountain Railway Company","status":"ACTIVE","country_code":"US","website_url":"https://www.intermountain-railway.com"},{"id":"jaegerndorfer","name":"Jaegerndorfer","registered_company_name":"Jägerndorfer Ges.m.b.H.","status":"Active","country_code":"AT","website_url":"https://www.jaegerndorfer.at"},{"id":"jouef","name":"Jouef","registered_company_name":"Jouef SAS","status":"ACTIVE","country_code":"FR","website_url":"https://www.jouef.com"},{"id":"kadee-quality-products","name":"Kadee Quality Products","registered_company_name":"Kadee Quality Products Co.","status":"ACTIVE","country_code":"US","website_url":"https://www.kadee.com"},{"id":"kato-precision-railroad-models","name":"Kato Precision Railroad Models","registered_company_name":"Seki-Sui Kinzoku Co. Ltd","status":"ACTIVE","country_code":"JP","website_url":"https://www.katomodels.com"},{"id":"lgb","name":"LGB","registered_company_name":"Gebr. Märklin & Cie. GmbH","status":"ACTIVE","country_code":"DE","website_url":"https://www.lgb.de"},{"id":"ls-models","name":"LS Models","registered_company_name":"LS Models Sprl","status":"ACTIVE","country_code":"BE","website_url":"http://www.lsmodels.com"},{"id":"lematec","name":"Lematec","registered_company_name":"Lematec Prestige Models SA","status":"Active","country_code":"CH","website_url":"https://www.lematec.ch"},{"id":"lenz","name":"Lenz","registered_company_name":"Lenz Elektronik GmbH","status":"ACTIVE","country_code":"DE","website_url":"https://www.lenz-elektronik.de"},{"id":"liliput","name":"Liliput","registered_company_name":"Bachmann Europe Plc","status":"Active","country_code":"DE","website_url":"https://www.liliput.de"},{"id":"lionel","name":"Lionel","registered_company_name":"Lionel LLC","status":"ACTIVE","country_code":"US","website_url":"https://www.lionel.com"},{"id":"mth-electric-trains","name":"MTH Electric Trains","registered_company_name":"MTH Electric Trains","status":"ACTIVE","country_code":"US","website_url":"https://www.mthtrains.com"},{"id":"micro-ace","name":"Micro Ace","registered_company_name":"Micro Ace Co. Ltd","status":"ACTIVE","country_code":"JP","website_url":"http://www.microace-arii.co.jp"},{"id":"mrklin","name":"Märklin","registered_company_name":"Gebr. Märklin & Cie. GmbH","status":"ACTIVE","country_code":"DE","website_url":"https://www.maerklin.de"},{"id":"oskar","name":"Os.Kar","registered_company_name":"Os.kar S.r.l.","status":"ACTIVE","country_code":"IT","website_url":"https://www.oskartrains.eu"},{"id":"peco","name":"Peco","registered_company_name":"Pritchard Patent Product Co. Ltd","status":"ACTIVE","country_code":"GB","website_url":"https://peco-uk.com"},{"id":"piko","name":"Piko","registered_company_name":"PIKO Spielwaren GmbH","status":"ACTIVE","country_code":"DE","website_url":"https://www.piko.de"},{"id":"ree-modeles","name":"REE Modeles","registered_company_name":"Rails Europ Express","status":"Active","country_code":"FR","website_url":"https://www.ree-modeles.com"},{"id":"rapido-trains","name":"Rapido Trains","registered_company_name":"Rapido Trains Inc.","status":"ACTIVE","country_code":"CA","website_url":"https://rapidotrains.com"},{"id":"revolution-trains","name":"Revolution Trains","registered_company_name":"Revolution Trains Ltd","status":"ACTIVE","country_code":"GB","website_url":"https://www.revolutiontrains.com"},{"id":"rivarossi","name":"Rivarossi","registered_company_name":"Rivarossi S.p.A.","status":"ACTIVE","country_code":"IT","website_url":"https://www.rivarossi.com"},{"id":"roco","name":"Roco","registered_company_name":"Modelleisenbahn München GmbH","status":"ACTIVE","country_code":"AT","website_url":"https://www.roco.cc"},{"id":"rokuhan","name":"Rokuhan","registered_company_name":"Toytec Co. Ltd","status":"ACTIVE","country_code":"JP","website_url":"https://www.rokuhan.com"},{"id":"scaletrains","name":"ScaleTrains","registered_company_name":"ScaleTrains.com Inc.","status":"ACTIVE","country_code":"US","website_url":"https://www.scaletrains.com"},{"id":"sudexpress","name":"Sudexpress","registered_company_name":"Ginger SAS","status":"Active","country_code":"FR","website_url":"https://www.sudexpress.com"},{"id":"tillig","name":"Tillig","registered_company_name":"TILLIG Modellbahnen GmbH & Co. KG","status":"Active","country_code":"DE","website_url":"https://www.tillig.com"},{"id":"tomix","name":"Tomix","registered_company_name":"Tomytec Co. Ltd","status":"ACTIVE","country_code":"JP","website_url":"https://www.tomytec.co.jp"},{"id":"trix","name":"Trix","registered_company_name":"Gebr. Märklin & Cie. GmbH","status":"ACTIVE","country_code":"DE","website_url":"https://www.trix.de"},{"id":"vitrains","name":"ViTrains","registered_company_name":"ViTrains S.r.l.","status":"ACTIVE","country_code":"IT","website_url":"https://www.vitrains.it"},{"id":"viessmann","name":"Viessmann","registered_company_name":"Viessmann Modelltechnik GmbH","status":"ACTIVE","country_code":"DE","website_url":"https://viessmann-modell.com"},{"id":"walthers","name":"Walthers","registered_company_name":"Wm. K. Walthers Inc.","status":"ACTIVE","country_code":"US","website_url":"https://www.walthers.com"},{"id":"hag","name":"hag","registered_company_name":"HAG Modelleisenbahnen GmbH","status":"Active","country_code":"CH","website_url":"https://www.hag.ch"},{"id":"igra-models","name":"igra models","registered_company_name":"Igra Model s.r.o.","status":"Active","country_code":"CZ","website_url":"https://igramodel.cz"},{"id":"mtb","name":"mtb","registered_company_name":"mtb-model s.r.o.","status":"Active","country_code":"CZ","website_url":"https://mtb-model.com"},{"id":"nme","name":"nme","registered_company_name":"NME Nürnberger Modell-Eisenbahnen GmbH","status":"Active","country_code":"DE","website_url":"https://www.nme-online.de"}]'
  ),
  kg = [
    {
      id: 'bdz',
      name: 'BDZ',
      registered_company_name: 'Balgarski darzhavni zheleznitsi',
      country_code: 'BG',
      status: 'Active',
      operating_since: '1888-01-01',
      operating_until: null
    },
    {
      id: 'br',
      name: 'BR',
      registered_company_name: 'British Railways',
      country_code: 'GB',
      status: 'INACTIVE',
      operating_since: '1948-01-01',
      operating_until: '1997-03-31'
    },
    {
      id: 'cfr',
      name: 'CFR',
      registered_company_name: 'Compania Națională de Căi Ferate CFR SA',
      country_code: 'RO',
      status: 'Active',
      operating_since: '1880-04-01',
      operating_until: null
    },
    {
      id: 'db',
      name: 'DB',
      registered_company_name: 'Deutsche Bahn AG',
      country_code: 'DE',
      status: 'Active',
      operating_since: '1994-01-01',
      operating_until: null
    },
    {
      id: 'dr',
      name: 'DR',
      registered_company_name: 'Deutsche Reichsbahn',
      country_code: 'DD',
      status: 'Merged',
      operating_since: '1949-04-01',
      operating_until: '1993-12-31'
    },
    {
      id: 'drg',
      name: 'DRG',
      registered_company_name: 'Deutsche Reichsbahn-Gesellschaft',
      country_code: 'DE',
      status: 'Merged',
      operating_since: '1920-04-01',
      operating_until: '1945-05-23'
    },
    {
      id: 'dsb',
      name: 'DSB',
      registered_company_name: 'Danske Statsbaner',
      country_code: 'DK',
      status: 'Active',
      operating_since: '1885-01-01',
      operating_until: null
    },
    {
      id: 'fs',
      name: 'FS',
      registered_company_name: 'Ferrovie dello Stato Italiane S.p.A.',
      country_code: 'IT',
      status: 'Active',
      operating_since: '1905-07-01',
      operating_until: null
    },
    {
      id: 'gwr',
      name: 'GWR',
      registered_company_name: 'Great Western Railway',
      country_code: 'GB',
      status: 'Merged',
      operating_since: '1835-08-31',
      operating_until: '1947-12-31'
    },
    {
      id: 'lner',
      name: 'LNER',
      registered_company_name: 'London and North Eastern Railway',
      country_code: 'GB',
      status: 'Merged',
      operating_since: '1923-01-01',
      operating_until: '1947-12-31'
    },
    {
      id: 'mv',
      name: 'MÁV',
      registered_company_name: 'Magyar Államvasutak',
      country_code: 'HU',
      status: 'Active',
      operating_since: '1868-07-01',
      operating_until: null
    },
    {
      id: 'nmbssncb',
      name: 'NMBS/SNCB',
      registered_company_name: 'Nationale Maatschappij der Belgische Spoorwegen',
      country_code: 'BE',
      status: 'Active',
      operating_since: '1926-07-23',
      operating_until: null
    },
    {
      id: 'nsb',
      name: 'NSB',
      registered_company_name: 'Norges Statsbaner',
      country_code: 'NO',
      status: 'Merged',
      operating_since: '1883-03-01',
      operating_until: '2019-03-22'
    },
    {
      id: 'network-rail',
      name: 'Network Rail',
      registered_company_name: 'Network Rail Limited',
      country_code: 'GB',
      status: 'Active',
      operating_since: '2002-10-03',
      operating_until: null
    },
    {
      id: 'pkp',
      name: 'PKP',
      registered_company_name: 'Polskie Koleje Państwowe',
      country_code: 'PL',
      status: 'Active',
      operating_since: '1918-11-01',
      operating_until: null
    },
    {
      id: 'renfe',
      name: 'RENFE',
      registered_company_name: 'Red Nacional de los Ferrocarriles Españoles',
      country_code: 'ES',
      status: 'Active',
      operating_since: '1941-01-24',
      operating_until: null
    },
    {
      id: 'rhb',
      name: 'RhB',
      registered_company_name: 'Rhätische Bahn AG',
      country_code: 'CH',
      status: 'Active',
      operating_since: '1888-02-07',
      operating_until: null
    },
    {
      id: 'sbb-cff-ffs',
      name: 'SBB CFF FFS',
      registered_company_name: 'Schweizerische Bundesbahnen',
      country_code: 'CH',
      status: 'Active',
      operating_since: '1902-01-01',
      operating_until: null
    },
    {
      id: 'sj',
      name: 'SJ',
      registered_company_name: 'Statens Järnvägar',
      country_code: 'SE',
      status: 'Active',
      operating_since: '1856-12-01',
      operating_until: null
    },
    {
      id: 'sncf',
      name: 'SNCF',
      registered_company_name: 'Société Nationale des Chemins de fer Français',
      country_code: 'FR',
      status: 'Active',
      operating_since: '1938-01-01',
      operating_until: null
    },
    {
      id: 'sr',
      name: 'SR',
      registered_company_name: 'Southern Railway',
      country_code: 'GB',
      status: 'Merged',
      operating_since: '1923-01-01',
      operating_until: '1947-12-31'
    },
    {
      id: 'vr',
      name: 'VR',
      registered_company_name: 'VR-Yhtymä Oyj',
      country_code: 'FI',
      status: 'Active',
      operating_since: '1862-03-17',
      operating_until: null
    },
    {
      id: 'vy',
      name: 'Vy',
      registered_company_name: 'Vygruppen AS',
      country_code: 'NO',
      status: 'Active',
      operating_since: '2019-03-22',
      operating_until: null
    },
    {
      id: 'zssk',
      name: 'ZSSK',
      registered_company_name: 'Železničná spoločnosť Slovensko',
      country_code: 'SK',
      status: 'Active',
      operating_since: '2005-01-01',
      operating_until: null
    },
    {
      id: 'bb',
      name: 'ÖBB',
      registered_company_name: 'Österreichische Bundesbahnen',
      country_code: 'AT',
      status: 'Active',
      operating_since: '1923-07-19',
      operating_until: null
    },
    {
      id: 'd',
      name: 'ČD',
      registered_company_name: 'České dráhy',
      country_code: 'CZ',
      status: 'Active',
      operating_since: '1993-01-01',
      operating_until: null
    },
    {
      id: 'sd',
      name: 'ČSD',
      registered_company_name: 'Československé státní dráhy',
      country_code: 'CZ',
      status: 'INACTIVE',
      operating_since: '1918-10-28',
      operating_until: '1992-12-31'
    }
  ],
  Ag = [
    { id: 'ANNOUNCED', labelKey: 'constants_availability_status_announced' },
    { id: 'AVAILABLE', labelKey: 'constants_availability_status_available' },
    { id: 'CANCELLED', labelKey: 'constants_availability_status_cancelled' },
    { id: 'DISCONTINUED', labelKey: 'constants_availability_status_discontinued' }
  ],
  Tg = [
    { id: 'LOCOMOTIVES', labelKey: 'constants_categories_locomotives' },
    { id: 'TRAIN_SETS', labelKey: 'constants_categories_train_sets' },
    { id: 'STARTER_SETS', labelKey: 'constants_categories_starter_sets' },
    { id: 'FREIGHT_CARS', labelKey: 'constants_categories_freight_cars' },
    { id: 'PASSENGER_CARS', labelKey: 'constants_categories_passenger_cars' },
    { id: 'ELECTRIC_MULTIPLE_UNITS', labelKey: 'constants_categories_electric_multiple_units' },
    { id: 'RAILCARS', labelKey: 'constants_categories_railcars' }
  ],
  zg = [
    { id: 'DCC_READY', display: 'DCC Ready' },
    { id: 'DCC_FITTED', display: 'DCC Fitted' },
    { id: 'DCC_SOUND', display: 'DCC Sound' },
    { id: 'NO_DCC', display: 'No DCC' }
  ],
  $g = [
    { id: 'NEM_651', display: 'NEM 651' },
    { id: 'NEM_652', display: 'NEM 652' },
    { id: 'NEM_654', display: 'NEM 654' },
    { id: 'PLUX_8', display: 'PluX 8' },
    { id: 'PLUX_12', display: 'PluX 12' },
    { id: 'PLUX_16', display: 'PluX 16' },
    { id: 'PLUX_22', display: 'PluX 22' },
    { id: 'NEXT_18', display: 'Next 18' },
    { id: 'NEXT_18_S', display: 'Next 18 S' },
    { id: 'MTC_21', display: 'MTC 21' }
  ],
  Og = [
    { id: 'DRIVING_CAR', labelKey: 'constants_electric_multiple_unit_types_driving_car' },
    { id: 'HIGH_SPEED_TRAIN', labelKey: 'constants_electric_multiple_unit_types_high_speed_train' },
    { id: 'MOTOR_CAR', labelKey: 'constants_electric_multiple_unit_types_motor_car' },
    { id: 'POWER_CAR', labelKey: 'constants_electric_multiple_unit_types_power_car' },
    { id: 'TRAILER_CAR', labelKey: 'constants_electric_multiple_unit_types_trailer_car' },
    { id: 'TRAIN_SET', labelKey: 'constants_electric_multiple_unit_types_train_set' }
  ],
  Pg = [
    { id: 'I', display: 'I' },
    { id: 'I/II', display: 'I/II' },
    { id: 'II', display: 'II' },
    { id: 'II/III', display: 'II/III' },
    { id: 'III', display: 'III' },
    { id: 'III/IV', display: 'III/IV' },
    { id: 'IV', display: 'IV' },
    { id: 'IV/V', display: 'IV/V' },
    { id: 'V', display: 'V' },
    { id: 'V/VI', display: 'V/VI' },
    { id: 'VI', display: 'VI' },
    { id: 'Vm', display: 'Vm' }
  ],
  Rg = [
    { id: 'AUTO_TRANSPORT_CARS', labelKey: 'constants_freight_car_types_auto_transport_cars' },
    { id: 'BRAKE_WAGON', labelKey: 'constants_freight_car_types_brake_wagon' },
    { id: 'CONTAINER_CARS', labelKey: 'constants_freight_car_types_container_cars' },
    { id: 'COVERED_FREIGHT_CARS', labelKey: 'constants_freight_car_types_covered_freight_cars' },
    { id: 'DEEP_WELL_FLAT_CARS', labelKey: 'constants_freight_car_types_deep_well_flat_cars' },
    { id: 'DUMP_CARS', labelKey: 'constants_freight_car_types_dump_cars' },
    { id: 'GONDOLA', labelKey: 'constants_freight_car_types_gondola' },
    { id: 'HEAVY_GOODS_WAGONS', labelKey: 'constants_freight_car_types_heavy_goods_wagons' },
    { id: 'HINGED_COVER_WAGONS', labelKey: 'constants_freight_car_types_hinged_cover_wagons' },
    { id: 'HOPPER_WAGON', labelKey: 'constants_freight_car_types_hopper_wagon' },
    { id: 'REFRIGERATOR_CARS', labelKey: 'constants_freight_car_types_refrigerator_cars' },
    { id: 'SILO_CONTAINER_CARS', labelKey: 'constants_freight_car_types_silo_container_cars' },
    { id: 'SLIDE_TARPAULIN_WAGON', labelKey: 'constants_freight_car_types_slide_tarpaulin_wagon' },
    { id: 'SLIDING_WALL_BOXCARS', labelKey: 'constants_freight_car_types_sliding_wall_boxcars' },
    { id: 'SPECIAL_TRANSPORT', labelKey: 'constants_freight_car_types_special_transport' },
    { id: 'STAKE_WAGONS', labelKey: 'constants_freight_car_types_stake_wagons' },
    { id: 'SWING_ROOF_WAGON', labelKey: 'constants_freight_car_types_swing_roof_wagon' },
    { id: 'TANK_CARS', labelKey: 'constants_freight_car_types_tank_cars' },
    { id: 'TELESCOPE_HOOD_WAGONS', labelKey: 'constants_freight_car_types_telescope_hood_wagons' }
  ],
  Ng = [
    { id: 'STEAM_LOCOMOTIVE', labelKey: 'constants_locomotive_types_steam_locomotive' },
    { id: 'DIESEL_LOCOMOTIVE', labelKey: 'constants_locomotive_types_diesel_locomotive' },
    { id: 'ELECTRIC_LOCOMOTIVE', labelKey: 'constants_locomotive_types_electric_locomotive' }
  ],
  Dg = [
    { id: 'BAGGAGE_CAR', labelKey: 'constants_passenger_car_types_baggage_car' },
    { id: 'BUFFET_CAR', labelKey: 'constants_passenger_car_types_buffet_car' },
    { id: 'COMBINE_CAR', labelKey: 'constants_passenger_car_types_combine_car' },
    { id: 'COMPARTMENT_COACH', labelKey: 'constants_passenger_car_types_compartment_coach' },
    { id: 'DINING_CAR', labelKey: 'constants_passenger_car_types_dining_car' },
    { id: 'DOUBLE_DECKER', labelKey: 'constants_passenger_car_types_double_decker' },
    { id: 'DOME_CAR', labelKey: 'constants_passenger_car_types_dome_car' },
    { id: 'DRIVING_TRAILER', labelKey: 'constants_passenger_car_types_driving_trailer' },
    { id: 'LOUNGE', labelKey: 'constants_passenger_car_types_lounge' },
    { id: 'OBSERVATION', labelKey: 'constants_passenger_car_types_observation' },
    { id: 'OPEN_COACH', labelKey: 'constants_passenger_car_types_open_coach' },
    { id: 'RAILWAY_POST_OFFICE', labelKey: 'constants_passenger_car_types_railway_post_office' },
    { id: 'SLEEPING_CAR', labelKey: 'constants_passenger_car_types_sleeping_car' },
    { id: 'SLEEPERETTE', labelKey: 'constants_passenger_car_types_sleeperette' }
  ],
  Zg = [
    { id: 'AC', display: 'AC' },
    { id: 'DC', display: 'DC' },
    { id: 'TRIX_EXPRESS', display: 'TRIX Express' }
  ],
  xg = [
    { id: 'Locomotive', labelKey: 'constants_categories_locomotives' },
    { id: 'PassengerCar', labelKey: 'constants_categories_passenger_cars' },
    { id: 'FreightCar', labelKey: 'constants_categories_freight_cars' },
    { id: 'Railcar', labelKey: 'constants_categories_railcars' },
    { id: 'ElectricMultipleUnit', labelKey: 'constants_categories_electric_multiple_units' }
  ],
  Lg = [
    { id: '1', display: 'I cl' },
    { id: '2', display: 'II cl' },
    { id: '3', display: 'III cl' },
    { id: '1/2', display: 'I cl / II cl' },
    { id: '2/3', display: 'II cl / III cl' },
    { id: '1/2/3', display: 'I cl / II cl / III cl' }
  ];
var Mg = b('<option> </option>'),
  Vg = b('<select class="select border-surface-600 bg-surface-800"><option> </option><!></select>'),
  jg = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  Fg = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  Kg = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  Ug = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  Gg = b('<span class="text-sm font-semibold"> </span> <!>', 1),
  Bg = b('<div class="grid grid-cols-1 gap-4 lg:grid-cols-2"><!></div>'),
  Hg = b('<!> <!>', 1),
  Jg = b('<!> <!> <!> <!> <!> <div class="lg:col-span-2"><!></div>', 1),
  Wg = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  qg = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  Xg = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  Yg = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  Qg = b('<!> <!> <!> <!> <!>', 1),
  eh = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  th = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  rh = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  nh = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  oh = b('<span class="text-sm font-semibold"> </span> <!>', 1),
  ah = b('<div class="grid grid-cols-1 gap-4 lg:grid-cols-2"><!> <!></div>'),
  sh = b('<!> <!>', 1),
  ih = b('<!> <!> <!> <!> <div class="lg:col-span-2"><!></div>', 1),
  ch = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  lh = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  uh = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  _h = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  dh = b('<span class="text-sm font-semibold"> </span> <!>', 1),
  ph = b('<div class="grid grid-cols-1 gap-4 lg:grid-cols-2"><!> <!></div>'),
  mh = b('<!> <!>', 1),
  fh = b(
    '<!> <!> <!> <!> <!> <label class="label flex items-center gap-2"><input class="checkbox" type="checkbox"/> <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"> </span></label> <div class="lg:col-span-2"><!></div>',
    1
  ),
  gh = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  hh = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  vh = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  yh = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  bh = b('<span class="text-sm font-semibold"> </span> <!>', 1),
  wh = b('<div class="grid grid-cols-1 gap-4 lg:grid-cols-2"><!> <!></div>'),
  Ih = b('<!> <!>', 1),
  Eh = b(
    '<!> <!> <!> <!> <!> <label class="label flex items-center gap-2"><input class="checkbox" type="checkbox"/> <span class="text-sm font-bold tracking-wider text-surface-300 uppercase"> </span></label> <div class="lg:col-span-2"><!></div>',
    1
  ),
  Sh = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  Ch = b(
    '<div class="variant-filled-surface card p-4"><div class="mb-4 flex items-center justify-between"><h4 class="h5"> </h4> <div class="flex gap-2"><button type="button" class="btn-icon btn-sm">📋</button> <button type="button" class="btn-icon btn-sm">🗑️</button></div></div> <div class="grid grid-cols-1 gap-4 lg:grid-cols-2"><!> <!> <!> <!></div></div>'
  );
function kh(e, t) {
  ze(t, !0);
  const r = (F, W = he, X = he, K = he, B = he) => {
      {
        let L = y(() => R(X()));
        H(F, {
          get label() {
            return W();
          },
          get error() {
            return _(L);
          },
          get required() {
            return K();
          },
          children: (U, D) => {
            var z = Vg(),
              P = $(z),
              w = $(P, !0);
            (O(P), (P.value = P.__value = ''));
            var I = v(P);
            (Nt(
              I,
              17,
              B,
              (S) => S.id,
              (S, E) => {
                var A = Mg(),
                  x = $(A, !0);
                O(A);
                var h = {};
                (oe(
                  (N) => {
                    (ie(x, N), h !== (h = _(E).id) && (A.value = (A.__value = _(E).id) ?? ''));
                  },
                  [() => ('name' in _(E) ? _(E).name : ee(_(E)))]
                ),
                  f(S, A));
              }
            ),
              O(z),
              oe((S) => ie(w, S), [() => ee(t.formLabels.selectPlaceholder)]),
              Ls(
                z,
                () => l()[X()],
                (S) => (l()[X()] = S)
              ),
              f(U, z));
          },
          $$slots: { default: !0 }
        });
      }
    },
    n = (F) => {
      var W = Jg(),
        X = Z(W);
      {
        let P = y(() => R('type_name'));
        H(X, {
          get label() {
            return t.formLabels.typeName;
          },
          get error() {
            return _(P);
          },
          required: !0,
          children: (w, I) => {
            var S = jg();
            (q(S),
              Q(
                S,
                () => l().type_name,
                (E) => (l().type_name = E)
              ),
              f(w, S));
          },
          $$slots: { default: !0 }
        });
      }
      var K = v(X, 2);
      r(
        K,
        () => t.formLabels.passengerCarType,
        () => 'passenger_car_type',
        () => !0,
        () => t.passengerCarTypesData
      );
      var B = v(K, 2);
      {
        let P = y(() => R('road_number'));
        H(B, {
          get label() {
            return t.formLabels.roadNumber;
          },
          get error() {
            return _(P);
          },
          children: (w, I) => {
            var S = Fg();
            (q(S),
              Q(
                S,
                () => l().road_number,
                (E) => (l().road_number = E)
              ),
              f(w, S));
          },
          $$slots: { default: !0 }
        });
      }
      var L = v(B, 2);
      {
        let P = y(() => R('series'));
        H(L, {
          get label() {
            return t.formLabels.series;
          },
          get error() {
            return _(P);
          },
          children: (w, I) => {
            var S = Kg();
            (q(S),
              Q(
                S,
                () => l().series,
                (E) => (l().series = E)
              ),
              f(w, S));
          },
          $$slots: { default: !0 }
        });
      }
      var U = v(L, 2);
      {
        let P = y(() => R('depot'));
        H(U, {
          get label() {
            return t.formLabels.depot;
          },
          get error() {
            return _(P);
          },
          children: (w, I) => {
            var S = Ug();
            (q(S),
              Q(
                S,
                () => l().depot,
                (E) => (l().depot = E)
              ),
              f(w, S));
          },
          $$slots: { default: !0 }
        });
      }
      var D = v(U, 2),
        z = $(D);
      (Ce(z, {
        collapsible: !0,
        children: (P, w) => {
          {
            let I = y(() => `technical-${t.index}-passenger`);
            c(P, {
              get value() {
                return _(I);
              },
              children: (S, E) => {
                var A = Hg(),
                  x = Z(A);
                u(x, {
                  class: 'flex w-full items-center justify-between px-2 py-1 text-left',
                  children: (N, M) => {
                    var Y = Gg(),
                      ne = Z(Y),
                      de = $(ne, !0);
                    O(ne);
                    var Re = v(ne, 2);
                    (C(Re, { class: 'text-muted text-xs' }),
                      oe((ye) => ie(de, ye), [() => ee(t.formLabels.technicalDetails)]),
                      f(N, Y));
                  },
                  $$slots: { default: !0 }
                });
                var h = v(x, 2);
                (g(h, {
                  class: 'px-2 pt-1 pb-2',
                  children: (N, M) => {
                    var Y = Bg(),
                      ne = $(Y);
                    (r(
                      ne,
                      () => t.formLabels.serviceLevel,
                      () => 'service_level',
                      () => !1,
                      () => t.serviceLevelsData
                    ),
                      O(Y),
                      f(N, Y));
                  },
                  $$slots: { default: !0 }
                }),
                  f(S, A));
              },
              $$slots: { default: !0 }
            });
          }
        },
        $$slots: { default: !0 }
      }),
        O(D),
        f(F, W));
    },
    o = (F) => {
      var W = Qg(),
        X = Z(W);
      {
        let D = y(() => R('type_name'));
        H(X, {
          get label() {
            return t.formLabels.typeName;
          },
          get error() {
            return _(D);
          },
          required: !0,
          children: (z, P) => {
            var w = Wg();
            (q(w),
              Q(
                w,
                () => l().type_name,
                (I) => (l().type_name = I)
              ),
              f(z, w));
          },
          $$slots: { default: !0 }
        });
      }
      var K = v(X, 2);
      r(
        K,
        () => t.formLabels.freightCarType,
        () => 'freight_car_type',
        () => !1,
        () => t.freightCarTypesData
      );
      var B = v(K, 2);
      {
        let D = y(() => R('road_number'));
        H(B, {
          get label() {
            return t.formLabels.roadNumber;
          },
          get error() {
            return _(D);
          },
          children: (z, P) => {
            var w = qg();
            (q(w),
              Q(
                w,
                () => l().road_number,
                (I) => (l().road_number = I)
              ),
              f(z, w));
          },
          $$slots: { default: !0 }
        });
      }
      var L = v(B, 2);
      {
        let D = y(() => R('series'));
        H(L, {
          get label() {
            return t.formLabels.series;
          },
          get error() {
            return _(D);
          },
          children: (z, P) => {
            var w = Xg();
            (q(w),
              Q(
                w,
                () => l().series,
                (I) => (l().series = I)
              ),
              f(z, w));
          },
          $$slots: { default: !0 }
        });
      }
      var U = v(L, 2);
      {
        let D = y(() => R('depot'));
        H(U, {
          get label() {
            return t.formLabels.depot;
          },
          get error() {
            return _(D);
          },
          children: (z, P) => {
            var w = Yg();
            (q(w),
              Q(
                w,
                () => l().depot,
                (I) => (l().depot = I)
              ),
              f(z, w));
          },
          $$slots: { default: !0 }
        });
      }
      f(F, W);
    },
    a = (F) => {
      var W = ih(),
        X = Z(W);
      {
        let z = y(() => R('type_name'));
        H(X, {
          get label() {
            return t.formLabels.typeName;
          },
          get error() {
            return _(z);
          },
          required: !0,
          children: (P, w) => {
            var I = eh();
            (q(I),
              Q(
                I,
                () => l().type_name,
                (S) => (l().type_name = S)
              ),
              f(P, I));
          },
          $$slots: { default: !0 }
        });
      }
      var K = v(X, 2);
      {
        let z = y(() => R('road_number'));
        H(K, {
          get label() {
            return t.formLabels.roadNumber;
          },
          get error() {
            return _(z);
          },
          children: (P, w) => {
            var I = th();
            (q(I),
              Q(
                I,
                () => l().road_number,
                (S) => (l().road_number = S)
              ),
              f(P, I));
          },
          $$slots: { default: !0 }
        });
      }
      var B = v(K, 2);
      {
        let z = y(() => R('series'));
        H(B, {
          get label() {
            return t.formLabels.series;
          },
          get error() {
            return _(z);
          },
          children: (P, w) => {
            var I = rh();
            (q(I),
              Q(
                I,
                () => l().series,
                (S) => (l().series = S)
              ),
              f(P, I));
          },
          $$slots: { default: !0 }
        });
      }
      var L = v(B, 2);
      {
        let z = y(() => R('depot'));
        H(L, {
          get label() {
            return t.formLabels.depot;
          },
          get error() {
            return _(z);
          },
          children: (P, w) => {
            var I = nh();
            (q(I),
              Q(
                I,
                () => l().depot,
                (S) => (l().depot = S)
              ),
              f(P, I));
          },
          $$slots: { default: !0 }
        });
      }
      var U = v(L, 2),
        D = $(U);
      (Ce(D, {
        collapsible: !0,
        children: (z, P) => {
          {
            let w = y(() => `technical-${t.index}-railcar`);
            c(z, {
              get value() {
                return _(w);
              },
              children: (I, S) => {
                var E = sh(),
                  A = Z(E);
                u(A, {
                  class: 'flex w-full items-center justify-between px-2 py-1 text-left',
                  children: (h, N) => {
                    var M = oh(),
                      Y = Z(M),
                      ne = $(Y, !0);
                    O(Y);
                    var de = v(Y, 2);
                    (C(de, { class: 'text-muted text-xs' }),
                      oe((Re) => ie(ne, Re), [() => ee(t.formLabels.technicalDetails)]),
                      f(h, M));
                  },
                  $$slots: { default: !0 }
                });
                var x = v(A, 2);
                (g(x, {
                  class: 'px-2 pt-1 pb-2',
                  children: (h, N) => {
                    var M = ah(),
                      Y = $(M);
                    r(
                      Y,
                      () => t.formLabels.control,
                      () => 'control',
                      () => !1,
                      () => t.controlsData
                    );
                    var ne = v(Y, 2);
                    (r(
                      ne,
                      () => t.formLabels.dccInterface,
                      () => 'dcc_interface',
                      () => !1,
                      () => t.dccInterfacesData
                    ),
                      O(M),
                      f(h, M));
                  },
                  $$slots: { default: !0 }
                }),
                  f(I, E));
              },
              $$slots: { default: !0 }
            });
          }
        },
        $$slots: { default: !0 }
      }),
        O(U),
        f(F, W));
    },
    s = (F) => {
      var W = fh(),
        X = Z(W);
      {
        let E = y(() => R('type_name'));
        H(X, {
          get label() {
            return t.formLabels.typeName;
          },
          get error() {
            return _(E);
          },
          required: !0,
          children: (A, x) => {
            var h = ch();
            (q(h),
              Q(
                h,
                () => l().type_name,
                (N) => (l().type_name = N)
              ),
              f(A, h));
          },
          $$slots: { default: !0 }
        });
      }
      var K = v(X, 2);
      r(
        K,
        () => t.formLabels.emuType,
        () => 'electric_multiple_unit_type',
        () => !0,
        () => t.electricMultipleUnitTypesData
      );
      var B = v(K, 2);
      {
        let E = y(() => R('road_number'));
        H(B, {
          get label() {
            return t.formLabels.roadNumber;
          },
          get error() {
            return _(E);
          },
          children: (A, x) => {
            var h = lh();
            (q(h),
              Q(
                h,
                () => l().road_number,
                (N) => (l().road_number = N)
              ),
              f(A, h));
          },
          $$slots: { default: !0 }
        });
      }
      var L = v(B, 2);
      {
        let E = y(() => R('series'));
        H(L, {
          get label() {
            return t.formLabels.series;
          },
          get error() {
            return _(E);
          },
          children: (A, x) => {
            var h = uh();
            (q(h),
              Q(
                h,
                () => l().series,
                (N) => (l().series = N)
              ),
              f(A, h));
          },
          $$slots: { default: !0 }
        });
      }
      var U = v(L, 2);
      {
        let E = y(() => R('depot'));
        H(U, {
          get label() {
            return t.formLabels.depot;
          },
          get error() {
            return _(E);
          },
          children: (A, x) => {
            var h = _h();
            (q(h),
              Q(
                h,
                () => l().depot,
                (N) => (l().depot = N)
              ),
              f(A, h));
          },
          $$slots: { default: !0 }
        });
      }
      var D = v(U, 2),
        z = $(D);
      q(z);
      var P = v(z, 2),
        w = $(P, !0);
      (O(P), O(D));
      var I = v(D, 2),
        S = $(I);
      (Ce(S, {
        collapsible: !0,
        children: (E, A) => {
          {
            let x = y(() => `technical-${t.index}-emu`);
            c(E, {
              get value() {
                return _(x);
              },
              children: (h, N) => {
                var M = mh(),
                  Y = Z(M);
                u(Y, {
                  class: 'flex w-full items-center justify-between px-2 py-1 text-left',
                  children: (de, Re) => {
                    var ye = dh(),
                      Se = Z(ye),
                      Ne = $(Se, !0);
                    O(Se);
                    var Tt = v(Se, 2);
                    (C(Tt, { class: 'text-muted text-xs' }),
                      oe((zt) => ie(Ne, zt), [() => ee(t.formLabels.technicalDetails)]),
                      f(de, ye));
                  },
                  $$slots: { default: !0 }
                });
                var ne = v(Y, 2);
                (g(ne, {
                  class: 'px-2 pt-1 pb-2',
                  children: (de, Re) => {
                    var ye = ph(),
                      Se = $(ye);
                    r(
                      Se,
                      () => t.formLabels.control,
                      () => 'control',
                      () => !1,
                      () => t.controlsData
                    );
                    var Ne = v(Se, 2);
                    (r(
                      Ne,
                      () => t.formLabels.dccInterface,
                      () => 'dcc_interface',
                      () => !1,
                      () => t.dccInterfacesData
                    ),
                      O(ye),
                      f(de, ye));
                  },
                  $$slots: { default: !0 }
                }),
                  f(h, M));
              },
              $$slots: { default: !0 }
            });
          }
        },
        $$slots: { default: !0 }
      }),
        O(I),
        oe((E) => ie(w, E), [() => ee(t.formLabels.isDummy)]),
        ka(
          z,
          () => l().is_dummy,
          (E) => (l().is_dummy = E)
        ),
        f(F, W));
    },
    i = (F) => {
      var W = Eh(),
        X = Z(W);
      {
        let E = y(() => R('class_name'));
        H(X, {
          get label() {
            return t.formLabels.className;
          },
          get error() {
            return _(E);
          },
          required: !0,
          children: (A, x) => {
            var h = gh();
            (q(h),
              Q(
                h,
                () => l().class_name,
                (N) => (l().class_name = N)
              ),
              f(A, h));
          },
          $$slots: { default: !0 }
        });
      }
      var K = v(X, 2);
      {
        let E = y(() => R('road_number'));
        H(K, {
          get label() {
            return t.formLabels.roadNumber;
          },
          get error() {
            return _(E);
          },
          required: !0,
          children: (A, x) => {
            var h = hh();
            (q(h),
              Q(
                h,
                () => l().road_number,
                (N) => (l().road_number = N)
              ),
              f(A, h));
          },
          $$slots: { default: !0 }
        });
      }
      var B = v(K, 2);
      {
        let E = y(() => R('series'));
        H(B, {
          get label() {
            return t.formLabels.series;
          },
          get error() {
            return _(E);
          },
          children: (A, x) => {
            var h = vh();
            (q(h),
              Q(
                h,
                () => l().series,
                (N) => (l().series = N)
              ),
              f(A, h));
          },
          $$slots: { default: !0 }
        });
      }
      var L = v(B, 2);
      {
        let E = y(() => R('depot'));
        H(L, {
          get label() {
            return t.formLabels.depot;
          },
          get error() {
            return _(E);
          },
          children: (A, x) => {
            var h = yh();
            (q(h),
              Q(
                h,
                () => l().depot,
                (N) => (l().depot = N)
              ),
              f(A, h));
          },
          $$slots: { default: !0 }
        });
      }
      var U = v(L, 2);
      r(
        U,
        () => t.formLabels.type,
        () => 'locomotive_type',
        () => !0,
        () => t.locomotiveTypesData
      );
      var D = v(U, 2),
        z = $(D);
      q(z);
      var P = v(z, 2),
        w = $(P, !0);
      (O(P), O(D));
      var I = v(D, 2),
        S = $(I);
      (Ce(S, {
        collapsible: !0,
        children: (E, A) => {
          {
            let x = y(() => `technical-${t.index}-locomotive`);
            c(E, {
              get value() {
                return _(x);
              },
              children: (h, N) => {
                var M = Ih(),
                  Y = Z(M);
                u(Y, {
                  class: 'flex w-full items-center justify-between px-2 py-1 text-left',
                  children: (de, Re) => {
                    var ye = bh(),
                      Se = Z(ye),
                      Ne = $(Se, !0);
                    O(Se);
                    var Tt = v(Se, 2);
                    (C(Tt, { class: 'text-muted text-xs' }),
                      oe((zt) => ie(Ne, zt), [() => ee(t.formLabels.technicalDetails)]),
                      f(de, ye));
                  },
                  $$slots: { default: !0 }
                });
                var ne = v(Y, 2);
                (g(ne, {
                  class: 'px-2 pt-1 pb-2',
                  children: (de, Re) => {
                    var ye = wh(),
                      Se = $(ye);
                    r(
                      Se,
                      () => t.formLabels.control,
                      () => 'control',
                      () => !1,
                      () => t.controlsData
                    );
                    var Ne = v(Se, 2);
                    (r(
                      Ne,
                      () => t.formLabels.dccInterface,
                      () => 'dcc_interface',
                      () => !1,
                      () => t.dccInterfacesData
                    ),
                      O(ye),
                      f(de, ye));
                  },
                  $$slots: { default: !0 }
                }),
                  f(h, M));
              },
              $$slots: { default: !0 }
            });
          }
        },
        $$slots: { default: !0 }
      }),
        O(I),
        oe((E) => ie(w, E), [() => ee(t.formLabels.isDummy)]),
        ka(
          z,
          () => l().is_dummy,
          (E) => (l().is_dummy = E)
        ),
        f(F, W));
    },
    c = Ce.Item,
    u = Ce.ItemTrigger,
    g = Ce.ItemContent,
    C = Ce.ItemIndicator,
    l = et(t, 'rs', 7),
    m = y(() => l().category === 'Locomotive'),
    k = y(() => l().category === 'PassengerCar'),
    j = y(() => l().category === 'FreightCar'),
    pe = y(() => l().category === 'Railcar'),
    se = y(() => l().category === 'ElectricMultipleUnit');
  function R(F) {
    return t.errors[`rolling_stocks.${t.index}.${F}`];
  }
  var le = Ch(),
    ge = $(le),
    Ae = $(ge),
    At = $(Ae);
  O(Ae);
  var Ve = v(Ae, 2),
    Ye = $(Ve);
  Ye.__click = () => t.onDuplicate(t.index);
  var lt = v(Ye, 2);
  ((lt.__click = () => t.onDelete(t.index)), O(Ve), O(ge));
  var Pe = v(ge, 2),
    ut = $(Pe);
  r(
    ut,
    () => t.formLabels.railwayCompany,
    () => 'railway_company_id',
    () => !0,
    () => t.railwayCompaniesData
  );
  var je = v(ut, 2);
  r(
    je,
    () => t.formLabels.rollingStockCategory,
    () => 'category',
    () => !0,
    () => t.rollingStockCategoriesData
  );
  var _t = v(je, 2);
  {
    let F = y(() => R('livery'));
    H(_t, {
      get label() {
        return t.formLabels.livery;
      },
      get error() {
        return _(F);
      },
      children: (W, X) => {
        var K = Sh();
        (q(K),
          Q(
            K,
            () => l().livery,
            (B) => (l().livery = B)
          ),
          f(W, K));
      },
      $$slots: { default: !0 }
    });
  }
  var V = v(_t, 2);
  {
    var ve = (F) => {
        i(F);
      },
      _e = (F) => {
        var W = me(),
          X = Z(W);
        {
          var K = (L) => {
              n(L);
            },
            B = (L) => {
              var U = me(),
                D = Z(U);
              {
                var z = (w) => {
                    o(w);
                  },
                  P = (w) => {
                    var I = me(),
                      S = Z(I);
                    {
                      var E = (x) => {
                          a(x);
                        },
                        A = (x) => {
                          var h = me(),
                            N = Z(h);
                          {
                            var M = (Y) => {
                              s(Y);
                            };
                            Ie(
                              N,
                              (Y) => {
                                _(se) && Y(M);
                              },
                              !0
                            );
                          }
                          f(x, h);
                        };
                      Ie(
                        S,
                        (x) => {
                          _(pe) ? x(E) : x(A, !1);
                        },
                        !0
                      );
                    }
                    f(w, I);
                  };
                Ie(
                  D,
                  (w) => {
                    _(j) ? w(z) : w(P, !1);
                  },
                  !0
                );
              }
              f(L, U);
            };
          Ie(
            X,
            (L) => {
              _(k) ? L(K) : L(B, !1);
            },
            !0
          );
        }
        f(F, W);
      };
    Ie(V, (F) => {
      _(m) ? F(ve) : F(_e, !1);
    });
  }
  (O(Pe),
    O(le),
    oe((F) => ie(At, `${F ?? ''} #${t.index + 1}`), [() => ee(t.formLabels.rollingStock)]),
    f(e, le),
    $e());
}
Ya(['click']);
var Ah = b('<option> </option>'),
  Th = b('<select class="select border-surface-600 bg-surface-800"><option> </option><!></select>'),
  zh = b('<div class="variant-filled-error mb-4 card p-4"> </div>'),
  $h = b('<h3 class="mb-0 h4"> </h3> <!>', 1),
  Oh = b('<input class="input border-surface-600 bg-surface-800 font-mono" type="text"/>'),
  Ph = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  Rh = b('<div class="grid grid-cols-1 gap-4 lg:grid-cols-2"><!> <!> <!> <!> <!> <!> <!></div>'),
  Nh = b('<!> <!>', 1),
  Dh = b('<h3 class="mb-0 h4"> </h3> <!>', 1),
  Zh = b('<input class="input border-surface-600 bg-surface-800" type="text"/>'),
  xh = b('<textarea class="textarea border-surface-600 bg-surface-800" rows="3"></textarea>'),
  Lh = b('<div class="grid grid-cols-1 gap-4 lg:grid-cols-2"><!> <!> <!></div>'),
  Mh = b('<!> <!>', 1),
  Vh = b('<h3 class="mb-0 h4"> <span class="variant-soft-primary badge"> </span></h3> <!>', 1),
  jh = b('<div class="text-muted text-sm">Add at least one rolling stock item to continue.</div>'),
  Fh = b(
    '<div class="space-y-4"><!> <!> <button type="button" class="cta-btn cta-primary btn svelte-kffwxd"> </button></div>'
  ),
  Kh = b('<!> <!>', 1),
  Uh = b('<!> <!> <!>', 1),
  Gh = b(
    '<div class="container mx-auto p-8"><h1 class="mb-8 h2"> </h1> <!> <form><!> <div class="mt-8 flex gap-4"><button type="submit" class="cta-btn cta-primary btn svelte-kffwxd"> </button> <button type="button" class="cta-btn cta-secondary btn svelte-kffwxd"> </button></div></form></div>'
  );
function Bh(e, t) {
  ze(t, !0);
  const r = (V, ve = he, _e = he, F = he, W = he, X = he, K = he) => {
      const B = y(W);
      H(V, {
        get label() {
          return ve();
        },
        get error() {
          return _e();
        },
        get required() {
          return F();
        },
        children: (L, U) => {
          var D = Th();
          D.__change = (S) => K()(S.currentTarget.value);
          var z = $(D),
            P = $(z, !0);
          (O(z), (z.value = z.__value = ''));
          var w = v(z);
          (Nt(
            w,
            17,
            X,
            (S) => S.id,
            (S, E) => {
              var A = Ah(),
                x = $(A, !0);
              O(A);
              var h = {};
              (oe(
                (N) => {
                  (ie(x, N), h !== (h = _(E).id) && (A.value = (A.__value = _(E).id) ?? ''));
                },
                [() => pe(_(E))]
              ),
                f(S, A));
            }
          ),
            O(D));
          var I;
          (Vs(D),
            oe(
              (S) => {
                (ie(P, S), I !== (I = _(B)) && ((D.value = (D.__value = _(B)) ?? ''), js(D, _(B))));
              },
              [() => ee(te.selectPlaceholder)]
            ),
            f(L, D));
        },
        $$slots: { default: !0 }
      });
    },
    n = Ce.Item,
    o = Ce.ItemTrigger,
    a = Ce.ItemContent,
    s = Ce.ItemIndicator;
  let i = $t(Ot(['basic-info', 'delivery-availability', 'rolling-stock'])),
    c = Ot({
      manufacturer_id: '',
      product_code: '',
      description: '',
      details: null,
      power_method: '',
      scale: '',
      epoch: '',
      category: '',
      delivery_date: null,
      availability_status: null,
      rolling_stocks: []
    }),
    u = $t(Ot({})),
    g = $t(!1);
  const C = y(() => c.rolling_stocks.length > 0);
  function l() {
    c.rolling_stocks.push(wg());
  }
  function m(V) {
    c.rolling_stocks.splice(V, 1);
  }
  function k(V) {
    const ve = structuredClone(c.rolling_stocks[V]);
    c.rolling_stocks.push(ve);
  }
  function j(V) {
    window.location.assign(V);
  }
  function pe(V) {
    return 'name' in V ? V.name : ee(V);
  }
  async function se() {
    (dt(g, !0), dt(u, {}, !0));
    try {
      const V = {
          manufacturer_id: c.manufacturer_id,
          product_code: c.product_code,
          description: c.description,
          details: c.details,
          power_method: c.power_method,
          scale: c.scale,
          epoch: c.epoch,
          category: c.category,
          delivery_date: c.delivery_date,
          availability_status: c.availability_status === '' ? null : c.availability_status,
          rolling_stocks: c.rolling_stocks.map(Ig)
        },
        ve = bg.parse(V),
        _e = await Us.createRailwayModel(ve);
      _e.status === 'ok'
        ? j(`/models/${_e.data}`)
        : (_(u).general = typeof _e.error == 'string' ? _e.error : JSON.stringify(_e.error));
    } catch (V) {
      V.issues
        ? V.issues.forEach((_e) => {
            const F = _e.path.join('.');
            _(u)[F] = _e.message;
          })
        : (_(u).general = typeof V == 'string' ? V : 'An unexpected error occurred');
    } finally {
      dt(g, !1);
    }
  }
  var R = Gh(),
    le = $(R),
    ge = $(le, !0);
  O(le);
  var Ae = v(le, 2);
  {
    var At = (V) => {
      var ve = zh(),
        _e = $(ve, !0);
      (O(ve), oe(() => ie(_e, _(u).general)), f(V, ve));
    };
    Ie(Ae, (V) => {
      _(u).general && V(At);
    });
  }
  var Ve = v(Ae, 2),
    Ye = $(Ve);
  Ce(Ye, {
    get value() {
      return _(i);
    },
    onValueChange: (V) => dt(i, V.value, !0),
    multiple: !0,
    collapsible: !0,
    class: 'space-y-3',
    children: (V, ve) => {
      var _e = Uh(),
        F = Z(_e);
      n(F, {
        value: 'basic-info',
        class: 'rounded-lg border border-surface-600',
        children: (K, B) => {
          var L = Nh(),
            U = Z(L);
          o(U, {
            class: 'flex w-full items-center justify-between px-3 py-2 text-left',
            children: (z, P) => {
              var w = $h(),
                I = Z(w),
                S = $(I, !0);
              O(I);
              var E = v(I, 2);
              (s(E, { class: 'text-muted text-sm' }),
                oe((A) => ie(S, A), [() => ee(te.basicInfo)]),
                f(z, w));
            },
            $$slots: { default: !0 }
          });
          var D = v(U, 2);
          (a(D, {
            class: 'px-3 pt-1 pb-4',
            children: (z, P) => {
              var w = Rh(),
                I = $(w);
              r(
                I,
                () => te.manufacturer,
                () => _(u).manufacturer_id,
                () => !0,
                () => c.manufacturer_id,
                () => Cg,
                () => (M) => (c.manufacturer_id = M)
              );
              var S = v(I, 2);
              H(S, {
                get label() {
                  return te.productCode;
                },
                get error() {
                  return _(u).product_code;
                },
                required: !0,
                children: (M, Y) => {
                  var ne = Oh();
                  (q(ne),
                    oe((de) => tt(ne, 'placeholder', de), [() => ee(te.productCodePlaceholder)]),
                    Q(
                      ne,
                      () => c.product_code,
                      (de) => (c.product_code = de)
                    ),
                    f(M, ne));
                },
                $$slots: { default: !0 }
              });
              var E = v(S, 2);
              H(E, {
                get label() {
                  return te.description;
                },
                get error() {
                  return _(u).description;
                },
                required: !0,
                children: (M, Y) => {
                  var ne = Ph();
                  (q(ne),
                    oe((de) => tt(ne, 'placeholder', de), [() => ee(te.descriptionPlaceholder)]),
                    Q(
                      ne,
                      () => c.description,
                      (de) => (c.description = de)
                    ),
                    f(M, ne));
                },
                $$slots: { default: !0 }
              });
              var A = v(E, 2);
              r(
                A,
                () => te.category,
                () => _(u).category,
                () => !0,
                () => c.category,
                () => Tg,
                () => (M) => (c.category = M)
              );
              var x = v(A, 2);
              r(
                x,
                () => te.scale,
                () => _(u).scale,
                () => !0,
                () => c.scale,
                () => Gs,
                () => (M) => (c.scale = M)
              );
              var h = v(x, 2);
              r(
                h,
                () => te.powerMethod,
                () => _(u).power_method,
                () => !0,
                () => c.power_method,
                () => Zg,
                () => (M) => (c.power_method = M)
              );
              var N = v(h, 2);
              (r(
                N,
                () => te.epoch,
                () => _(u).epoch,
                () => !0,
                () => c.epoch,
                () => Pg,
                () => (M) => (c.epoch = M)
              ),
                O(w),
                f(z, w));
            },
            $$slots: { default: !0 }
          }),
            f(K, L));
        },
        $$slots: { default: !0 }
      });
      var W = v(F, 2);
      n(W, {
        value: 'delivery-availability',
        class: 'rounded-lg border border-surface-600',
        children: (K, B) => {
          var L = Mh(),
            U = Z(L);
          o(U, {
            class: 'flex w-full items-center justify-between px-3 py-2 text-left',
            children: (z, P) => {
              var w = Dh(),
                I = Z(w),
                S = $(I, !0);
              O(I);
              var E = v(I, 2);
              (s(E, { class: 'text-muted text-sm' }),
                oe((A) => ie(S, A), [() => ee(te.deliveryAvailability)]),
                f(z, w));
            },
            $$slots: { default: !0 }
          });
          var D = v(U, 2);
          (a(D, {
            class: 'px-3 pt-1 pb-4',
            children: (z, P) => {
              var w = Lh(),
                I = $(w);
              H(I, {
                get label() {
                  return te.deliveryDate;
                },
                get error() {
                  return _(u).delivery_date;
                },
                children: (A, x) => {
                  var h = Zh();
                  (q(h),
                    oe((N) => tt(h, 'placeholder', N), [() => ee(te.deliveryDatePlaceholder)]),
                    Q(
                      h,
                      () => c.delivery_date,
                      (N) => (c.delivery_date = N)
                    ),
                    f(A, h));
                },
                $$slots: { default: !0 }
              });
              var S = v(I, 2);
              r(
                S,
                () => te.availabilityStatus,
                () => _(u).availability_status,
                () => !1,
                () => c.availability_status ?? '',
                () => Ag,
                () => (A) => (c.availability_status = A || null)
              );
              var E = v(S, 2);
              (H(E, {
                get label() {
                  return te.additionalDetails;
                },
                get error() {
                  return _(u).details;
                },
                children: (A, x) => {
                  var h = xh();
                  (Fs(h),
                    oe((N) => tt(h, 'placeholder', N), [() => ee(te.detailsPlaceholder)]),
                    Q(
                      h,
                      () => c.details,
                      (N) => (c.details = N)
                    ),
                    f(A, h));
                },
                $$slots: { default: !0 }
              }),
                O(w),
                f(z, w));
            },
            $$slots: { default: !0 }
          }),
            f(K, L));
        },
        $$slots: { default: !0 }
      });
      var X = v(W, 2);
      (n(X, {
        value: 'rolling-stock',
        class: 'rounded-lg border border-surface-600',
        children: (K, B) => {
          var L = Kh(),
            U = Z(L);
          o(U, {
            class: 'flex w-full items-center justify-between px-3 py-2 text-left',
            children: (z, P) => {
              var w = Vh(),
                I = Z(w),
                S = $(I),
                E = v(S),
                A = $(E, !0);
              (O(E), O(I));
              var x = v(I, 2);
              (s(x, { class: 'text-muted text-sm' }),
                oe(
                  (h) => {
                    (ie(S, `${h ?? ''} `), ie(A, c.rolling_stocks.length));
                  },
                  [() => ee(te.rollingStock)]
                ),
                f(z, w));
            },
            $$slots: { default: !0 }
          });
          var D = v(U, 2);
          (a(D, {
            class: 'px-3 pt-1 pb-4',
            children: (z, P) => {
              var w = Fh(),
                I = $(w);
              {
                var S = (h) => {
                  var N = jh();
                  f(h, N);
                };
                Ie(I, (h) => {
                  _(C) || h(S);
                });
              }
              var E = v(I, 2);
              Nt(
                E,
                17,
                () => c.rolling_stocks,
                Ks,
                (h, N, M) => {
                  kh(h, {
                    get rs() {
                      return _(N);
                    },
                    index: M,
                    get errors() {
                      return _(u);
                    },
                    get rollingStockCategoriesData() {
                      return xg;
                    },
                    get railwayCompaniesData() {
                      return kg;
                    },
                    get locomotiveTypesData() {
                      return Ng;
                    },
                    get passengerCarTypesData() {
                      return Dg;
                    },
                    get freightCarTypesData() {
                      return Rg;
                    },
                    get electricMultipleUnitTypesData() {
                      return Og;
                    },
                    get controlsData() {
                      return zg;
                    },
                    get dccInterfacesData() {
                      return $g;
                    },
                    get serviceLevelsData() {
                      return Lg;
                    },
                    get formLabels() {
                      return te;
                    },
                    onDuplicate: () => k(M),
                    onDelete: () => m(M)
                  });
                }
              );
              var A = v(E, 2);
              A.__click = l;
              var x = $(A);
              (O(A),
                O(w),
                oe((h) => ie(x, `+ ${h ?? ''}`), [() => ee(te.addRollingStock)]),
                f(z, w));
            },
            $$slots: { default: !0 }
          }),
            f(K, L));
        },
        $$slots: { default: !0 }
      }),
        f(V, _e));
    },
    $$slots: { default: !0 }
  });
  var lt = v(Ye, 2),
    Pe = $(lt),
    ut = $(Pe, !0);
  O(Pe);
  var je = v(Pe, 2);
  je.__click = () => j('/');
  var _t = $(je, !0);
  (O(je),
    O(lt),
    O(Ve),
    O(R),
    oe(
      (V, ve, _e) => {
        (ie(ge, V), (Pe.disabled = _(g)), ie(ut, ve), ie(_t, _e));
      },
      [
        () => ee(te.title),
        () => (_(g) ? `${ee(te.create)}...` : ee(te.create)),
        () => ee(te.cancel)
      ]
    ),
    Ms('submit', Ve, (V) => {
      (V.preventDefault(), se());
    }),
    f(e, R),
    $e());
}
Ya(['change', 'click']);
function sv(e) {
  Bh(e, {});
}
export { sv as component };
