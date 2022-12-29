class ConnectionBuilder {
  static offer() {
    return new ConnectionBuilder();
  }

  static answer(id) {
    return new ConnectionBuilder(id);
  }

  p_on_error; p_on_status_changed; p_on_message; p_on_close; peer_id;
  constructor(peer_id = null) {
    this.p_on_error = (e) => {console.error(e);};
    this.p_on_status_changed = (s) => { console.log(`Status changed to ${s}`); };
    this.p_on_message = (m) => { console.log(`Message received: ${m}`); };
    this.p_on_close = () => { console.log('Connection closed'); };
    this.peer_id = peer_id;
  }

  on_error(f) { this.p_on_error = f; return this;}
  on_status_changed(f) { this.p_on_status_changed = f; return this;}
  on_message(f) { this.on_message = f; return this;}
  on_close(f) { this.on_close = f; return this;}

  make() {
    return new Connection(this.peer_id, this.p_on_error, this.p_on_status_changed, this.p_on_message, this.p_on_close).start();
  }
}

export { ConnectionBuilder };

