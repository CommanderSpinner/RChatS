# RCHATS
Simple rust server and client for chatting. The project is build in 2 parts.
- Server
- Client

They are 2 seperate applications.

## Server
to build the server run the powerhsell script in the directory(default build is linux change to your prefered platform and give it the "-package" option to get a zip file of the project).
The server uses a postgresql db and potentially later also the option to use sqlite. The server also has a web interfaces. configure the server via the config file with the db credentials and the port where the web interface will run.
To host a server you can use the provided docker image that is build automaticly on every push to the repo and you can also modify the docker-compose.yml in the projects root to set up the postgres server 

## Client




<br>
<br>
<br>
<span style="color: red;">the entire project is Work in progress</span>