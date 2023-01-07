const { Connection } = require('./connection.js');

class ConnectionBuilder {
  static offer() {
    return new ConnectionBuilder();
  }

  static answer(id) {
    return new ConnectionBuilder(id);
  }

  p_on_error; p_on_status_changed; p_on_message; p_on_close; peer_id; p_ice_servers;
  constructor(peer_id = null) {
    this.p_on_error = (e) => {console.error(e);};
    this.p_on_status_changed = (s) => { console.log(`Status changed to ${s}`); };
    this.p_on_message = (m) => { console.log(`Message received: ${m}`); };
    this.peer_id = peer_id;
    this.p_ice_servers = null;
  }

  on_error(f) { this.p_on_error = f; return this;}
  on_status_changed(f) { this.p_on_status_changed = f; return this;}
  on_message(f) { this.p_on_message = f; return this;}
  ice_servers(v) { this.p_ice_servers = v; return this;}

  make() {
    const connection = new Connection(this.peer_id, this.p_on_error, this.p_on_status_changed, this.p_on_message, this.p_ice_servers);
    return (this.peer_id === null ?  connection.call() : connection.answer());
  }
}

export { ConnectionBuilder };

