let btn_contacts = document.getElementById("add_contact");

btn_contacts.addEventListener("click", function () {
  console.log("adding contact");
  chat.send("dummy string");
});