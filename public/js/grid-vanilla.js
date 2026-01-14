var __defProp = Object.defineProperty;
var __publicField = (obj, key, value) => {
  if (typeof key !== "symbol")
    key += "";
  if (key in obj)
    return __defProp(obj, key, {enumerable: true, configurable: true, writable: true, value});
  return obj[key] = value;
};
function B(p, r, e) {
  if (p.length > 0) {
    if (p[0].target === r)
      return p[0].contentRect.width;
    if (p.length === 2 && p[1].target === r)
      return p[1].contentRect.width;
  }
  return e;
}
class x {
  constructor(r) {
    __publicField(this, "gap", -1);
    __publicField(this, "frameWidth", -1);
    __publicField(this, "containerWidth", -1);
    __publicField(this, "columnsCount", -1);
    __publicField(this, "containerAspectRatio", -1);
    __publicField(this, "framesPositionsMap", /* @__PURE__ */ new WeakMap());
    __publicField(this, "resizeObserver");
    __publicField(this, "mutationObserver");
    __publicField(this, "marker");
    this.container = r;
    const e = getComputedStyle(r), a = document.createElement("div");
    r.append(a);
    const l = new ResizeObserver((s) => {
      const t = B(s, r, this.containerWidth), m = B(s, a, this.frameWidth);
      let i = this.gap;
      if ((i === -1 || m !== this.frameWidth) && (i = parseFloat(e.gap), isNaN(i) && (i = 0)), i === this.gap && t === this.containerWidth && m === this.frameWidth)
        return;
      this.containerWidth = t, this.frameWidth = m;
      const f = Math.round((t + i) / (m + i));
      if (this.columnsCount === f && i === this.gap) {
        n(), this.resizeHeight(), o();
        return;
      }
      this.gap = i, this.columnsCount = f, n(), this.reflow(), o();
    }), h = new MutationObserver(() => {
      n(), r.append(a), this.columnsCount > 0 && this.reflow(true), o();
    }), o = () => {
      h.observe(r, {
        childList: true,
        attributeFilter: ["style"],
        subtree: true
      });
    }, n = () => {
      h.disconnect();
    };
    this.marker = a, this.mutationObserver = h, this.resizeObserver = l, l.observe(r), l.observe(a), o();
  }
  resizeHeight() {
    const {
      container: r,
      containerAspectRatio: e
    } = this;
    e !== -1 ? r.style.height = `${this.containerWidth * e}px` : r.style.removeProperty("height");
  }
  getFrameAspectRatio(r) {
    const e = parseFloat(r.style.getPropertyValue("--width"));
    return parseFloat(r.style.getPropertyValue("--height")) / e;
  }
  getFramePosition(r, e, a) {
    const {
      gap: l,
      columnsCount: h,
      frameWidth: o
    } = this, n = this.getFrameAspectRatio(r), s = n * o, t = a + s + (e >= h ? l : 0);
    return {
      aspectRatio: n,
      realIndex: e,
      virtualIndex: e,
      height: s,
      realBottom: t,
      virtualBottom: t,
      width: o
    };
  }
  getFramePositionAndCache(r, e, a) {
    const l = this.getFramePosition(r, e, a);
    return this.framesPositionsMap.set(r, l), l;
  }
  getCachedScaledFramePosition(r, e, a) {
    const {frameWidth: l} = this, h = this.getFrameAspectRatio(r), o = this.framesPositionsMap.get(r);
    if (!o || o.realIndex !== e || o.virtualIndex >= a || o.aspectRatio !== h)
      return null;
    const n = l / o.width;
    return o.height *= n, o.realBottom *= n, o.virtualBottom *= n, o.width = l, o;
  }
  destroy() {
    const {
      resizeObserver: r,
      mutationObserver: e,
      marker: a,
      container: l,
      framesPositionsMap: h
    } = this;
    r.disconnect(), e.disconnect(), a.remove();
    const o = l.children;
    l.style.removeProperty("height");
    for (let n = 0, s, t = o.length; n < t; n++)
      s = o[n], s.style.removeProperty("transform"), s.style.removeProperty("order"), h.delete(s);
  }
}
class M extends x {
  constructor() {
    super(...arguments);
    __publicField(this, "bufferA", []);
    __publicField(this, "bufferB", []);
  }
  balanceRow(r, e, a) {
    const {
      columnsCount: l,
      bufferA: h,
      bufferB: o
    } = this, n = a - e + 1;
    h.length = n, o.length = n;
    for (let s = 0, t = e; t <= a; s++, t++)
      h[s] = r[t - l], o[s] = r[t];
    h.sort((s, t) => t.virtualBottom - s.virtualBottom), o.sort((s, t) => s.height - t.height);
    for (let s = 0, t = e, m = 0; t <= a; s++, t++)
      m = h[s].virtualIndex + l, r[m] = o[s], o[s].virtualIndex = m;
  }
  reflow(r = false) {
    const {columnsCount: e} = this;
    if (r && e === 1)
      return;
    const {
      container: a,
      marker: l
    } = this, h = a.children, o = h.length - 1, n = o - 1, s = Array(o);
    let t = -1, m = r;
    l.style.order = String(o);
    for (let i = 0, f, u = 0, g = 0, c; i < o; i++) {
      if (f = h[i], e === 1) {
        f.style.removeProperty("transform"), f.style.removeProperty("order");
        continue;
      }
      if (i % e === 0 && (g = u), m) {
        const v = this.getCachedScaledFramePosition(f, i, o);
        if (v)
          c = v, s[c.virtualIndex] = c;
        else {
          m = false;
          const y = i - i % e;
          for (let d = y, P; d < i; d++)
            P = h[d], c = this.getFramePositionAndCache(P, d, g), s[d] = c;
        }
      }
      if (m)
        u = Math.max(u, c.realBottom), t = Math.max(t, c.virtualBottom);
      else if (c = this.getFramePositionAndCache(f, i, g), s[i] = c, i >= e && ((i + 1) % e === 0 || i === n)) {
        const v = i - i % e, y = i;
        this.balanceRow(s, v, y);
        for (let d = v; d <= y; d++) {
          c = s[d], f = h[c.realIndex], f.style.order = String(c.virtualIndex);
          const P = s[d - e], b = g - P.virtualBottom;
          if (b !== 0) {
            const R = b * 100 / c.height * -1;
            f.style.transform = `translateY(${R}%)`, c.virtualBottom -= b;
          } else
            f.style.removeProperty("transform");
          u = Math.max(u, c.realBottom), t = Math.max(t, c.virtualBottom);
        }
      } else
        i < e && (f.style.order = String(i), f.style.removeProperty("transform"), f.style.removeProperty("order"), u = Math.max(u, c.realBottom), t = Math.max(t, c.virtualBottom));
    }
    t === -1 ? (a.style.removeProperty("height"), this.containerAspectRatio = -1) : (a.style.height = `${t}px`, this.containerAspectRatio = t / this.containerWidth);
  }
  destroy() {
    super.destroy(), this.bufferA.length = 0, this.bufferB.length = 0;
  }
}
class C extends x {
  reflow(r = false) {
    const {columnsCount: e} = this;
    if (r && e === 1)
      return;
    const {
      container: a,
      framesPositionsMap: l
    } = this, h = a.children, o = h.length - 1;
    let n = -1, s = r;
    for (let t = 0, m, i = 0, f = 0, u; t < o; t++) {
      if (m = h[t], e === 1) {
        m.style.removeProperty("transform");
        continue;
      }
      if (t % e === 0 && (f = i), s) {
        const g = this.getCachedScaledFramePosition(m, t, o);
        g ? u = g : s = false;
      }
      if (!s)
        if (u = this.getFramePositionAndCache(m, t, f), t >= e) {
          const g = l.get(h[t - e]), c = f - g.virtualBottom;
          if (c !== 0) {
            const v = c * 100 / u.height * -1;
            m.style.transform = `translateY(${v}%)`, u.virtualBottom -= c;
          } else
            m.style.removeProperty("transform");
        } else
          m.style.removeProperty("transform");
      i = Math.max(i, u.realBottom), n = Math.max(n, u.virtualBottom);
    }
    n === -1 ? (a.style.removeProperty("height"), this.containerAspectRatio = -1) : (a.style.height = `${n}px`, this.containerAspectRatio = n / this.containerWidth);
  }
}
const w = C;
export {M as BalancedMasonryGrid, x as BaseMasonryGrid, w as MasonryGrid, C as RegularMasonryGrid};
export default null;
