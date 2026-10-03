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
      container.innerHTML = "";

      data.data.forEach(([id, username]) => {
        const btn = document.createElement("button");
        btn.type = "button";
        btn.className = "btn btn-dark border-light text-start text-truncate w-100 d-flex align-items-center gap-2 py-2 px-3";
        btn.dataset.contactId = id;

        // round avatar with the first letter
        const avatar = document.createElement("span");
        avatar.className = "badge rounded-2 bg-secondary text-dark text-uppercase";
        avatar.textContent = username.charAt(0);

        const label = document.createElement("span");
        label.className = "flex-grow-1 text-truncate";
        label.textContent = username;

        btn.append(avatar, label);

        btn.addEventListener("click", () => {
          // reset all contact buttons, then highlight the clicked one
          container.querySelectorAll("button").forEach(b => {
            b.classList.remove("active", "btn-primary");
            b.classList.add("btn-dark");
          });
          btn.classList.remove("btn-dark");
          btn.classList.add("active", "btn-primary");

          this.onContactClick(id, username);
        });

        container.appendChild(btn);
      });
    }
  }

  onContactClick(id, username) {
    console.log("Clicked contact:", id, username);
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