let btn_contacts = document.getElementById("add_contact");

btn_contacts.addEventListener("click", function () {
  console.log("adding contact");

  let uid1 = getCookie("userid");
  let uid2 = prompt("Enter user ID of your contact");
  const createChat = {
    type: "CreateChat",
    chatname: "My first chat",
    userids: [uid1, uid2]
  };

    chat.send(createChat);
});