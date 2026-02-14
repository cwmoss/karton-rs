console.log("+++ folders");
const tpl = (it, comp) => `
<li><a href="${comp.base}/${comp.path}${it}">${it}</a></li>
`;
class FolderList extends HTMLElement {
  data = {};
  name = "";
  path = "";
  mode = "";
  up = "";
  connectedCallback() {
    this.data = window.__load_data;
    this.name = this.data.name;
    this.path = this.data.path;
    if (this.path) this.path += "/";

    this.mode = this.getAttribute("mode");
    this.base = this.mode == "browse" ? "./b" : "./a";

    console.log("Folders connected ...", this.data.folders);
    this.render();
  }

  render_up() {
    console.log("render-up", this.path);
    if (!this.path) return "";
    return `<li class="up"><a href="${this.base}/${this.path}../">../</a></li>`;
  }

  render() {
    this.innerHTML = `<ul>${this.render_up()}${this.data.folders.map((f) => tpl(f, this)).join("")}</ul>`;
  }
}

customElements.define("folder-list", FolderList);
export default FolderList;
