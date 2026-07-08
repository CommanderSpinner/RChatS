let btn_contacts = document.getElementById("add_contact");

btn_contacts.addEventListener("click", function () {
  console.log("adding contact");

  const createChat = {
    type: "CreateChat",
    chatname: "My first chat",
    userids: [1, 2]
  };

    chat.send(createChat);
});