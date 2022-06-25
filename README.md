# Remote debug rust code example

build with `docker build . -t rweb:latest`

run with 
`docker run --name rweb -p 4096:4096 -p3000:3000 -p 4000-4090:4000-4090 --rm --privileged -ti rweb:latest`

attach lldb server `docker exec -ti rweb bash`

in this container, start the lldb server (needs the main listen port 4096 and a range of ports for gdbserver communication 4000-4090 - these need to be mapped in the docker run command above)
```
$ lldb-server platform --listen "*:4096" --server --min-gdbserver-port 4000 --max-gdbserver-port 4090
```

run lldb locally:
```
$ lldb
(lldb) platform select remote-linux
(lldb) platform connect connect://localhost:4096
(lldb) attach 1
```

All threads receive SIGSTOP, you can set breakpoints, then use `continue` to resume the program