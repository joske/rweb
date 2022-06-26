# Remote debug rust code example

build with `docker build . -t rweb:latest`

run with 
`docker run --name rweb -p3000:3000 -p 4000-4096:4000-4096 --rm --privileged -d rweb:latest`

attach debug server in same container `docker exec -ti rweb bash`

## GDB

in this container, start the gdb server
```
root@e46efd6209cb:/app# gdbserver 0.0.0.0:4096 --attach <pid of target program>
```

In another container (can be any image that has gdb - here using same image), mount the source code via a volume and attach to the rweb process via the gdb server (needs to be a linux container as macos does not have gdb natively and also because the binary we're debugging is a **linux** binary). This needs `--net=host` to be able to access the forwarded port 

```
user@host$ docker run --net=host -v $(pwd):/app/src --rm --privileged -ti --name debug --entrypoint bash rweb:latest
```

now connect to remote gdbserver
```
root@docker-desktop:/app# cd /app/src/
root@docker-desktop:/app# gdb
(gdb) target remote localhost:4096
```

All threads receive SIGSTOP, you can set breakpoints, then use `continue` to resume the program

## LLDB (for some reason remote debugger does not find the sources, while this works on gdb)
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