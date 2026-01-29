import stats from "./server-stats.js";

console.log("+++ admin panel started");
const tpl = `
<server-stats></server-stats>

<div><input type="file" 
    class="filepond"
    name="filepond" 
    multiple 
    data-allow-reorder="true"
    data-max-file-size="10MB"
    data-max-files="200"></div>
`;
class AdminPanel extends HTMLElement {
  data = {};
  name = "";

  connectedCallback() {
    this.data = window.__load_data;
    this.name = this.data.name;
    console.log("AdminPanel connected ...", this.data);
    this.render();
    FilePond.create(this.querySelector("input"), {
      credits: null,
      dropOnPage: true,
      dropOnElement: false,
      server: {
        url: "./a/" + this.name, // api + gallery?.name + '/',
        process: {
          url: "",
          withCredentials: true,
        },
        fetch: null,
        revert: null,
      },
    });
  }

  render() {
    this.innerHTML = tpl;
  }
}

customElements.define("admin-panel", AdminPanel);
export default AdminPanel;
