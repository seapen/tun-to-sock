# tun-to-sock
A simple stest applicaion for moving network data between TUNs via a sock

# Testing on a single host

To test the applicion on a single host, net namespaces are required. For this test configuration, the server will run in the local namespace and the client will run in a seperate net namespace called `client`.

To setup the server:
    `sudo ./tun_to_vsock 1`

To run the client:
    `ip netns add client`
    `sudo ip netns exec client zsh`
    `sudo ./tun_to_vsock 1`