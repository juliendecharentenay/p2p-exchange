class Connection {
  constructor(config) {
    this.send_icecandidates = false;
    this.icecandidates = [];

    this.on_error = ('on_error' in config ? config.on_error : (e) => {console.error(e);});
    this.on_connected = ('on_connected' in config ? config.on_connected : () => {console.log('connected...');});
    this.on_message = ('on_message' in config ? config.on_message: (m) => {console.log(`Received message ${m}`);});
    this.p_log = ('log' in config ? config.log : (l) => {console.log(l);});
    this.signaler = new SignalingChannel(config.name, this.on_error, ('id' in config ? config.id : null))
          .set_on_ready('on_ready' in config ? config.on_ready : () => {this.log("Signaling channel is ready;");});

    this.connection = new RTCPeerConnection({
      iceServers: [{ urls: "stun:stun.services.mozilla.com:3478"}],
    });
    this.channel = null;
    this.channel_status = 0;
    this.connection.ondatachannel = (evt) => { this.log(`ondatachannel: ${JSON.stringify(evt)}`); this.set_channel(evt.channel); };

    let making_offer = false;
    this.connection.onnegotiationneeded = () => {
      making_offer = true;
      this.connection.setLocalDescription()
      .then(() => {
        this.log(`localDescription: ${JSON.stringify(this.connection.localDescription)}`);
        making_offer = false;
        return this.signaler.send({ description: this.connection.localDescription });
      })
      .catch((e) => {
        this.signaler.stop();
        this.on_error(e);
      });
    };

    this.connection.onicecandidate = ({candidate}) => { 
      this.log(`Candidate: ${JSON.stringify(candidate)}`); 
      if (this.send_icecandidates) {
        this.signaler.send({candidate}).catch((e) => {this.signaler.stop(); this.on_error(e);}); 
      } else {
        this.icecandidates.push(candidate);
      }
    };

    let ignore_offer = false;
    let messages = {};
    this.signaler.set_on_message(async (id, { description, candidate }) => {
      try {
        if (! (id in messages)) {
          this.log(`on_message ${id}: ${JSON.stringify(description)} and ${JSON.stringify(candidate)}`);
          messages[id] = true;
          if (description) {
            const offer_collision = (description.type === "offer" && (making_offer || this.connection.signalingState !== "stable"));
            ignore_offer = !this.signaler.is_polite() && offer_collision;
            if (ignore_offer) { this.icecandidates = []; return; }

            await this.connection.setRemoteDescription(description);
            if (description.type === "offer") {
              await this.connection.setLocalDescription();
              this.signaler.send({ description: this.connection.localDescription }).catch((e) => {this.signaler.stop(); this.on_error(e);});
            }
            this.icecandidates.forEach((candidate) => { this.signaler.send({candidate}).catch((e) => {this.signaler.stop(); this.on_error(e);}); });
            this.send_icecandidates = true;

          } else if (candidate) {
            try {
              await this.connection.addIceCandidate(candidate);
            } catch(e) {
              if (!ignore_offer) { throw e; }
            }
          }
        }
      } catch (e) {
        this.signaler.stop();
        this.on_error(e);
      }
    });

  }

  set_channel(channel) {
    this.channel = channel;
    this.channel.onopen = () => { this.signaler.stop(); this.log("Channel is open. Signaler stopped!"); this.on_connected(); };
    this.channel.onclose = () => { this.log("Channel is closed"); };
    this.channel.onmessage = (evt) => { this.on_message(evt.data); };
    return this;
  }

  log(message) {
    if (this.p_log !== null) { this.p_log(message); }
  }

  set_log(cb) {
    this.p_log = cb;
    return this;
  }

  start() {
    this.set_channel(this.connection.createDataChannel("sendChannel"));
    return this;
  }

  close() {
    this.channel.close();
    this.connection.close();
    return this;
  }

  send(message) {
    return this.channel.send(message);
  }

  get_id() {
    return this.signaler.get_id();
  }
}

export { Connection, };
