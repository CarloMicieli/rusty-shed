import {
  invoke as i,
  transformCallback as M,
  SERIALIZE_TO_IPC_FN as u,
  Resource as G
} from './mPlcS5K-.js';
var l;
(function (t) {
  ((t.WINDOW_RESIZED = 'tauri://resize'),
    (t.WINDOW_MOVED = 'tauri://move'),
    (t.WINDOW_CLOSE_REQUESTED = 'tauri://close-requested'),
    (t.WINDOW_DESTROYED = 'tauri://destroyed'),
    (t.WINDOW_FOCUS = 'tauri://focus'),
    (t.WINDOW_BLUR = 'tauri://blur'),
    (t.WINDOW_SCALE_FACTOR_CHANGED = 'tauri://scale-change'),
    (t.WINDOW_THEME_CHANGED = 'tauri://theme-changed'),
    (t.WINDOW_CREATED = 'tauri://window-created'),
    (t.WEBVIEW_CREATED = 'tauri://webview-created'),
    (t.DRAG_ENTER = 'tauri://drag-enter'),
    (t.DRAG_OVER = 'tauri://drag-over'),
    (t.DRAG_DROP = 'tauri://drag-drop'),
    (t.DRAG_LEAVE = 'tauri://drag-leave'));
})(l || (l = {}));
async function C(t, e) {
  (window.__TAURI_EVENT_PLUGIN_INTERNALS__.unregisterListener(t, e),
    await i('plugin:event|unlisten', { event: t, eventId: e }));
}
async function p(t, e, n) {
  var r;
  const a =
    typeof n?.target == 'string'
      ? { kind: 'AnyLabel', label: n.target }
      : (r = n?.target) !== null && r !== void 0
        ? r
        : { kind: 'Any' };
  return i('plugin:event|listen', { event: t, target: a, handler: M(e) }).then(
    (d) => async () => C(t, d)
  );
}
async function W(t, e, n) {
  return p(
    t,
    (r) => {
      (C(t, r.id), e(r));
    },
    n
  );
}
async function x(t, e) {
  await i('plugin:event|emit', { event: t, payload: e });
}
async function N(t, e, n) {
  await i('plugin:event|emit_to', {
    target: typeof t == 'string' ? { kind: 'AnyLabel', label: t } : t,
    event: e,
    payload: n
  });
}
class S {
  constructor(...e) {
    ((this.type = 'Logical'),
      e.length === 1
        ? 'Logical' in e[0]
          ? ((this.width = e[0].Logical.width), (this.height = e[0].Logical.height))
          : ((this.width = e[0].width), (this.height = e[0].height))
        : ((this.width = e[0]), (this.height = e[1])));
  }
  toPhysical(e) {
    return new h(this.width * e, this.height * e);
  }
  [u]() {
    return { width: this.width, height: this.height };
  }
  toJSON() {
    return this[u]();
  }
}
class h {
  constructor(...e) {
    ((this.type = 'Physical'),
      e.length === 1
        ? 'Physical' in e[0]
          ? ((this.width = e[0].Physical.width), (this.height = e[0].Physical.height))
          : ((this.width = e[0].width), (this.height = e[0].height))
        : ((this.width = e[0]), (this.height = e[1])));
  }
  toLogical(e) {
    return new S(this.width / e, this.height / e);
  }
  [u]() {
    return { width: this.width, height: this.height };
  }
  toJSON() {
    return this[u]();
  }
}
class c {
  constructor(e) {
    this.size = e;
  }
  toLogical(e) {
    return this.size instanceof S ? this.size : this.size.toLogical(e);
  }
  toPhysical(e) {
    return this.size instanceof h ? this.size : this.size.toPhysical(e);
  }
  [u]() {
    return { [`${this.size.type}`]: { width: this.size.width, height: this.size.height } };
  }
  toJSON() {
    return this[u]();
  }
}
class P {
  constructor(...e) {
    ((this.type = 'Logical'),
      e.length === 1
        ? 'Logical' in e[0]
          ? ((this.x = e[0].Logical.x), (this.y = e[0].Logical.y))
          : ((this.x = e[0].x), (this.y = e[0].y))
        : ((this.x = e[0]), (this.y = e[1])));
  }
  toPhysical(e) {
    return new o(this.x * e, this.y * e);
  }
  [u]() {
    return { x: this.x, y: this.y };
  }
  toJSON() {
    return this[u]();
  }
}
class o {
  constructor(...e) {
    ((this.type = 'Physical'),
      e.length === 1
        ? 'Physical' in e[0]
          ? ((this.x = e[0].Physical.x), (this.y = e[0].Physical.y))
          : ((this.x = e[0].x), (this.y = e[0].y))
        : ((this.x = e[0]), (this.y = e[1])));
  }
  toLogical(e) {
    return new P(this.x / e, this.y / e);
  }
  [u]() {
    return { x: this.x, y: this.y };
  }
  toJSON() {
    return this[u]();
  }
}
class w {
  constructor(e) {
    this.position = e;
  }
  toLogical(e) {
    return this.position instanceof P ? this.position : this.position.toLogical(e);
  }
  toPhysical(e) {
    return this.position instanceof o ? this.position : this.position.toPhysical(e);
  }
  [u]() {
    return { [`${this.position.type}`]: { x: this.position.x, y: this.position.y } };
  }
  toJSON() {
    return this[u]();
  }
}
class b extends G {
  constructor(e) {
    super(e);
  }
  static async new(e, n, r) {
    return i('plugin:image|new', { rgba: _(e), width: n, height: r }).then((a) => new b(a));
  }
  static async fromBytes(e) {
    return i('plugin:image|from_bytes', { bytes: _(e) }).then((n) => new b(n));
  }
  static async fromPath(e) {
    return i('plugin:image|from_path', { path: e }).then((n) => new b(n));
  }
  async rgba() {
    return i('plugin:image|rgba', { rid: this.rid }).then((e) => new Uint8Array(e));
  }
  async size() {
    return i('plugin:image|size', { rid: this.rid });
  }
}
function _(t) {
  return t == null ? null : typeof t == 'string' ? t : t instanceof b ? t.rid : t;
}
var D;
(function (t) {
  ((t[(t.Critical = 1)] = 'Critical'), (t[(t.Informational = 2)] = 'Informational'));
})(D || (D = {}));
class F {
  constructor(e) {
    ((this._preventDefault = !1), (this.event = e.event), (this.id = e.id));
  }
  preventDefault() {
    this._preventDefault = !0;
  }
  isPreventDefault() {
    return this._preventDefault;
  }
}
var E;
(function (t) {
  ((t.None = 'none'),
    (t.Normal = 'normal'),
    (t.Indeterminate = 'indeterminate'),
    (t.Paused = 'paused'),
    (t.Error = 'error'));
})(E || (E = {}));
function T() {
  return new g(window.__TAURI_INTERNALS__.metadata.currentWindow.label, { skip: !0 });
}
async function f() {
  return i('plugin:window|get_all_windows').then((t) => t.map((e) => new g(e, { skip: !0 })));
}
const v = ['tauri://created', 'tauri://error'];
class g {
  constructor(e, n = {}) {
    var r;
    ((this.label = e),
      (this.listeners = Object.create(null)),
      n?.skip ||
        i('plugin:window|create', {
          options: {
            ...n,
            parent:
              typeof n.parent == 'string'
                ? n.parent
                : (r = n.parent) === null || r === void 0
                  ? void 0
                  : r.label,
            label: e
          }
        })
          .then(async () => this.emit('tauri://created'))
          .catch(async (a) => this.emit('tauri://error', a)));
  }
  static async getByLabel(e) {
    var n;
    return (n = (await f()).find((r) => r.label === e)) !== null && n !== void 0 ? n : null;
  }
  static getCurrent() {
    return T();
  }
  static async getAll() {
    return f();
  }
  static async getFocusedWindow() {
    for (const e of await f()) if (await e.isFocused()) return e;
    return null;
  }
  async listen(e, n) {
    return this._handleTauriEvent(e, n)
      ? () => {
          const r = this.listeners[e];
          r.splice(r.indexOf(n), 1);
        }
      : p(e, n, { target: { kind: 'Window', label: this.label } });
  }
  async once(e, n) {
    return this._handleTauriEvent(e, n)
      ? () => {
          const r = this.listeners[e];
          r.splice(r.indexOf(n), 1);
        }
      : W(e, n, { target: { kind: 'Window', label: this.label } });
  }
  async emit(e, n) {
    if (v.includes(e)) {
      for (const r of this.listeners[e] || []) r({ event: e, id: -1, payload: n });
      return;
    }
    return x(e, n);
  }
  async emitTo(e, n, r) {
    if (v.includes(n)) {
      for (const a of this.listeners[n] || []) a({ event: n, id: -1, payload: r });
      return;
    }
    return N(e, n, r);
  }
  _handleTauriEvent(e, n) {
    return v.includes(e)
      ? (e in this.listeners ? this.listeners[e].push(n) : (this.listeners[e] = [n]), !0)
      : !1;
  }
  async scaleFactor() {
    return i('plugin:window|scale_factor', { label: this.label });
  }
  async innerPosition() {
    return i('plugin:window|inner_position', { label: this.label }).then((e) => new o(e));
  }
  async outerPosition() {
    return i('plugin:window|outer_position', { label: this.label }).then((e) => new o(e));
  }
  async innerSize() {
    return i('plugin:window|inner_size', { label: this.label }).then((e) => new h(e));
  }
  async outerSize() {
    return i('plugin:window|outer_size', { label: this.label }).then((e) => new h(e));
  }
  async isFullscreen() {
    return i('plugin:window|is_fullscreen', { label: this.label });
  }
  async isMinimized() {
    return i('plugin:window|is_minimized', { label: this.label });
  }
  async isMaximized() {
    return i('plugin:window|is_maximized', { label: this.label });
  }
  async isFocused() {
    return i('plugin:window|is_focused', { label: this.label });
  }
  async isDecorated() {
    return i('plugin:window|is_decorated', { label: this.label });
  }
  async isResizable() {
    return i('plugin:window|is_resizable', { label: this.label });
  }
  async isMaximizable() {
    return i('plugin:window|is_maximizable', { label: this.label });
  }
  async isMinimizable() {
    return i('plugin:window|is_minimizable', { label: this.label });
  }
  async isClosable() {
    return i('plugin:window|is_closable', { label: this.label });
  }
  async isVisible() {
    return i('plugin:window|is_visible', { label: this.label });
  }
  async title() {
    return i('plugin:window|title', { label: this.label });
  }
  async theme() {
    return i('plugin:window|theme', { label: this.label });
  }
  async isAlwaysOnTop() {
    return i('plugin:window|is_always_on_top', { label: this.label });
  }
  async center() {
    return i('plugin:window|center', { label: this.label });
  }
  async requestUserAttention(e) {
    let n = null;
    return (
      e && (e === D.Critical ? (n = { type: 'Critical' }) : (n = { type: 'Informational' })),
      i('plugin:window|request_user_attention', { label: this.label, value: n })
    );
  }
  async setResizable(e) {
    return i('plugin:window|set_resizable', { label: this.label, value: e });
  }
  async setEnabled(e) {
    return i('plugin:window|set_enabled', { label: this.label, value: e });
  }
  async isEnabled() {
    return i('plugin:window|is_enabled', { label: this.label });
  }
  async setMaximizable(e) {
    return i('plugin:window|set_maximizable', { label: this.label, value: e });
  }
  async setMinimizable(e) {
    return i('plugin:window|set_minimizable', { label: this.label, value: e });
  }
  async setClosable(e) {
    return i('plugin:window|set_closable', { label: this.label, value: e });
  }
  async setTitle(e) {
    return i('plugin:window|set_title', { label: this.label, value: e });
  }
  async maximize() {
    return i('plugin:window|maximize', { label: this.label });
  }
  async unmaximize() {
    return i('plugin:window|unmaximize', { label: this.label });
  }
  async toggleMaximize() {
    return i('plugin:window|toggle_maximize', { label: this.label });
  }
  async minimize() {
    return i('plugin:window|minimize', { label: this.label });
  }
  async unminimize() {
    return i('plugin:window|unminimize', { label: this.label });
  }
  async show() {
    return i('plugin:window|show', { label: this.label });
  }
  async hide() {
    return i('plugin:window|hide', { label: this.label });
  }
  async close() {
    return i('plugin:window|close', { label: this.label });
  }
  async destroy() {
    return i('plugin:window|destroy', { label: this.label });
  }
  async setDecorations(e) {
    return i('plugin:window|set_decorations', { label: this.label, value: e });
  }
  async setShadow(e) {
    return i('plugin:window|set_shadow', { label: this.label, value: e });
  }
  async setEffects(e) {
    return i('plugin:window|set_effects', { label: this.label, value: e });
  }
  async clearEffects() {
    return i('plugin:window|set_effects', { label: this.label, value: null });
  }
  async setAlwaysOnTop(e) {
    return i('plugin:window|set_always_on_top', { label: this.label, value: e });
  }
  async setAlwaysOnBottom(e) {
    return i('plugin:window|set_always_on_bottom', { label: this.label, value: e });
  }
  async setContentProtected(e) {
    return i('plugin:window|set_content_protected', { label: this.label, value: e });
  }
  async setSize(e) {
    return i('plugin:window|set_size', { label: this.label, value: e instanceof c ? e : new c(e) });
  }
  async setMinSize(e) {
    return i('plugin:window|set_min_size', {
      label: this.label,
      value: e instanceof c ? e : e ? new c(e) : null
    });
  }
  async setMaxSize(e) {
    return i('plugin:window|set_max_size', {
      label: this.label,
      value: e instanceof c ? e : e ? new c(e) : null
    });
  }
  async setSizeConstraints(e) {
    function n(r) {
      return r ? { Logical: r } : null;
    }
    return i('plugin:window|set_size_constraints', {
      label: this.label,
      value: {
        minWidth: n(e?.minWidth),
        minHeight: n(e?.minHeight),
        maxWidth: n(e?.maxWidth),
        maxHeight: n(e?.maxHeight)
      }
    });
  }
  async setPosition(e) {
    return i('plugin:window|set_position', {
      label: this.label,
      value: e instanceof w ? e : new w(e)
    });
  }
  async setFullscreen(e) {
    return i('plugin:window|set_fullscreen', { label: this.label, value: e });
  }
  async setSimpleFullscreen(e) {
    return i('plugin:window|set_simple_fullscreen', { label: this.label, value: e });
  }
  async setFocus() {
    return i('plugin:window|set_focus', { label: this.label });
  }
  async setFocusable(e) {
    return i('plugin:window|set_focusable', { label: this.label, value: e });
  }
  async setIcon(e) {
    return i('plugin:window|set_icon', { label: this.label, value: _(e) });
  }
  async setSkipTaskbar(e) {
    return i('plugin:window|set_skip_taskbar', { label: this.label, value: e });
  }
  async setCursorGrab(e) {
    return i('plugin:window|set_cursor_grab', { label: this.label, value: e });
  }
  async setCursorVisible(e) {
    return i('plugin:window|set_cursor_visible', { label: this.label, value: e });
  }
  async setCursorIcon(e) {
    return i('plugin:window|set_cursor_icon', { label: this.label, value: e });
  }
  async setBackgroundColor(e) {
    return i('plugin:window|set_background_color', { color: e });
  }
  async setCursorPosition(e) {
    return i('plugin:window|set_cursor_position', {
      label: this.label,
      value: e instanceof w ? e : new w(e)
    });
  }
  async setIgnoreCursorEvents(e) {
    return i('plugin:window|set_ignore_cursor_events', { label: this.label, value: e });
  }
  async startDragging() {
    return i('plugin:window|start_dragging', { label: this.label });
  }
  async startResizeDragging(e) {
    return i('plugin:window|start_resize_dragging', { label: this.label, value: e });
  }
  async setBadgeCount(e) {
    return i('plugin:window|set_badge_count', { label: this.label, value: e });
  }
  async setBadgeLabel(e) {
    return i('plugin:window|set_badge_label', { label: this.label, value: e });
  }
  async setOverlayIcon(e) {
    return i('plugin:window|set_overlay_icon', { label: this.label, value: e ? _(e) : void 0 });
  }
  async setProgressBar(e) {
    return i('plugin:window|set_progress_bar', { label: this.label, value: e });
  }
  async setVisibleOnAllWorkspaces(e) {
    return i('plugin:window|set_visible_on_all_workspaces', { label: this.label, value: e });
  }
  async setTitleBarStyle(e) {
    return i('plugin:window|set_title_bar_style', { label: this.label, value: e });
  }
  async setTheme(e) {
    return i('plugin:window|set_theme', { label: this.label, value: e });
  }
  async onResized(e) {
    return this.listen(l.WINDOW_RESIZED, (n) => {
      ((n.payload = new h(n.payload)), e(n));
    });
  }
  async onMoved(e) {
    return this.listen(l.WINDOW_MOVED, (n) => {
      ((n.payload = new o(n.payload)), e(n));
    });
  }
  async onCloseRequested(e) {
    return this.listen(l.WINDOW_CLOSE_REQUESTED, async (n) => {
      const r = new F(n);
      (await e(r), r.isPreventDefault() || (await this.destroy()));
    });
  }
  async onDragDropEvent(e) {
    const n = await this.listen(l.DRAG_ENTER, (s) => {
        e({
          ...s,
          payload: { type: 'enter', paths: s.payload.paths, position: new o(s.payload.position) }
        });
      }),
      r = await this.listen(l.DRAG_OVER, (s) => {
        e({ ...s, payload: { type: 'over', position: new o(s.payload.position) } });
      }),
      a = await this.listen(l.DRAG_DROP, (s) => {
        e({
          ...s,
          payload: { type: 'drop', paths: s.payload.paths, position: new o(s.payload.position) }
        });
      }),
      d = await this.listen(l.DRAG_LEAVE, (s) => {
        e({ ...s, payload: { type: 'leave' } });
      });
    return () => {
      (n(), a(), r(), d());
    };
  }
  async onFocusChanged(e) {
    const n = await this.listen(l.WINDOW_FOCUS, (a) => {
        e({ ...a, payload: !0 });
      }),
      r = await this.listen(l.WINDOW_BLUR, (a) => {
        e({ ...a, payload: !1 });
      });
    return () => {
      (n(), r());
    };
  }
  async onScaleChanged(e) {
    return this.listen(l.WINDOW_SCALE_FACTOR_CHANGED, e);
  }
  async onThemeChanged(e) {
    return this.listen(l.WINDOW_THEME_CHANGED, e);
  }
}
var O;
(function (t) {
  ((t.Disabled = 'disabled'), (t.Throttle = 'throttle'), (t.Suspend = 'suspend'));
})(O || (O = {}));
var z;
(function (t) {
  ((t.Default = 'default'), (t.FluentOverlay = 'fluentOverlay'));
})(z || (z = {}));
var A;
(function (t) {
  ((t.AppearanceBased = 'appearanceBased'),
    (t.Light = 'light'),
    (t.Dark = 'dark'),
    (t.MediumLight = 'mediumLight'),
    (t.UltraDark = 'ultraDark'),
    (t.Titlebar = 'titlebar'),
    (t.Selection = 'selection'),
    (t.Menu = 'menu'),
    (t.Popover = 'popover'),
    (t.Sidebar = 'sidebar'),
    (t.HeaderView = 'headerView'),
    (t.Sheet = 'sheet'),
    (t.WindowBackground = 'windowBackground'),
    (t.HudWindow = 'hudWindow'),
    (t.FullScreenUI = 'fullScreenUI'),
    (t.Tooltip = 'tooltip'),
    (t.ContentBackground = 'contentBackground'),
    (t.UnderWindowBackground = 'underWindowBackground'),
    (t.UnderPageBackground = 'underPageBackground'),
    (t.Mica = 'mica'),
    (t.Blur = 'blur'),
    (t.Acrylic = 'acrylic'),
    (t.Tabbed = 'tabbed'),
    (t.TabbedDark = 'tabbedDark'),
    (t.TabbedLight = 'tabbedLight'));
})(A || (A = {}));
var R;
(function (t) {
  ((t.FollowsWindowActiveState = 'followsWindowActiveState'),
    (t.Active = 'active'),
    (t.Inactive = 'inactive'));
})(R || (R = {}));
function B() {
  return new k(T(), window.__TAURI_INTERNALS__.metadata.currentWebview.label, { skip: !0 });
}
async function I() {
  return i('plugin:webview|get_all_webviews').then((t) =>
    t.map((e) => new k(new g(e.windowLabel, { skip: !0 }), e.label, { skip: !0 }))
  );
}
const m = ['tauri://created', 'tauri://error'];
class k {
  constructor(e, n, r) {
    ((this.window = e),
      (this.label = n),
      (this.listeners = Object.create(null)),
      r?.skip ||
        i('plugin:webview|create_webview', { windowLabel: e.label, options: { ...r, label: n } })
          .then(async () => this.emit('tauri://created'))
          .catch(async (a) => this.emit('tauri://error', a)));
  }
  static async getByLabel(e) {
    var n;
    return (n = (await I()).find((r) => r.label === e)) !== null && n !== void 0 ? n : null;
  }
  static getCurrent() {
    return B();
  }
  static async getAll() {
    return I();
  }
  async listen(e, n) {
    return this._handleTauriEvent(e, n)
      ? () => {
          const r = this.listeners[e];
          r.splice(r.indexOf(n), 1);
        }
      : p(e, n, { target: { kind: 'Webview', label: this.label } });
  }
  async once(e, n) {
    return this._handleTauriEvent(e, n)
      ? () => {
          const r = this.listeners[e];
          r.splice(r.indexOf(n), 1);
        }
      : W(e, n, { target: { kind: 'Webview', label: this.label } });
  }
  async emit(e, n) {
    if (m.includes(e)) {
      for (const r of this.listeners[e] || []) r({ event: e, id: -1, payload: n });
      return;
    }
    return x(e, n);
  }
  async emitTo(e, n, r) {
    if (m.includes(n)) {
      for (const a of this.listeners[n] || []) a({ event: n, id: -1, payload: r });
      return;
    }
    return N(e, n, r);
  }
  _handleTauriEvent(e, n) {
    return m.includes(e)
      ? (e in this.listeners ? this.listeners[e].push(n) : (this.listeners[e] = [n]), !0)
      : !1;
  }
  async position() {
    return i('plugin:webview|webview_position', { label: this.label }).then((e) => new o(e));
  }
  async size() {
    return i('plugin:webview|webview_size', { label: this.label }).then((e) => new h(e));
  }
  async close() {
    return i('plugin:webview|webview_close', { label: this.label });
  }
  async setSize(e) {
    return i('plugin:webview|set_webview_size', {
      label: this.label,
      value: e instanceof c ? e : new c(e)
    });
  }
  async setPosition(e) {
    return i('plugin:webview|set_webview_position', {
      label: this.label,
      value: e instanceof w ? e : new w(e)
    });
  }
  async setFocus() {
    return i('plugin:webview|set_webview_focus', { label: this.label });
  }
  async setAutoResize(e) {
    return i('plugin:webview|set_webview_auto_resize', { label: this.label, value: e });
  }
  async hide() {
    return i('plugin:webview|webview_hide', { label: this.label });
  }
  async show() {
    return i('plugin:webview|webview_show', { label: this.label });
  }
  async setZoom(e) {
    return i('plugin:webview|set_webview_zoom', { label: this.label, value: e });
  }
  async reparent(e) {
    return i('plugin:webview|reparent', {
      label: this.label,
      window: typeof e == 'string' ? e : e.label
    });
  }
  async clearAllBrowsingData() {
    return i('plugin:webview|clear_all_browsing_data');
  }
  async setBackgroundColor(e) {
    return i('plugin:webview|set_webview_background_color', { color: e });
  }
  async onDragDropEvent(e) {
    const n = await this.listen(l.DRAG_ENTER, (s) => {
        e({
          ...s,
          payload: { type: 'enter', paths: s.payload.paths, position: new o(s.payload.position) }
        });
      }),
      r = await this.listen(l.DRAG_OVER, (s) => {
        e({ ...s, payload: { type: 'over', position: new o(s.payload.position) } });
      }),
      a = await this.listen(l.DRAG_DROP, (s) => {
        e({
          ...s,
          payload: { type: 'drop', paths: s.payload.paths, position: new o(s.payload.position) }
        });
      }),
      d = await this.listen(l.DRAG_LEAVE, (s) => {
        e({ ...s, payload: { type: 'leave' } });
      });
    return () => {
      (n(), a(), r(), d());
    };
  }
}
function V() {
  const t = B();
  return new y(t.label, { skip: !0 });
}
async function L() {
  return i('plugin:window|get_all_windows').then((t) => t.map((e) => new y(e, { skip: !0 })));
}
class y {
  constructor(e, n = {}) {
    var r;
    ((this.label = e),
      (this.listeners = Object.create(null)),
      n?.skip ||
        i('plugin:webview|create_webview_window', {
          options: {
            ...n,
            parent:
              typeof n.parent == 'string'
                ? n.parent
                : (r = n.parent) === null || r === void 0
                  ? void 0
                  : r.label,
            label: e
          }
        })
          .then(async () => this.emit('tauri://created'))
          .catch(async (a) => this.emit('tauri://error', a)));
  }
  static async getByLabel(e) {
    var n;
    const r = (n = (await L()).find((a) => a.label === e)) !== null && n !== void 0 ? n : null;
    return r ? new y(r.label, { skip: !0 }) : null;
  }
  static getCurrent() {
    return V();
  }
  static async getAll() {
    return L();
  }
  async listen(e, n) {
    return this._handleTauriEvent(e, n)
      ? () => {
          const r = this.listeners[e];
          r.splice(r.indexOf(n), 1);
        }
      : p(e, n, { target: { kind: 'WebviewWindow', label: this.label } });
  }
  async once(e, n) {
    return this._handleTauriEvent(e, n)
      ? () => {
          const r = this.listeners[e];
          r.splice(r.indexOf(n), 1);
        }
      : W(e, n, { target: { kind: 'WebviewWindow', label: this.label } });
  }
  async setBackgroundColor(e) {
    return i('plugin:window|set_background_color', { color: e }).then(() =>
      i('plugin:webview|set_webview_background_color', { color: e })
    );
  }
}
U(y, [g, k]);
function U(t, e) {
  (Array.isArray(e) ? e : [e]).forEach((n) => {
    Object.getOwnPropertyNames(n.prototype).forEach((r) => {
      var a;
      (typeof t.prototype == 'object' && t.prototype && r in t.prototype) ||
        Object.defineProperty(
          t.prototype,
          r,
          (a = Object.getOwnPropertyDescriptor(n.prototype, r)) !== null && a !== void 0
            ? a
            : Object.create(null)
        );
    });
  });
}
const j = {
  async isDbInitialized() {
    return await i('is_db_initialized');
  },
  async getManufacturerById(t) {
    try {
      return { status: 'ok', data: await i('get_manufacturer_by_id', { manufacturerId: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async getRailwayModelById(t) {
    try {
      return { status: 'ok', data: await i('get_railway_model_by_id', { railwayModelId: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async getRailwayModelsByIds(t) {
    try {
      return { status: 'ok', data: await i('get_railway_models_by_ids', { railwayModelIds: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async getRailwayCompanyById(t) {
    try {
      return { status: 'ok', data: await i('get_railway_company_by_id', { railwayCompanyId: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async createRailwayModel(t) {
    try {
      return { status: 'ok', data: await i('create_railway_model', { input: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async getCollection() {
    try {
      return { status: 'ok', data: await i('get_collection') };
    } catch (t) {
      if (t instanceof Error) throw t;
      return { status: 'error', error: t };
    }
  },
  async getDepot() {
    try {
      return { status: 'ok', data: await i('get_depot') };
    } catch (t) {
      if (t instanceof Error) throw t;
      return { status: 'error', error: t };
    }
  },
  async listCollectionItems(t) {
    try {
      return { status: 'ok', data: await i('list_collection_items', { search: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async createCollectionItem(t) {
    try {
      return { status: 'ok', data: await i('create_collection_item', { input: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async updateCollectionItem(t) {
    try {
      return { status: 'ok', data: await i('update_collection_item', { input: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async deleteCollectionItem(t) {
    try {
      return { status: 'ok', data: await i('delete_collection_item', { id: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async getWishlists() {
    try {
      return { status: 'ok', data: await i('get_wishlists') };
    } catch (t) {
      if (t instanceof Error) throw t;
      return { status: 'error', error: t };
    }
  },
  async getWishlistById(t) {
    try {
      return { status: 'ok', data: await i('get_wishlist_by_id', { id: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async createWishlist(t) {
    try {
      return { status: 'ok', data: await i('create_wishlist', { input: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async renameWishlist(t) {
    try {
      return { status: 'ok', data: await i('rename_wishlist', { input: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async deleteWishlist(t) {
    try {
      return { status: 'ok', data: await i('delete_wishlist', { id: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async setDefaultWishlist(t) {
    try {
      return { status: 'ok', data: await i('set_default_wishlist', { id: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async addToWishlist(t) {
    try {
      return { status: 'ok', data: await i('add_to_wishlist', { input: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async removeFromWishlist(t) {
    try {
      return { status: 'ok', data: await i('remove_from_wishlist', { itemId: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async moveItemToList(t) {
    try {
      return { status: 'ok', data: await i('move_item_to_list', { input: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async getMaintenanceDashboard() {
    try {
      return { status: 'ok', data: await i('get_maintenance_dashboard') };
    } catch (t) {
      if (t instanceof Error) throw t;
      return { status: 'error', error: t };
    }
  },
  async addMaintenanceRecord(t) {
    try {
      return { status: 'ok', data: await i('add_maintenance_record', { input: t }) };
    } catch (e) {
      if (e instanceof Error) throw e;
      return { status: 'error', error: e };
    }
  },
  async dashboardSummary() {
    try {
      return { status: 'ok', data: await i('dashboard_summary') };
    } catch (t) {
      if (t instanceof Error) throw t;
      return { status: 'error', error: t };
    }
  },
  async getAppVersion() {
    return await i('get_app_version');
  }
};
export { j as commands };
