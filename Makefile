app:
	trunk build ./telescope-web/index.html --dist=./static
	subo build . --bundle --native