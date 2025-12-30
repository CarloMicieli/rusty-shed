import {
  r as A,
  i as Y,
  w as j,
  e as N,
  f as Z,
  g as L,
  u as q,
  s as z,
  h as H,
  A as J,
  j as ee,
  k as te,
  l as se,
  m as w,
  n as ie,
  o as S,
  M as ne
} from './Cxbe8PBw.js';
var U = 'layer:request-dismiss',
  B = {
    layers: [],
    branches: [],
    count() {
      return this.layers.length;
    },
    pointerBlockingLayers() {
      return this.layers.filter((e) => e.pointerBlocking);
    },
    topMostPointerBlockingLayer() {
      return [...this.pointerBlockingLayers()].slice(-1)[0];
    },
    hasPointerBlockingLayer() {
      return this.pointerBlockingLayers().length > 0;
    },
    isBelowPointerBlockingLayer(e) {
      const t = this.indexOf(e),
        i = this.topMostPointerBlockingLayer()
          ? this.indexOf(this.topMostPointerBlockingLayer()?.node)
          : -1;
      return t < i;
    },
    isTopMost(e) {
      return this.layers[this.count() - 1]?.node === e;
    },
    getNestedLayers(e) {
      return Array.from(this.layers).slice(this.indexOf(e) + 1);
    },
    getLayersByType(e) {
      return this.layers.filter((t) => t.type === e);
    },
    getNestedLayersByType(e, t) {
      const i = this.indexOf(e);
      return i === -1 ? [] : this.layers.slice(i + 1).filter((s) => s.type === t);
    },
    getParentLayerOfType(e, t) {
      const i = this.indexOf(e);
      if (!(i <= 0))
        return this.layers
          .slice(0, i)
          .reverse()
          .find((s) => s.type === t);
    },
    countNestedLayersOfType(e, t) {
      return this.getNestedLayersByType(e, t).length;
    },
    isInNestedLayer(e, t) {
      return this.getNestedLayers(e).some((i) => N(i.node, t));
    },
    isInBranch(e) {
      return Array.from(this.branches).some((t) => N(t, e));
    },
    add(e) {
      (this.layers.push(e), this.syncLayers());
    },
    addBranch(e) {
      this.branches.push(e);
    },
    remove(e) {
      const t = this.indexOf(e);
      t < 0 ||
        (t < this.count() - 1 && this.getNestedLayers(e).forEach((s) => B.dismiss(s.node, e)),
        this.layers.splice(t, 1),
        this.syncLayers());
    },
    removeBranch(e) {
      const t = this.branches.indexOf(e);
      t >= 0 && this.branches.splice(t, 1);
    },
    syncLayers() {
      this.layers.forEach((e, t) => {
        (e.node.style.setProperty('--layer-index', `${t}`),
          e.node.removeAttribute('data-nested'),
          e.node.removeAttribute('data-has-nested'),
          this.getParentLayerOfType(e.node, e.type) && e.node.setAttribute('data-nested', e.type));
        const s = this.countNestedLayersOfType(e.node, e.type);
        (s > 0 && e.node.setAttribute('data-has-nested', e.type),
          e.node.style.setProperty('--nested-layer-count', `${s}`));
      });
    },
    indexOf(e) {
      return this.layers.findIndex((t) => t.node === e);
    },
    dismiss(e, t) {
      const i = this.indexOf(e);
      if (i === -1) return;
      const s = this.layers[i];
      (oe(e, U, (n) => {
        (s.requestDismiss?.(n), n.defaultPrevented || s?.dismiss());
      }),
        re(e, U, {
          originalLayer: e,
          targetLayer: t,
          originalIndex: i,
          targetIndex: t ? this.indexOf(t) : -1
        }),
        this.syncLayers());
    },
    clear() {
      this.remove(this.layers[0].node);
    }
  };
