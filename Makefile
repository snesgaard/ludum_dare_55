ASEPRITE_FILES=$(wildcard assets/aseprite/*.aseprite)
ATLAS_PNG=target/atlas.png
ATLAS_JSON=target/atlas.json

run: build
	cargo run -j 6 --release

build:
	cargo build -j 6 --release

test:
	cargo test -j 6

asprite_export: $(ATLAS_JSON)

$(ATLAS_JSON) $(ATLAS_PNG): $(ASEPRITE_FILES)
	aseprite -b $< --sheet $(ATLAS_PNG) --data $(ATLAS_JSON) \
	           --list-slices  --trim --inner-padding 1 --format json-array\
	           --list-tags --sheet-pack