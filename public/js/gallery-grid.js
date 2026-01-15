import { BalancedMasonryGrid } from "./grid-vanilla.js";
// "https://cdn.skypack.dev/@masonry-grid/vanilla";

let item_tpl = (it, name) => `<figure class="${ratio(it)}" style="--width: ${
  it.w
}; --height: ${it.h};">
    <a href="/${name}/big/${it.name}" data-pswp-width="${
  it.w
}" data-pswp-height="${it.h}"><img src="/${name}/thumb/${
  it.name
}" alt="photography" /></a>
    <figcaption>${it.name} ${it.w} x ${it.h}</figcaption>
</figure>`;

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

  connectedCallback() {
    this.data = window.__load_data;
    this.name = this.data.name;
    console.log("GalleryGrid connected", this.data);
    this.render();
  }

  render() {
    let html = this.data.images
      .map((it) => {
        return item_tpl(it, this.name);
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