function re(e, t, i) {
  const s = e.ownerDocument.defaultView || window,
    n = new s.CustomEvent(t, { cancelable: !0, bubbles: !0, detail: i });
  return e.dispatchEvent(n);
}
function oe(e, t, i) {
  e.addEventListener(t, i, { once: !0 });
}
function ae(e, t = {}) {
  const { defer: i } = t,
    s = i ? A : (a) => a(),
    n = [];
  return (
    n.push(
      s(() => {
        const a = Y(e) ? e() : e;
        if (!a) {
          j('[@zag-js/dismissable] branch node is `null` or `undefined`');
          return;
        }
        (B.addBranch(a),
          n.push(() => {
            B.removeBranch(a);
          }));
      })
    ),
    () => {
      n.forEach((a) => a?.());
    }
  );
}
var ce = ee('toast').parts(
    'group',
    'root',
    'title',
    'description',
    'actionTrigger',
    'closeTrigger'
  ),
  I = ce.build(),
  le = (e) => `toast-group:${e}`,
  V = (e, t) => e.getById(`toast-group:${t}`),
  Q = (e) => `toast:${e.id}`,
  W = (e) => e.getById(Q(e)),
  $ = (e) => `toast:${e.id}:title`,
  G = (e) => `toast:${e.id}:description`,
  ue = (e) => `toast${e.id}:close`,
  _ = { info: 5e3, error: 5e3, success: 2e3, loading: 1 / 0, DEFAULT: 5e3 };
