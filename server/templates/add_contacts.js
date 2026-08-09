let btn_contacts = document.getElementById("add_contact");

btn_contacts.addEventListener("click", function () {
  console.log("adding contact");

  let uid1 = getCookie("userid");
  let uid2 = prompt("Enter user ID of your contact");
  const createChat = {
    type: "CreateChat",
    data: {
      chatname: "chat of " + uid1 + " and " + uid2, // need to work on naming the chats
      userids: [Number(uid1), Number(uid2)]
    }
  };

    chat.send(createChat);
});