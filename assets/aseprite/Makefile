sprites=$(wildcard *.aseprite)
jsons=$(addprefix ./build/, $(notdir $(sprites:.aseprite=.json)))
atlas_im=./atlas.png
atlas_json=./atlas.json
raw_atlas_json=./build/atlas.json
info=./index.lua
aseprite=aseprite

all: create_build_dirs $(jsons) $(atlas_im) $(atlas_json)

debug:
	@echo $(jsons)

$(atlas_json): $(raw_atlas_json) $(jsons)
	python ../../join_aseprite_jsons.py $@ $^

$(atlas_im) $(raw_atlas_json): $(sprites)
	@$(aseprite) -b $^ --sheet $(atlas_im) --data $(raw_atlas_json) \
	           --list-slices  --trim --inner-padding 1 --format json-array\
	           --list-tags --sheet-pack

build/%.json: %.aseprite
	@$(aseprite) -b $< --data $@ \
	           --list-slices  --trim --inner-padding 1 --format json-array\
	           --list-tags --sheet-pack

create_build_dirs:
	@mkdir -p build

clean:
	@rm -rf build
	@rm -f $(info)
	@rm -f $(atlas_im)
	@rm -f $(atlas_json)