function F(e, t) {
  return e ?? _[t] ?? _.DEFAULT;
}
var de = (e) => (typeof e == 'string' ? { left: e, right: e, bottom: e, top: e } : e);
function ge(e, t) {
  const { prop: i, computed: s, context: n } = e,
    { offsets: a, gap: c } = i('store').attrs,
    u = n.get('heights'),
    l = de(a),
    y = i('dir') === 'rtl',
    d = t.replace('-start', y ? '-right' : '-left').replace('-end', y ? '-left' : '-right'),
    g = d.includes('right'),
    v = d.includes('left'),
    h = {
      position: 'fixed',
      pointerEvents: s('count') > 0 ? void 0 : 'none',
      display: 'flex',
      flexDirection: 'column',
      '--gap': `${c}px`,
      '--first-height': `${u[0]?.height || 0}px`,
      '--viewport-offset-left': l.left,
      '--viewport-offset-right': l.right,
      '--viewport-offset-top': l.top,
      '--viewport-offset-bottom': l.bottom,
      zIndex: ne
    };
  let f = 'center';
  if ((g && (f = 'flex-end'), v && (f = 'flex-start'), (h.alignItems = f), d.includes('top'))) {
    const m = l.top;
    h.top = `max(env(safe-area-inset-top, 0px), ${m})`;
  }
  if (d.includes('bottom')) {
    const m = l.bottom;
    h.bottom = `max(env(safe-area-inset-bottom, 0px), ${m})`;
  }
  if (!d.includes('left')) {
    const m = l.right;
    h.insetInlineEnd = `calc(env(safe-area-inset-right, 0px) + ${m})`;
  }
  if (!d.includes('right')) {
    const m = l.left;
    h.insetInlineStart = `calc(env(safe-area-inset-left, 0px) + ${m})`;
  }
  return h;
}
function pe(e, t) {
  const { prop: i, context: s, computed: n } = e,
    a = i('parent'),
    c = a.computed('placement'),
    { gap: u } = a.prop('store').attrs,
    [l] = c.split('-'),
    y = s.get('mounted'),
    d = s.get('remainingTime'),
    g = n('height'),
    v = n('frontmost'),
    h = !v,
    f = !i('stacked'),
    m = i('stacked'),
    D = i('type') === 'loading' ? Number.MAX_SAFE_INTEGER : d,
    R = n('heightIndex') * u + n('heightBefore'),
    O = {
      position: 'absolute',
      pointerEvents: 'auto',
      '--opacity': '0',
      '--remove-delay': `${i('removeDelay')}ms`,
      '--duration': `${D}ms`,
      '--initial-height': `${g}px`,
      '--offset': `${R}px`,
      '--index': i('index'),
      '--z-index': n('zIndex'),
      '--lift-amount': 'calc(var(--lift) * var(--gap))',
      '--y': '100%',
      '--x': '0'
    },
    p = (C) => Object.assign(O, C);
  return (
    l === 'top'
      ? p({ top: '0', '--sign': '-1', '--y': '-100%', '--lift': '1' })
      : l === 'bottom' && p({ bottom: '0', '--sign': '1', '--y': '100%', '--lift': '-1' }),
    y &&
      (p({ '--y': '0', '--opacity': '1' }),
      m && p({ '--y': 'calc(var(--lift) * var(--offset))', '--height': 'var(--initial-height)' })),
    t || p({ '--opacity': '0', pointerEvents: 'none' }),
    h &&
      f &&
      (p({
        '--base-scale': 'var(--index) * 0.05 + 1',
        '--y': 'calc(var(--lift-amount) * var(--index))',
        '--scale': 'calc(-1 * var(--base-scale))',
        '--height': 'var(--first-height)'
      }),
      t || p({ '--y': 'calc(var(--sign) * 40%)' })),
    h && m && !t && p({ '--y': 'calc(var(--lift) * var(--offset) + var(--lift) * -100%)' }),
    v && !t && p({ '--y': 'calc(var(--lift) * -100%)' }),
    O
  );
}
function he(e, t) {
  const { computed: i } = e,
    s = { position: 'absolute', inset: '0', scale: '1 2', pointerEvents: t ? 'none' : 'auto' },
    n = (a) => Object.assign(s, a);
  return (i('frontmost') && !t && n({ height: 'calc(var(--initial-height) + 80%)' }), s);
}
function fe() {
  return {
    position: 'absolute',
    left: '0',
    height: 'calc(var(--gap) + 2px)',
    bottom: '100%',
    width: '100%'
  };
}
function me(e, t) {
  const { context: i, prop: s, send: n, refs: a, computed: c } = e;
  return {
    getCount() {
      return i.get('toasts').length;
    },
    getToasts() {
      return i.get('toasts');
    },
    getGroupProps(u = {}) {
      const { label: l = 'Notifications' } = u,
        { hotkey: y } = s('store').attrs,
        d = y.join('+').replace(/Key/g, '').replace(/Digit/g, ''),
        g = c('placement'),
        [v, h = 'center'] = g.split('-');
      return t.element({
        ...I.group.attrs,
        dir: s('dir'),
        tabIndex: -1,
        'aria-label': `${g} ${l} ${d}`,
        id: le(g),
        'data-placement': g,
        'data-side': v,
        'data-align': h,
        'aria-live': 'polite',
        role: 'region',
        style: ge(e, g),
        onMouseEnter() {
          a.get('ignoreMouseTimer').isActive() || n({ type: 'REGION.POINTER_ENTER', placement: g });
        },
        onMouseMove() {
          a.get('ignoreMouseTimer').isActive() || n({ type: 'REGION.POINTER_ENTER', placement: g });
        },
        onMouseLeave() {
          a.get('ignoreMouseTimer').isActive() || n({ type: 'REGION.POINTER_LEAVE', placement: g });
        },
        onFocus(f) {
          n({ type: 'REGION.FOCUS', target: f.relatedTarget });
        },
        onBlur(f) {
          a.get('isFocusWithin') &&
            !N(f.currentTarget, f.relatedTarget) &&
            queueMicrotask(() => n({ type: 'REGION.BLUR' }));
        }
      });
    },
    subscribe(u) {
      return s('store').subscribe(() => u(i.get('toasts')));
    }
  };
}
var { guards: ye, createMachine: ve } = z(),
  { and: Te } = ye,
  Ee = ve({
    props({ props: e }) {
      return { dir: 'ltr', id: q(), ...e, store: e.store };
    },
    initialState({ prop: e }) {
      return e('store').attrs.overlap ? 'overlap' : 'stack';
    },
    refs() {
      return {
        lastFocusedEl: null,
        isFocusWithin: !1,
        isPointerWithin: !1,
        ignoreMouseTimer: J.create(),
        dismissableCleanup: void 0
      };
    },
    context({ bindable: e }) {
      return {
        toasts: e(() => ({
          defaultValue: [],
          sync: !0,
          hash: (t) => t.map((i) => i.id).join(',')
        })),
        heights: e(() => ({ defaultValue: [], sync: !0 }))
      };
    },
    computed: {
      count: ({ context: e }) => e.get('toasts').length,
      overlap: ({ prop: e }) => e('store').attrs.overlap,
      placement: ({ prop: e }) => e('store').attrs.placement
    },
    effects: ['subscribeToStore', 'trackDocumentVisibility', 'trackHotKeyPress'],
    watch({ track: e, context: t, action: i }) {
      e([() => t.hash('toasts')], () => {
        queueMicrotask(() => {
          i(['collapsedIfEmpty', 'setDismissableBranch']);
        });
      });
    },
    exit: ['clearDismissableBranch', 'clearLastFocusedEl', 'clearMouseEventTimer'],
    on: {
      'DOC.HOTKEY': { actions: ['focusRegionEl'] },
      'REGION.BLUR': [
        {
          guard: Te('isOverlapping', 'isPointerOut'),
          target: 'overlap',
          actions: ['collapseToasts', 'resumeToasts', 'restoreFocusIfPointerOut']
        },
        {
          guard: 'isPointerOut',
          target: 'stack',
          actions: ['resumeToasts', 'restoreFocusIfPointerOut']
        },
        { actions: ['clearFocusWithin'] }
      ],
      'TOAST.REMOVE': { actions: ['removeToast', 'removeHeight', 'ignoreMouseEventsTemporarily'] },
      'TOAST.PAUSE': { actions: ['pauseToasts'] }
    },
    states: {
      stack: {
        on: {
          'REGION.POINTER_LEAVE': [
            {
              guard: 'isOverlapping',
              target: 'overlap',
              actions: ['clearPointerWithin', 'resumeToasts', 'collapseToasts']
            },
            { actions: ['clearPointerWithin', 'resumeToasts'] }
          ],
          'REGION.OVERLAP': { target: 'overlap', actions: ['collapseToasts'] },
          'REGION.FOCUS': { actions: ['setLastFocusedEl', 'pauseToasts'] },
          'REGION.POINTER_ENTER': { actions: ['setPointerWithin', 'pauseToasts'] }
        }
      },
      overlap: {
        on: {
          'REGION.STACK': { target: 'stack', actions: ['expandToasts'] },
          'REGION.POINTER_ENTER': {
            target: 'stack',
            actions: ['setPointerWithin', 'pauseToasts', 'expandToasts']
          },
          'REGION.FOCUS': {
            target: 'stack',
            actions: ['setLastFocusedEl', 'pauseToasts', 'expandToasts']
          }
        }
      }
    },
    implementations: {
      guards: {
        isOverlapping: ({ computed: e }) => e('overlap'),
        isPointerOut: ({ refs: e }) => !e.get('isPointerWithin')
      },
      effects: {
        subscribeToStore({ context: e, prop: t }) {
          return t('store').subscribe((i) => {
            if (i.dismiss) {
              e.set('toasts', (s) => s.filter((n) => n.id !== i.id));
              return;
            }
            e.set('toasts', (s) => {
              const n = s.findIndex((a) => a.id === i.id);
              return n !== -1
                ? [...s.slice(0, n), { ...s[n], ...i }, ...s.slice(n + 1)]
                : [i, ...s];
            });
          });
        },
        trackHotKeyPress({ prop: e, send: t }) {
          return H(
            document,
            'keydown',
            (s) => {
              const { hotkey: n } = e('store').attrs;
              n.every((c) => s[c] || s.code === c) && t({ type: 'DOC.HOTKEY' });
            },
            { capture: !0 }
          );
        },
        trackDocumentVisibility({ prop: e, send: t, scope: i }) {
          const { pauseOnPageIdle: s } = e('store').attrs;
          if (!s) return;
          const n = i.getDoc();
          return H(n, 'visibilitychange', () => {
            const a = n.visibilityState === 'hidden';
            t({ type: a ? 'PAUSE_ALL' : 'RESUME_ALL' });
          });
        }
      },
      actions: {
        setDismissableBranch({ refs: e, context: t, computed: i, scope: s }) {
          const n = t.get('toasts'),
            a = i('placement'),
            c = n.length > 0;
          if (!c) {
            e.get('dismissableCleanup')?.();
            return;
          }
          if (c && e.get('dismissableCleanup')) return;
          const l = ae(() => V(s, a), { defer: !0 });
          e.set('dismissableCleanup', l);
        },
        clearDismissableBranch({ refs: e }) {
          e.get('dismissableCleanup')?.();
        },
        focusRegionEl({ scope: e, computed: t }) {
          queueMicrotask(() => {
            V(e, t('placement'))?.focus();
          });
        },
        pauseToasts({ prop: e }) {
          e('store').pause();
        },
        resumeToasts({ prop: e }) {
          e('store').resume();
        },
        expandToasts({ prop: e }) {
          e('store').expand();
        },
        collapseToasts({ prop: e }) {
          e('store').collapse();
        },
        removeToast({ prop: e, event: t }) {
          e('store').remove(t.id);
        },
        removeHeight({ event: e, context: t }) {
          e?.id != null &&
            queueMicrotask(() => {
              t.set('heights', (i) => i.filter((s) => s.id !== e.id));
            });
        },
        collapsedIfEmpty({ send: e, computed: t }) {
          !t('overlap') || t('count') > 1 || e({ type: 'REGION.OVERLAP' });
        },
        setLastFocusedEl({ refs: e, event: t }) {
          e.get('isFocusWithin') ||
            !t.target ||
            (e.set('isFocusWithin', !0), e.set('lastFocusedEl', t.target));
        },
        restoreFocusIfPointerOut({ refs: e }) {
          !e.get('lastFocusedEl') ||
            e.get('isPointerWithin') ||
            (e.get('lastFocusedEl')?.focus({ preventScroll: !0 }),
            e.set('lastFocusedEl', null),
            e.set('isFocusWithin', !1));
        },
        setPointerWithin({ refs: e }) {
          e.set('isPointerWithin', !0);
        },
        clearPointerWithin({ refs: e }) {
          (e.set('isPointerWithin', !1),
            e.get('lastFocusedEl') &&
              !e.get('isFocusWithin') &&
              (e.get('lastFocusedEl')?.focus({ preventScroll: !0 }), e.set('lastFocusedEl', null)));
        },
        clearFocusWithin({ refs: e }) {
          e.set('isFocusWithin', !1);
        },
        clearLastFocusedEl({ refs: e }) {
          e.get('lastFocusedEl') &&
            (e.get('lastFocusedEl')?.focus({ preventScroll: !0 }),
            e.set('lastFocusedEl', null),
            e.set('isFocusWithin', !1));
        },
        ignoreMouseEventsTemporarily({ refs: e }) {
          e.get('ignoreMouseTimer').request();
        },
        clearMouseEventTimer({ refs: e }) {
          e.get('ignoreMouseTimer').cancel();
        }
      }
    }
  });
