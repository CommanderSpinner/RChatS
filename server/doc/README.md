# Server
this server handles requests and connects to the db.

### folders
in server_data the config files are stored and for the postgres db a simple config for debugging. in produciton envirnoment you should set up a proper postgresql server with a secure password.
in chat_data the data of the useres such as photos, audio, and videos are stored. every user gets a folder and sub folders for the data they get deleted when the account gets deleted

wip