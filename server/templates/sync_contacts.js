
function sync_contacts() {
    let uid = getCookie("userid");

    const get_contacts = {
        type: "GetContacts",
        data: {
            userid: uid
        }
    };

    chat.send(get_contacts);
}