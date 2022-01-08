sudo ip netns add server
sudo ip link set tun_server netns server
sudo ip netns exec server ip addr add 10.0.0.2/24 dev tun_server
sudo ip netns exec server ip l s tun_server up
sudo ip netns exec server iperf3 -B 10.0.0.2 -s