app:
	trunk build ./telescope-web/index.html --dist=./static/app --public-url app
	subo build . --native