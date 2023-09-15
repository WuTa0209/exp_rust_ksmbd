if [ -f "../output/images/start-qemu.sh" ]; then
    # Delete the last line of foo.sh
    sed -i '$d' ../output/images/start-qemu.sh

    # Append "hello world" to the end of foo.sh
    echo 'exec qemu-system-x86_64 -M pc -m 4G -kernel bzImage -drive file=rootfs.ext2,if=virtio,format=raw -append "rootwait root=/dev/vda console=tty1 console=ttyS0 nokaslr" -nic bridge,br=br0,model=virtio-net-pci,mac=02:76:7d:d7:1e:3f ${EXTRA_ARGS} "$@" -s -S' >> ../output/images/start-qemu.sh

    echo "Last line deleted and commmand added to foo.sh"
else
    echo "File 'foo.sh' does not exist."
fi
