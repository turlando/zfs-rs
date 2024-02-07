VDEV_DIR ?= $(realpath vdevs)
VDEV_SIZE ?= 128M
ZPOOL_PREFIX ?= zfsrs-test

default:

.PHONY: mirror
mirror:
	mkdir -p $(VDEV_DIR)
	truncate --size $(VDEV_SIZE) \
		$(VDEV_DIR)/mirror1.img \
		$(VDEV_DIR)/mirror2.img
	sudo zpool create \
		-m none \
		-o ashift=12 \
		-O canmount=off \
		-O checksum=fletcher4 \
		-O compression=zstd \
		-O xattr=sa \
		-O normalization=formD \
		-O atime=off \
		$(ZPOOL_PREFIX)-mirror \
		mirror \
		$(VDEV_DIR)/mirror1.img \
		$(VDEV_DIR)/mirror2.img
