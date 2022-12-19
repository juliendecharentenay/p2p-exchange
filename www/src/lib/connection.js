
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
      console.log("Sending data", data, this); 
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
                this.id = r.id; this.polite = r.polite; this.ready = 2; console.log("is ready:", this);
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
        .then((r) => r.json())
        .then(resolve)
        .catch(reject);
        break;
      case -1:
        console.error("Connection to signaling server failed...");
        break;
      default:
        throw new Error(`Ready status ${this.ready} is not supported`);
    }
  }

  listen() {
    this.listen_interval = setInterval(() => {
        console.log("Listening...", this);
        if (this.peer_id === null) {
          // Looking for peer response
          fetch('/api/answer?' + new URLSearchParams({offer_id: this.id}))
          .then((r) => {
            console.log("Response: ", r);
            if (r.ok) {
              return r.json();
            } else {
              return [];
            }
          })
          .then((e) => {if (e.length > 0) { this.peer_id = e[0].id;}})
          .catch(this.on_error);

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
          .catch(this.on_error);

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

class Connection {
  constructor(config) {
    this.on_error = ('on_error' in config ? config.on_error : (e) => {console.error(e);});
    this.on_connected = ('on_connected' in config ? config.on_connected : () => {console.log('connected...');});
    this.on_message = ('on_message' in config ? config.on_message: (m) => {console.log(`Received message ${m}`);});
    this.signaler = new SignalingChannel(config.name, this.on_error, ('id' in config ? config.id : null))
          .set_on_ready('on_ready' in config ? config.on_ready : () => {console.log("Signaling channel is ready;");});

    this.connection = new RTCPeerConnection({
      iceServers: [{ urls: "stun:stun.services.mozilla.com:3478"}],
    });
    this.channel = null;
    this.channel_status = 0;
    this.connection.ondatachannel = (evt) => { this.set_channel(evt.channel); };

    let making_offer = false;
    this.connection.onnegotiationneeded = () => {
      making_offer = true;
      this.connection.setLocalDescription()
      .then(() => {
        this.signaler.send({ description: this.connection.localDescription });
        making_offer = false;
      })
      .catch(this.on_error);
    };

    this.connection.onicecandidate = ({candidate}) => { this.signaler.send({candidate}); };

    let ignore_offer = false;
    let messages = {};
    this.signaler.set_on_message(async (id, { description, candidate }) => {
      try {
        if (! (id in messages)) {
          messages[id] = true;
          if (description) {
            const offer_collision = (description.type === "offer" && (making_offer || this.connection.signalingState !== "stable"));
            ignore_offer = !this.signaler.is_polite() && offer_collision;
            if (ignore_offer) { return; }

            await this.connection.setRemoteDescription(description);
            if (description.type === "offer") {
              await this.connection.setLocalDescription();
              this.signaler.send({ description: this.connection.localDescription });
            }
          } else if (candidate) {
            try {
              await this.connection.addIceCandidate(candidate);
            } catch(e) {
              if (!ignore_offer) { throw e; }
            }
          }
        }
      } catch (e) {
        this.on_error(e);
      }
    });

  }

  set_channel(channel) {
    this.channel = channel;
    this.channel.onopen = () => { this.signaler.stop(); console.log("Channel is open. Signaler stopped!"); this.on_connected(); };
    this.channel.onclose = () => { console.log("Channel is closed"); };
    this.channel.onmessage = (evt) => { this.on_message(evt.data); };
  }

  start() {
    this.set_channel(this.connection.createDataChannel("sendChannel"));
  }

  close() {
    this.channel.close();
    this.connection.close();
  }

  send(message) {
    return this.channel.send(message);
  }

  get_id() {
    return this.signaler.get_id();
  }
}

export { 
  Connection,
  SignalingChannel,
};
