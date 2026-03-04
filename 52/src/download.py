from PIL import Image
import requests, numpy as np, io, sys



# Download image
r = requests.get(sys.argv[1]).content
img = Image.open(io.BytesIO(r)).convert("RGB")

# Resize to VGA 80x25
img = img.resize((80, 25))
img_array = np.array(img)

# Map pixels to Rust tuples
def colormapper(pixel):
    r, g, b = pixel[:3]
    return f"({r},{g},{b})"

rows = [
    ", ".join(colormapper(pixel) for pixel in row)
    for row in img_array
]

# Build Rust array
rust_array = "pub const IMG: [[(u8,u8,u8); 80]; 25] = [\n"
rust_array += ",\n".join(f"    [{row}]" for row in rows)
rust_array += "\n];\n"

# Write to file
with open("src/colors/img.rs", "w") as f:
    f.write(rust_array)

print("Rust file written: src/colors/img.rs")



