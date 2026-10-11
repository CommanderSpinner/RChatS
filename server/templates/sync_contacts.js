
function sync_contacts() {
    let uid = getCookie("userid");

    const get_contacts = {
        type: "GetContacts",
        data: {
            userid: Number(uid)
        }
    };

    chat.send(get_contacts);
}