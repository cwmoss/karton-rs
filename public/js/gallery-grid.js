import { BalancedMasonryGrid } from "./grid-vanilla.js";
// "https://cdn.skypack.dev/@masonry-grid/vanilla";

let item_tpl = (it, grid) => `<figure class="${ratio(it)}" style="--width: ${
  it.w
}; --height: ${it.h};"><div>
    <a href="${grid.base}/i/big/${grid.path}${it.path}" data-pswp-width="${
      it.w
    }" data-pswp-height="${it.h}"><img src="${grid.base}/i/thumb/${grid.path}${
      it.path
    }" alt="photography" loading="lazy"/></a>
    <figcaption>${it.path} ${it.w} x ${it.h}</figcaption>
</div></figure>`;

function ratio(it) {
  if (it.h > it.w) {
    return "portrait";
  } else if (it.w > it.h) {
    return "landscape";
  } else {
    return "square";
  }
}

class GalleryGrid extends HTMLElement {
  data = {};
  name = "";
  base = "";
  path = "";
  connectedCallback() {
    this.data = window.__load_data;
    this.name = this.data.name;
    this.path = this.data.path;
    if (this.path) this.path += "/";
    this.mode = this.getAttribute("mode");
    this.base = this.mode == "browse" ? "./b" : "./a";

    console.log("GalleryGrid connected ...", this.data);
    this.render();
  }

  render() {
    let html = this.data.images
      .map((it) => {
        return item_tpl(it, this);
      })
      .join("\n");
    this.innerHTML = html;
    // FlexMasonry.init("gallery-grid");
    new BalancedMasonryGrid(this);
    window.setTimeout(() => {
      this.onLoaded();
    }, 4000);
  }

  onLoaded() {
    this.classList.add("loaded");
  }
}

customElements.define("gallery-grid", GalleryGrid);
