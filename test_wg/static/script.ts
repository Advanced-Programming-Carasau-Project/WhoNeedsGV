function generateWorld(): void {
    let worldData: {
        size: number;
        tiles: string[][];
    };
    fetch('http://127.0.0.1:8000/get_data')
        .then(response => response.json())
        .then(data => {
            console.log(data);
            // Handle the received data
            worldData = {
                size: data.size,
                tiles: data.tiles
            }
            drawWorld(data);
        })
        .catch(error => {
            console.error('Error fetching data:', error);

        });
}

function drawWorld(world: { size: number; tiles: string[][]; }): void {
    const canvas = document.getElementById("worldCanvas") as HTMLCanvasElement;
    const ctx = canvas.getContext("2d");
    const tileSize = canvas.width / world.size; // Adjust this based on your preference
    if (ctx) {
        for (let row = 0; row < world.size; row++) {
            for (let col = 0; col < world.size; col++) {
                const tile = world.tiles[row][col];
                const x = col * tileSize;
                const y = row * tileSize;

                // Draw the tile based on its type
                if (tile === "Grass") {
                    ctx.fillStyle = "green";
                } else if (tile === "Sand") {
                    ctx.fillStyle = "#fcb475";
                } else if (tile === "Street") {
                    ctx.fillStyle = "gray";
                } else if (tile === "Water") {
                    ctx.fillStyle = "blue";
                } else if (tile === "Hill") {
                    ctx.fillStyle = "darkgreen";
                } else if (tile === "Mountain") {
                    ctx.fillStyle = "#964b00";
                } else if (tile === "Snow") {
                    ctx.fillStyle = "white";
                } else if (tile === "Lava") {
                    ctx.fillStyle = "orange";
                } else if (tile === "ShallowWater") {
                    ctx.fillStyle = "#007fff";
                } else if (tile === "DeepWater") {
                    ctx.fillStyle = "blue";
                } else {
                    ctx.fillStyle = "black";
                }
                ctx.fillRect(x, y, tileSize, tileSize);
            }
        }
    }
}