function Fe(e, t) {
  const { state: i, send: s, prop: n, scope: a, context: c, computed: u } = e,
    l = i.hasTag('visible'),
    y = i.hasTag('paused'),
    d = c.get('mounted'),
    g = u('frontmost'),
    v = n('parent').computed('placement'),
    h = n('type'),
    f = n('stacked'),
    m = n('title'),
    k = n('description'),
    D = n('action'),
    [R, O = 'center'] = v.split('-');
  return {
    type: h,
    title: m,
    description: k,
    placement: v,
    visible: l,
    paused: y,
    closable: !!n('closable'),
    pause() {
      s({ type: 'PAUSE' });
    },
    resume() {
      s({ type: 'RESUME' });
    },
    dismiss() {
      s({ type: 'DISMISS', src: 'programmatic' });
    },
    getRootProps() {
      return t.element({
        ...I.root.attrs,
        dir: n('dir'),
        id: Q(a),
        'data-state': l ? 'open' : 'closed',
        'data-type': h,
        'data-placement': v,
        'data-align': O,
        'data-side': R,
        'data-mounted': S(d),
        'data-paused': S(y),
        'data-first': S(g),
        'data-sibling': S(!g),
        'data-stack': S(f),
        'data-overlap': S(!f),
        role: 'status',
        'aria-atomic': 'true',
        'aria-describedby': k ? G(a) : void 0,
        'aria-labelledby': m ? $(a) : void 0,
        tabIndex: 0,
        style: pe(e, l),
        onKeyDown(p) {
          p.defaultPrevented ||
            (p.key == 'Escape' && (s({ type: 'DISMISS', src: 'keyboard' }), p.preventDefault()));
        }
      });
    },
    getGhostBeforeProps() {
      return t.element({ 'data-ghost': 'before', style: he(e, l) });
    },
    getGhostAfterProps() {
      return t.element({ 'data-ghost': 'after', style: fe() });
    },
    getTitleProps() {
      return t.element({ ...I.title.attrs, id: $(a) });
    },
    getDescriptionProps() {
      return t.element({ ...I.description.attrs, id: G(a) });
    },
    getActionTriggerProps() {
      return t.button({
        ...I.actionTrigger.attrs,
        type: 'button',
        onClick(p) {
          p.defaultPrevented || (D?.onClick?.(), s({ type: 'DISMISS', src: 'user' }));
        }
      });
    },
    getCloseTriggerProps() {
      return t.button({
        id: ue(a),
        ...I.closeTrigger.attrs,
        type: 'button',
        'aria-label': 'Dismiss notification',
        onClick(p) {
          p.defaultPrevented || s({ type: 'DISMISS', src: 'user' });
        }
      });
    }
  };
}
var { not: be } = se(),
  Ae = te({
    props({ props: e }) {
      return (
        ie(e, ['id', 'type', 'parent', 'removeDelay'], 'toast'),
        { closable: !0, ...e, duration: F(e.duration, e.type) }
      );
    },
    initialState({ prop: e }) {
      return e('type') === 'loading' || e('duration') === 1 / 0 ? 'visible:persist' : 'visible';
    },
    context({ prop: e, bindable: t }) {
      return {
        remainingTime: t(() => ({ defaultValue: F(e('duration'), e('type')) })),
        createdAt: t(() => ({ defaultValue: Date.now() })),
        mounted: t(() => ({ defaultValue: !1 })),
        initialHeight: t(() => ({ defaultValue: 0 }))
      };
    },
    refs() {
      return { closeTimerStartTime: Date.now(), lastCloseStartTimerStartTime: 0 };
    },
    computed: {
      zIndex: ({ prop: e }) => {
        const t = e('parent').context.get('toasts'),
          i = t.findIndex((s) => s.id === e('id'));
        return t.length - i;
      },
      height: ({ prop: e }) =>
        e('parent')
          .context.get('heights')
          .find((s) => s.id === e('id'))?.height ?? 0,
      heightIndex: ({ prop: e }) =>
        e('parent')
          .context.get('heights')
          .findIndex((i) => i.id === e('id')),
      frontmost: ({ prop: e }) => e('index') === 0,
      heightBefore: ({ prop: e }) => {
        const t = e('parent').context.get('heights'),
          i = t.findIndex((s) => s.id === e('id'));
        return t.reduce((s, n, a) => (a >= i ? s : s + n.height), 0);
      },
      shouldPersist: ({ prop: e }) => e('type') === 'loading' || e('duration') === 1 / 0
    },
    watch({ track: e, prop: t, send: i }) {
      (e([() => t('message')], () => {
        const s = t('message');
        s && i({ type: s, src: 'programmatic' });
      }),
        e([() => t('type'), () => t('duration')], () => {
          i({ type: 'UPDATE' });
        }));
    },
    on: {
      UPDATE: [
        { guard: 'shouldPersist', target: 'visible:persist', actions: ['resetCloseTimer'] },
        { target: 'visible:updating', actions: ['resetCloseTimer'] }
      ],
      MEASURE: { actions: ['measureHeight'] }
    },
    entry: ['setMounted', 'measureHeight', 'invokeOnVisible'],
    effects: ['trackHeight'],
    states: {
      'visible:updating': {
        tags: ['visible', 'updating'],
        effects: ['waitForNextTick'],
        on: { SHOW: { target: 'visible' } }
      },
      'visible:persist': {
        tags: ['visible', 'paused'],
        on: {
          RESUME: { guard: be('isLoadingType'), target: 'visible', actions: ['setCloseTimer'] },
          DISMISS: { target: 'dismissing' }
        }
      },
      visible: {
        tags: ['visible'],
        effects: ['waitForDuration'],
        on: {
          DISMISS: { target: 'dismissing' },
          PAUSE: { target: 'visible:persist', actions: ['syncRemainingTime'] }
        }
      },
      dismissing: {
        entry: ['invokeOnDismiss'],
        effects: ['waitForRemoveDelay'],
        on: { REMOVE: { target: 'unmounted', actions: ['notifyParentToRemove'] } }
      },
      unmounted: { entry: ['invokeOnUnmount'] }
    },
    implementations: {
      effects: {
        waitForRemoveDelay({ prop: e, send: t }) {
          return w(() => {
            t({ type: 'REMOVE', src: 'timer' });
          }, e('removeDelay'));
        },
        waitForDuration({ send: e, context: t, computed: i }) {
          if (!i('shouldPersist'))
            return w(() => {
              e({ type: 'DISMISS', src: 'timer' });
            }, t.get('remainingTime'));
        },
        waitForNextTick({ send: e }) {
          return w(() => {
            e({ type: 'SHOW', src: 'timer' });
          }, 0);
        },
        trackHeight({ scope: e, prop: t }) {
          let i;
          return (
            A(() => {
              const s = W(e);
              if (!s) return;
              const n = () => {
                  const u = s.style.height;
                  s.style.height = 'auto';
                  const l = s.getBoundingClientRect().height;
                  s.style.height = u;
                  const y = { id: t('id'), height: l };
                  K(t('parent'), y);
                },
                a = e.getWin(),
                c = new a.MutationObserver(n);
              (c.observe(s, { childList: !0, subtree: !0, characterData: !0 }),
                (i = () => c.disconnect()));
            }),
            () => i?.()
          );
        }
      },
      guards: {
        isLoadingType: ({ prop: e }) => e('type') === 'loading',
        shouldPersist: ({ computed: e }) => e('shouldPersist')
      },
      actions: {
        setMounted({ context: e }) {
          A(() => {
            e.set('mounted', !0);
          });
        },
        measureHeight({ scope: e, prop: t, context: i }) {
          queueMicrotask(() => {
            const s = W(e);
            if (!s) return;
            const n = s.style.height;
            s.style.height = 'auto';
            const a = s.getBoundingClientRect().height;
            ((s.style.height = n), i.set('initialHeight', a));
            const c = { id: t('id'), height: a };
            K(t('parent'), c);
          });
        },
        setCloseTimer({ refs: e }) {
          e.set('closeTimerStartTime', Date.now());
        },
        resetCloseTimer({ context: e, refs: t, prop: i }) {
          (t.set('closeTimerStartTime', Date.now()),
            e.set('remainingTime', F(i('duration'), i('type'))));
        },
        syncRemainingTime({ context: e, refs: t }) {
          e.set('remainingTime', (i) => {
            const s = t.get('closeTimerStartTime'),
              n = Date.now() - s;
            return (t.set('lastCloseStartTimerStartTime', Date.now()), i - n);
          });
        },
        notifyParentToRemove({ prop: e }) {
          e('parent').send({ type: 'TOAST.REMOVE', id: e('id') });
        },
        invokeOnDismiss({ prop: e, event: t }) {
          e('onStatusChange')?.({ status: 'dismissing', src: t.src });
        },
        invokeOnUnmount({ prop: e }) {
          e('onStatusChange')?.({ status: 'unmounted' });
        },
        invokeOnVisible({ prop: e }) {
          e('onStatusChange')?.({ status: 'visible' });
        }
      }
    }
  });
