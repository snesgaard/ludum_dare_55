import json
import sys
import os


def load_json(path):
    with open(path, 'r') as f:
        return json.load(f)


def fetch_global_index(sprite_json, local_index, frame_to_index):
    frame = sprite_json['frames'][local_index]
    name = frame['filename']
    global_index = frame_to_index[name]
    return global_index


def patch_frame_tag(prefix, sprite_json, frame_to_index):
    tags = sprite_json['meta']['frameTags']

    new_tags = []

    for tag in tags:
        name = "%s/%s" % (prefix, tag["name"])
        from_index = fetch_global_index(sprite_json, tag["from"], frame_to_index)
        to_index = fetch_global_index(sprite_json, tag["to"], frame_to_index)
        direction = tag["direction"]
        new_tags.append({
            "name": name, "to": to_index, "from": from_index,
            "direction": direction
        })

    frames = sprite_json["frames"]
    first_frame = frames[0]
    last_frame = frames[-1]
    from_index = frame_to_index[first_frame["filename"]]
    to_index = frame_to_index[last_frame["filename"]]

    new_tags.append({
        "name": prefix, "to": to_index, "from": from_index,
        "direction": "forward"
    })

    return new_tags


def patch_frame_slices(sprite_json, frame_to_index):
    frames = sprite_json["frames"]
    slices = sprite_json['meta']['slices']

    def key_map(key):
        frame = key["frame"]
        new_key = key.copy()
        global_index = fetch_global_index(sprite_json, frame, frame_to_index)
        new_key["frame"] = global_index
        return new_key


    first_frame = frames[0]
    last_frame = frames[-1]
    from_index = frame_to_index[first_frame["filename"]]
    to_index = frame_to_index[last_frame["filename"]]

    new_slices = []
    for slice in slices:
        new_slice = slice.copy()
        new_slice["keys"] = [key_map(s) for s in slice["keys"]]
        new_slice["from"] = from_index
        new_slice["to"] = to_index
        new_slices.append(new_slice)

    return new_slices


def main():
    output = sys.argv[1]
    atlas_path = sys.argv[2]
    sprite_path = sys.argv[3:]

    atlas_json = load_json(atlas_path)

    frame_to_index = dict()
    frames = enumerate(atlas_json['frames'])

    for index, frame in frames:
        frame_to_index[frame['filename']] = index


    new_tags = []
    new_slices = []
    for path in sprite_path:
        sprite_json = load_json(path)
        prefix = os.path.basename(path)
        prefix = os.path.splitext(prefix)[0]
        new_tags += patch_frame_tag(prefix, sprite_json, frame_to_index)
        new_slices += patch_frame_slices(sprite_json, frame_to_index)

    for tag in new_tags:
        print(tag)

    atlas_json["meta"]["frameTags"] = new_tags
    atlas_json["meta"]["slices"] = new_slices

    with open(output, "w") as f:
        json.dump(atlas_json, f, indent=4)


if __name__ == "__main__":
    main()
