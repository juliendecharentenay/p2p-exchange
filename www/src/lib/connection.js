const { SignalingChannel } = require ('./signalingchannel.js');

const READY = "ready";
const CONNECTED = "connected";
const CLOSED = "close";

class Connection {
  constructor(peer_id, on_error, on_status_changed, on_message, iceServers = null) {
    // Initialise iceServers - if not provided
    if (iceServers === null) {
      iceServers = [ { urls: "stun:stun.services.mozilla.com:3478"} ];
    }

    // Assign callbacks
    this.on_error = on_error;
    this.on_status_changed = on_status_changed;
    this.on_message = on_message;

    // Handle log
    this.p_log = (e) => { console.log(e); };

    // Internal variables
    this.channel = null;

    // Declare peer connection
    this.connection = new RTCPeerConnection({iceServers});

    // Declare signaling channel
    this.signaler = new SignalingChannel(this.on_error, peer_id)
          .on_ready(() => {this.on_status_changed(READY);});

    /**********************************************************
     * Peer connection process
     */
    let making_offer = false;
    let send_icecandidates = false;
    let icecandidates = [];

    // Perfect negotiation pattern 
    // see https://developer.mozilla.org/en-US/docs/Web/API/WebRTC_API/Perfect_negotiation
    this.signaler.on_message(async ({ description, candidate }) => {
      try {
        if (description) {
          this.log(`Received description: ${description}`);
          const offer_collision = (description.type === "offer" && (making_offer || this.connection.signalingState !== "stable"));
          const ignore_offer = !this.signaler.is_polite() && offer_collision;
          if (ignore_offer) { icecandidates = []; return; }

          send_icecandidates = true;
          await this.connection.setRemoteDescription(description);
          if (description.type === "offer") {
            await this.connection.setLocalDescription();
            this.signaler.send({ description: this.connection.localDescription }).catch((e) => {this.handle_error(e);});
          }
          icecandidates.forEach((candidate) => { this.signaler.send({candidate}).catch((e) => {this.handle_error(e);});});

        } else if (candidate) {
          this.log(`Received candidate: ${candidate}`);
          await this.connection.addIceCandidate(candidate);
        }
      } catch (e) {
        this.handle_error(e);
      }
    });

    // Configure peer connection
    this.connection.ondatachannel = (evt) => { this.set_channel(evt.channel); };
    this.connection.onnegotiationneeded = () => {
      this.log("Connection: onnegotiationneeded");
      making_offer = true;
      this.connection.setLocalDescription()
      .then(() => {
        making_offer = false;
        return this.signaler.send({ description: this.connection.localDescription });
      })
      .catch((e) => {this.handle_error(e);});
    };
    this.connection.onicecandidate = ({candidate}) => { 
      if (send_icecandidates) {
        this.signaler.send({candidate}).catch((e) => {this.handle_error(e);});
      } else {
        icecandidates.push(candidate);
      }
    };
    this.connection.onconnectionstatechange = () => {
      console.log(`Connection state: ${this.connection.connectionState}`);
      if (this.connection.connectionState === "closed") {
        this.on_status_changed(CLOSED);
      }
    };
    this.connection.oniceconnectionstatechange = () => {
      console.log("Ice connection state", this.connection.iceConnectionState);
      if (this.connection.iceConnectionState === "closed") {
        this.on_status_changed(CLOSED);
      }
    };

    this.signaler.start()
    .catch((e) => {this.handle_error(e);});
  }

  set_channel(channel) {
    this.log("Assign channel");
    this.channel = channel;
    this.channel.onopen = () => { 
      this.log("Channel is open. Signaler stopped!"); 
      this.on_status_changed(CONNECTED); 
      this.signaler.stop(); 
    };
    this.channel.onclose = () => { this.log("Channel is closed"); this.on_status_changed(CLOSED); };
    this.channel.onmessage = (evt) => { this.on_message(evt.data); };
  }

  call() { this.set_channel(this.connection.createDataChannel("sendChannel")); return this; }
  answer() { return this; }

  id() {
    return this.signaler.get_id();
  }

  send(message) {
    return this.channel.send(message);
  }

  close() {
    this.channel.close();
    this.connection.close();
    return this;
  }

  handle_error(e) {
    this.signaler.stop(); this.on_error(e);
  }
  log(message) { if (this.p_log !== null) { this.p_log(message); } }
  set_log(f) { this.p_log = f; return this; }
}

export { Connection, READY, CONNECTED, CLOSED };
