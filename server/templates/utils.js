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

    this.socket.addEventListener("message", (event) => {
      this.onMessage(event);
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

  onMessage(event) {
    const data = JSON.parse(event.data);


    console.log("Received from server:", data);


    if (data.type === "Contacts") {
      const container = document.getElementById("new_contacts");
      container.innerHTML = ""; // clear old buttons so repeated messages don't duplicate

      data.data.forEach(([id, username]) => {
        const btn = document.createElement("button");
        btn.type = "button";
        btn.className = "list-group-item list-group-item-action";
        btn.dataset.contactId = id;
        btn.textContent = username; // textContent (not innerHTML) avoids XSS from contact names

        btn.addEventListener("click", () => this.onContactClick(id, username));

        container.appendChild(btn);
      });
    }
  }

  onContactClick(id, username) {
    console.log("Clicked contact:", id, name);
    // open chat, load messages, etc.
  }
}



const chat = new ChatSocket();

function getCookie(name) {
  const cookies = document.cookie.split("; ");

  for (const cookie of cookies) {
    const [key, value] = cookie.split("=");
    if (key === name) {
      return decodeURIComponent(value);
    }
  }

  return null;
}

console.log(getCookie("username"));