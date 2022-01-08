# tun-to-sock
A simple test applicaion for passing network data between two TUNs via a tcp socket

# Building the application

run `cargo build`

# Testing on a single host

Note: This application has only been tested on:
    - Userspace: Linux Mint 20.2 Cinnamon
    - Kernel: 5.4.0-89-generic

Note: This application must be run in privilaged mode

To test the application on a single host, net namespaces are required. For this test configuration, the client will run in the local namespace and the server will run in a seperate net namespace called `server`. We will then run a simple iperf applicion to test connectivity.

To start, please run two instances of this application, one in server mode and then one in client mode:

    - server: `sudo ./target/debug/tun_to_vsock 1`
    - client: `sudo ./target/debug/tun_to_vsock 0`

Once the client and server applications are running, move `tun_server` into a seperate namespace and start an iperf server.

```
sudo ip netns add server
sudo ip link set tun_server netns server
sudo ip netns exec server ip addr add 10.0.0.2/24 dev tun_server
sudo ip netns exec server ip l s tun_server up
sudo ip netns exec server iperf3 -B 10.0.0.2 -s
```

Then connect to the iperf server from within the default namespace by running `iperf3 -B 10.0.0.3 -c 10.0.0.2`