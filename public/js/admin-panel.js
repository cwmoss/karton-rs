console.log("+++ admin panel started");
const tpl = `
<server-stats></server-stats>
<h3>admin</h3>
<div><input type="file"></div>
`
class AdminPanel extends HTMLElement {
    data = {};
    name = "";

    connectedCallback() {
        this.data = window.__load_data;
        this.name = this.data.name;
        console.log("AdminPanel connected ...", this.data);
        this.render();
    }

    render() {
        this.innerHTML = tpl
    }
}

customElements.define("admin-panel", AdminPanel);