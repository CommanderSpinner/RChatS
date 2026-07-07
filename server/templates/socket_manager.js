class ChatSocket {
  constructor(url) {
    const protocol = window.location.protocol === "https:" ? "wss" : "ws";
    this.socket = new WebSocket(`${protocol}://${window.location.host}/ws`);
    this.queue = [];

    this.socket.addEventListener("open", () => {
      console.log("Connected");

      // Send anything that was waiting
      while (this.queue.length > 0) {
        this.socket.send(this.queue.shift());
      }
    });

    this.socket.addEventListener("close", () => {
      console.log("Disconnected");
    });
  }

  #generateId() {
    return Date.now().toString(36) + Math.random().toString(36).substring(2);
   }

  send(data) {
    let id = this.#generateId()
    const message = JSON.stringify({
      id: id,
      //timestamp: Date.now(),
      ...data
    });

    if (this.socket.readyState === WebSocket.OPEN) {
      this.socket.send(message);
    } else {
      this.queue.push(message);
    }
  }
}

const chat = new ChatSocket();