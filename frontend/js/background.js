const bgPath = "/media/background/";

// grab the paths from manifest
const manifest = await fetch(`${bgPath}manifest.json`)
    .then(response => response.json())
    .catch(error => {
        console.error(error);
        return [];
    });
const icons = manifest.files.sort(() => 0 <= Math.random() - 0.5);

const patternCols = Math.ceil(Math.sqrt(icons.length));
const patternRows = Math.ceil(icons.length / patternCols);

const rotation = Math.random() * 360;

const grid = document.querySelector(".icon-grid");

async function buildGrid() {
    const tileSize = parseFloat(
        getComputedStyle(grid).getPropertyValue("--tile-size")
    );

    const patternWidth = patternCols * tileSize;
    const patternHeight = patternRows * tileSize;
    
    // Account for the grid being rotated.
    const size = Math.hypot(window.innerWidth, window.innerHeight);
    const scrollSpeed = 10 * Math.hypot(patternCols, patternRows) / Math.SQRT2;

    const width = size + 2 * patternWidth;
    const height = size + 2 * patternHeight;

    const columns = Math.ceil((width + patternWidth) / tileSize);
    const rows = Math.ceil(height / tileSize);

    // set bounds
    grid.style.width = `${width}px`;
    grid.style.height = `${height}px`;
    
    grid.style.setProperty("--pattern-width", `${patternWidth}px`);
    grid.style.setProperty("--pattern-height", `${patternHeight}px`);
    grid.style.setProperty("--scroll-speed", `${scrollSpeed}s`);
    document.querySelector(".grid-angle")
        .style.setProperty("--rotation", `${rotation}deg`);

    // create grid
    const fragment = document.createDocumentFragment();
    let rotationMap = new Map();    // effectivly seeding rotation

    for (let y = 0; y < rows; y++) {
        const row = document.createElement("div");
        row.className = "icon-row";

        for (let x = 0; x < columns; x++) {
            const icon = document.createElement("div");
            icon.className = "icon";

            const img = document.createElement("img");
            // img.src = bgPath + icons[x % icons.length];
            // img.src = bgPath + icons[(y % 2) * 2 + (x % 2)];
            img.src = bgPath + icons[
                ((y % patternRows) * patternCols + (x % patternCols))
                % icons.length
            ];
            img.alt = "";

            // set the initial rotation vector for randomized icons
            if (!rotationMap.has(img.src) && img.src != undefined) { 
                rotationMap.set(img.src, Math.random() * 360); 
                // console.log(img.src, ":", rotationMap.get(img.src));
            }

            img.style.setProperty(
                "--initial-rotation",
                `${rotationMap.get(img.src)}deg`
            );

            icon.appendChild(img);
            row.appendChild(icon);
        }

        fragment.appendChild(row);
    }

    grid.replaceChildren(fragment);
}

function complementaryColor(source, target = source) {
    const color = getComputedStyle(source).backgroundColor;

    target.style.backgroundColor = `hsl(from ${color} calc(h + 180) s l / alpha)`;
    // console.log(target.style.backgroundColor)
}

function initialColor(target) {
    const color = getComputedStyle(target).backgroundColor;

    target.style.backgroundColor = `hsl(from ${color} calc(h + ${Math.random() * 360}) s l / alpha)`;
}

initialColor(
    document.querySelector(".animated-background"),
);
complementaryColor(
    document.querySelector(".animated-background"),
    document.querySelector(".menubar")
);

buildGrid();
window.addEventListener("resize", buildGrid);