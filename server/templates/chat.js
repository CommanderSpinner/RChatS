console.log("test")

document.getElementById("chatForm").addEventListener("submit", async function (event) {
    event.preventDefault(); // Prevent page reload

    const input = document.getElementById("messageInput");
    const message = input.value.trim();

    if (!message) return;

    const createMessage = {
        type: "CreateMessage",
        data: {
            username: getCookie("username"),
            plain_password: getCookie("password"),
            to: getCookie("chat_partner"),
            content: document.getElementById("messageInput").value,
        }
    };

    document.getElementById("messageInput").value = "";

    chat.send(createMessage);
});

function chatInsertMessages(messages) {
    const chatBox = document.getElementById("chat-box");
    chatBox.innerHTML = "";

    if (!Array.isArray(messages) || messages.length === 0) {
        const empty = document.createElement("div");
        empty.className = "text-muted";
        empty.textContent = "Start of conversation...";
        chatBox.appendChild(empty);
        return;
    }

    for (const msg of messages) {
        const { mid, cid, uid, url, content, created_at } = msg;

        const line = document.createElement("div");
        line.className = "mb-1 text-white";
        line.textContent = `${uid}: ${content}`;

        chatBox.appendChild(line);
    }

    // scroll automaticly to the bottom
    chatBox.scrollTop = chatBox.scrollHeight;
}