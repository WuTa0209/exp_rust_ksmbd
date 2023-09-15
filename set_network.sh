sudo ip link add name br0 type bridge
sudo ip addr add 192.168.100.1/24 brd + dev br0
sudo ip link set br0 up
sudo sysctl -w net.ipv4.ip_forward=1
sudo iptables -t filter -P FORWARD ACCEPT
sudo iptables -t nat -A POSTROUTING -o enx3a3af171fa88 -j MASQUERADE
sudo iptables -t nat -A POSTROUTING -o wlp4s0 -j MASQUERADE
