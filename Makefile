build-windows:
	CROSS_CONTAINER_UID=1000 cross build --target x86_64-pc-windows-gnu

build-windows-release:
	CROSS_CONTAINER_UID=1000 cross build --release --target x86_64-pc-windows-gnu