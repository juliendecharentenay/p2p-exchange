class SignalingChannel {
  constructor(on_error, id = null) {
    this.ready = 0;
    this.p_on_error = on_error;
    this.p_on_message = null;
    this.p_on_ready = null;
    this.polite = null;
    this.id = null;
    this.peer_id = id;
    this.messages = {};
    this.listen_interval = null;
    this.listen_interval_step_ms = 3000;
    this.listen_count = 0;
  }

  get_id() { return this.id; }
  is_polite() { return this.polite; }
  on_message(f) { this.p_on_message = f; return this; }
  on_ready(f) { this.p_on_ready = f; return this; }
  stop()  { 
    if (this.listen_interval !== null) { clearInterval(this.listen_interval); this.listen_interval = null; }
  }

  start() {
    return new Promise((resolve, reject) => {
      this.ready = 1;
      const fargs = (this.peer_id === null
        ? { url: '/api/offer',  options: { method: 'POST', body: JSON.stringify({name: "offer"}) } }
        : { url: '/api/answer', options: { method: 'POST', body: JSON.stringify({name: "answer", offer_id: this.peer_id }) } });
      fetch(fargs.url, fargs.options)
        .then((r) => {
          if (! r.ok) { throw new Error(`Error communicating with api. Status: ${r.statusText} [${r.status}]`); }
          return r.json();
        })
        .then((r) => {
          this.id = r.id; this.polite = r.polite; this.ready = 2;
          this.listen();
          if (this.p_on_ready !== null) { this.p_on_ready(); }
        })
        .catch((e) => {this.ready = -1; reject(e);});
    });
  }

  send(data) { 
    return new Promise((resolve, reject) => {
      this.send_impl(data, resolve, reject);
    });
  }

  send_impl(data, resolve, reject) {
    switch (this.ready) {
      case 0:
      case 1:
        setTimeout(() => {this.send_impl(data, resolve, reject);}, 1500);
        break;
      case 2:
        fetch('/api/message', {
          method: 'POST',
          body: JSON.stringify({originator_id: this.id, message: JSON.stringify(data)}),
        })
        .then((r) => {
          if (! r.ok) { throw new Error(`Error when sending data to api. Status ${r.statusText} [${r.status}]`); }
          return r.json();
        })
        .then(resolve)
        .catch(reject);
        break;
      case -1:
        reject(new Error("Connection to signaling server failed..."));
        break;
      default:
        reject(new Error(`Ready status ${this.ready} is not supported`));
    }
  }

  listen() {
    this.listen_interval = setInterval(() => {
        if (this.peer_id === null) {
          // Looking for peer
          fetch('/api/answer?' + new URLSearchParams({offer_id: this.id}))
          .then((r) => {
            if (! r.ok) { throw new Error(`Error when retrieving answer. Status: ${r.statusText} [${r.status}]`); }
            return r.json();
          })
          .then((e) => {
            if (e.length > 0) { this.peer_id = e[0].id;}
          })
          .catch((e) => { this.stop(); this.p_on_error(e); });

        } else {
          // Looking for peer messages
          fetch('/api/message?' + new URLSearchParams({originator_id: this.peer_id}))
          .then((r) => {
            if (! r.ok) { throw new Error(`Error when retrieving messages. Status: ${r.statusText} [${r.status}]`); }
            return r.json();
          })
          .then((e) => {
            e.forEach(({id, message}) => {
              if (this.p_on_message !== null) {
                if (! (id in this.messages)) {
                  this.messages[id] = true;
                  this.p_on_message(JSON.parse(message));
                }
              }
            });
          })
          .catch((e) => { this.stop(); this.p_on_error(e); });
        }
        this.listen_count += 1;
        if (this.listen_count * this.listen_interval_step_ms > 5 * 60 * 1000) { // Have been listening for 5 minutes
          this.stop(); this.p_on_error("5 minutes without response. Connection dropped");
        }
      },
      this.listen_interval_step_ms);
  }
}

export { SignalingChannel };
