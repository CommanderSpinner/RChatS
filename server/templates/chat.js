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