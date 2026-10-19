import libs.yaml
with open("config.yaml", "r") as f:
    config = libs.yaml.safe_load(f)

import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent / config["paths"]["pyLibs"]))

import lxml.etree as ET
from libs.svgpathtools import svg2paths2
from libs.scour import scour
import libs.cairosvg
from PIL import Image
from io import BytesIO

import json

ICONS_DIR   = config["paths"]["icons"]
MANIFEST    = config["fileNames"]["manifest"]
PRECISION   = config["svg"]["elements"]["precision"]

with open(ICONS_DIR + MANIFEST) as file:
    jsonData = json.load(file)

for svg in jsonData["files"]:
    svg = ICONS_DIR + svg

    tree = ET.parse(svg)
    root = tree.getroot()

    # Standardize attribs
    for element in root.iter():
        # Override fill attributes
        if element.get("fill") != "none":
            element.set(
                "fill", 
                config["svg"]["elements"]["color"]
            )

        # Override opacity attributes
        element.set(
            "opacity", 
            config["svg"]["elements"]["opacity"]
        )
        element.set(
            "fill-opacity", 
            config["svg"]["elements"]["opacity"]
        )
        element.set(
            "stroke-opacity", 
            config["svg"]["elements"]["opacity"]
        )

        # Remove conflicting inline styles
        if "style" in element.attrib:
            styles = element.get("style").split(";")
            styles = [
                s for s in styles
                if s.strip().split(":")[0].strip()
                not in (
                    "fill", 
                    "opacity", 
                    "fill-opacity", 
                    "stroke-opacity"
                )
            ]
            element.set("style", ";".join(styles))

    # set dims
    root.set(
        "width", 
        config["svg"]["elements"]["size"]
    )
    root.set(
        "height", 
        config["svg"]["elements"]["size"]
    )

    # set viewport
    paths, _, _ = svg2paths2(
        svg,
    )

    bounds = [path.bbox() for path in paths if len(path) > 0]

    xmin = min(b[0] for b in bounds)
    xmax = max(b[1] for b in bounds)
    ymin = min(b[2] for b in bounds)
    ymax = max(b[3] for b in bounds)

    root.set("viewBox", f"{xmin} {ymin} {xmax - xmin} {ymax - ymin}")

    # check if file is gone
    png = libs.cairosvg.svg2png(
        ET.tostring(root, encoding="utf-8")
    )
    image = Image.open(BytesIO(png)).convert("RGBA")
    if image.getchannel("A").getbbox() is None:
        print(f"WARNING: {svg} renders completely transparent!")
        continue # cancel conversion

    # write to file
    tree.write(svg, encoding="utf-8")

    # reduce precision/metadata
    options = scour.parse_args([])

    options.digits = PRECISION
    options.remove_metadata = (
        config
            ["svg"]
            ["optimization"]
            ["remove_metadata"]
    )
    options.strip_comments = (
        config
            ["svg"]
            ["optimization"]
            ["strip_comments"]
    )

    with open(svg, "r", encoding="utf-8") as f:
        file = f.read()
        scouredsvg = scour.scourString(file, options)

        # check if file is gone
        png = libs.cairosvg.svg2png(
            bytestring=scouredsvg.encode("utf-8")
        )
        image = Image.open(BytesIO(png)).convert("RGBA")
        if image.getchannel("A").getbbox() is None:
            print(f"WARNING: {svg} renders completely transparent!")
            continue # cancel conversion

        # save
        with open(svg, "w", encoding="utf-8") as f:
            f.write(scouredsvg)

# end program, log
print("complete")
