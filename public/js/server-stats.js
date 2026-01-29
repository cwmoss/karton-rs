console.log("+++ server-stats");
const tpl = (s) => `
<div>up ${s.server_uptime}</div><div>mem ${s.physical_mem}</div><div>cache ${s.cache_size}</div>
`;
class ServerStats extends HTMLElement {
  data = {};
  name = "";

  async connectedCallback() {
    let r = await fetch("./stats");
    let d = await r.json();
    this.render(d);
  }

  render(d) {
    this.innerHTML = tpl(d);
  }
}

customElements.define("server-stats", ServerStats);
export default ServerStats;
