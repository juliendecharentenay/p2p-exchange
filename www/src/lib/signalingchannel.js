class SignalingChannel {
  constructor(name, on_error, id = null) {
    this.ready = 0;
    this.on_error = on_error;
    this.on_message = null;
    this.on_ready = null;
    this.polite = null;
    this.id = null;
    this.peer_id = id;
    this.name = name;
  }

  is_ready() { return this.ready; }
  get_id() { return this.id; }
  is_polite() { return this.polite; }

  send(data) { 
    return new Promise((resolve, reject) => {
      if (this.ready === 0) {
        this.ready = 1;
        const fargs = (this.peer_id === null
               ? { url: '/api/offer',  options: { method: 'POST', body: JSON.stringify({name: this.name}) } }
               : { url: '/api/answer', options: { method: 'POST', body: JSON.stringify({name: this.name, offer_id: this.peer_id }) } });
        fetch(fargs.url, fargs.options)
              .then((r) => {
                if (! r.ok) { throw new Error(`Error communicating with api. Status: ${r.statusText} [${r.status}]`); }
                return r.json();
              })
              .then((r) => {
                this.id = r.id; this.polite = r.polite; this.ready = 2;
                this.listen();
                if (this.on_ready !== null) { this.on_ready(); }
              })
              .catch((e) => {this.ready = -1; reject(e);});
      }
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
        throw new Error("Connection to signaling server failed...");
        break;
      default:
        throw new Error(`Ready status ${this.ready} is not supported`);
    }
  }

  listen() {
    this.listen_interval = setInterval(() => {
        if (this.peer_id === null) {
          // Looking for peer response
          fetch('/api/answer?' + new URLSearchParams({offer_id: this.id}))
          .then((r) => {
            if (r.ok) {
              return r.json();
            } else {
              return [];
            }
          })
          .then((e) => {
            if (e.length > 0) { this.peer_id = e[0].id;}
          })
          .catch((e) => { this.stop(); this.on_error(e); });

        } else {
          // Looking for peer messages
          fetch('/api/message?' + new URLSearchParams({originator_id: this.peer_id}))
          .then((r) => r.json())
          .then((e) => {
            e.forEach((item) => {
              if (this.on_message !== null) {
                this.on_message(item.id, JSON.parse(item.message));
              }
            });
          })
          .catch((e) => { this.stop(); this.on_error(e); });

        }
      },
      3000);
  }
  stop_listen() {
    clearInterval(this.listen_interval);
  }
  
  set_on_message(f) { this.on_message = f; return this; }
  set_on_ready(f) { this.on_ready = f; return this; }
  stop()  { console.log("Signaler stopped"); this.stop_listen(); }
}

export { SignalingChannel };