function K(e, t) {
  const { id: i, height: s } = t;
  e.context.set('heights', (n) =>
    n.find((c) => c.id === i)
      ? n.map((c) => (c.id === i ? { ...c, height: s } : c))
      : [{ id: i, height: s }, ...n]
  );
}
var xe = (e, t) => ({ ...t, ...Z(e) });
function Se(e = {}) {
  const t = xe(e, {
    placement: 'bottom',
    overlap: !1,
    max: 24,
    gap: 16,
    offsets: '1rem',
    hotkey: ['altKey', 'KeyT'],
    removeDelay: 200,
    pauseOnPageIdle: !0
  });
  let i = [],
    s = [],
    n = new Set(),
    a = [];
  const c = (r) => (
      i.push(r),
      () => {
        const o = i.indexOf(r);
        i.splice(o, 1);
      }
    ),
    u = (r) => (i.forEach((o) => o(r)), r),
    l = (r) => {
      if (s.length >= t.max) {
        a.push(r);
        return;
      }
      (u(r), s.unshift(r));
    },
    y = () => {
      for (; a.length > 0 && s.length < t.max; ) {
        const r = a.shift();
        r && (u(r), s.unshift(r));
      }
    },
    d = (r) => {
      const o = r.id ?? `toast:${q()}`,
        b = s.find((T) => T.id === o);
      return (
        n.has(o) && n.delete(o),
        b
          ? (s = s.map((T) => (T.id === o ? u({ ...T, ...r, id: o }) : T)))
          : l({
              id: o,
              duration: t.duration,
              removeDelay: t.removeDelay,
              type: 'info',
              ...r,
              stacked: !t.overlap,
              gap: t.gap
            }),
        o
      );
    },
    g = (r) => (
      n.add(r),
      r
        ? (i.forEach((o) => o({ id: r, dismiss: !0 })), (s = s.filter((o) => o.id !== r)), y())
        : (s.forEach((o) => {
            i.forEach((b) => b({ id: o.id, dismiss: !0 }));
          }),
          (s = []),
          (a = [])),
      r
    );
  return {
    attrs: t,
    subscribe: c,
    create: d,
    update: (r, o) => d({ id: r, ...o }),
    remove: g,
    dismiss: (r) => {
      r != null
        ? (s = s.map((o) => (o.id === r ? u({ ...o, message: 'DISMISS' }) : o)))
        : (s = s.map((o) => u({ ...o, message: 'DISMISS' })));
    },
    error: (r) => d({ ...r, type: 'error' }),
    success: (r) => d({ ...r, type: 'success' }),
    info: (r) => d({ ...r, type: 'info' }),
    warning: (r) => d({ ...r, type: 'warning' }),
    loading: (r) => d({ ...r, type: 'loading' }),
    getVisibleToasts: () => s.filter((r) => !n.has(r.id)),
    getCount: () => s.length,
    promise: (r, o, b = {}) => {
      if (!o || !o.loading) {
        j(
          "[zag-js > toast] toaster.promise() requires at least a 'loading' option to be specified"
        );
        return;
      }
      const T = d({ ...b, ...o.loading, promise: r, type: 'loading' });
      let M = !0,
        P;
      const X = L(r)
        .then(async (E) => {
          if (((P = ['resolve', E]), Ie(E) && !E.ok)) {
            M = !1;
            const x = L(o.error, `HTTP Error! status: ${E.status}`);
            d({ ...b, ...x, id: T, type: 'error' });
          } else if (o.success !== void 0) {
            M = !1;
            const x = L(o.success, E);
            d({ ...b, ...x, id: T, type: 'success' });
          }
        })
        .catch(async (E) => {
          if (((P = ['reject', E]), o.error !== void 0)) {
            M = !1;
            const x = L(o.error, E);
            d({ ...b, ...x, id: T, type: 'error' });
          }
        })
        .finally(() => {
          (M && g(T), o.finally?.());
        });
      return {
        id: T,
        unwrap: () =>
          new Promise((E, x) => X.then(() => (P[0] === 'reject' ? x(P[1]) : E(P[1]))).catch(x))
      };
    },
    pause: (r) => {
      r != null
        ? (s = s.map((o) => (o.id === r ? u({ ...o, message: 'PAUSE' }) : o)))
        : (s = s.map((o) => u({ ...o, message: 'PAUSE' })));
    },
    resume: (r) => {
      r != null
        ? (s = s.map((o) => (o.id === r ? u({ ...o, message: 'RESUME' }) : o)))
        : (s = s.map((o) => u({ ...o, message: 'RESUME' })));
    },
    isVisible: (r) => !n.has(r) && !!s.find((o) => o.id === r),
    isDismissed: (r) => n.has(r),
    expand: () => {
      s = s.map((r) => u({ ...r, stacked: !0 }));
    },
    collapse: () => {
      s = s.map((r) => u({ ...r, stacked: !1 }));
    }
  };
}
var Ie = (e) =>
    e &&
    typeof e == 'object' &&
    'ok' in e &&
    typeof e.ok == 'boolean' &&
    'status' in e &&
    typeof e.status == 'number',
  Ne = { connect: me, machine: Ee };
const Be = Se({ placement: 'top-end', overlap: !0, duration: 4e3 });
export { Fe as c, Ne as g, Ae as m, Be as t };
