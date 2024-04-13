run: asprite_export
	cargo run -j 6 --release

build: asprite_export
	cargo build -j 6 --release

test:
	cargo test -j 6

asprite_export: 
	make -C assets/aseprite all
	mkdir -p target
	cp assets/aseprite/atlas.json target/
	cp assets/aseprite/atlas.png target/

clean:
	rm -f assets/atlas*
