image:
	cargo run > image.ppm
	feh --force-aliasing -Z image.ppm

clean:
	cargo clean
	rm -f image.ppm

