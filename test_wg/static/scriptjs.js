let worldData;
let world;
let left;
let right;
let sopra;
let sotto;
let gap;
let b_colors = false;
let e_lines = false;
let v_content = false;
const colors = {
    Ocean : {
      Grass: "#52ff3e",
      Grass2 : "#3fce31",
      Hill : "#017001",
      Hill2 : "#015001",
      Mountain : "#964b00",
      Mountain2 : "#854202",
      Lava : "#e74045",
      Snow : "#ffffff",
      Snow2 : "#eeeeee",
      Sand : "#fcdd75",
      Sand2 : "#e5c864",
      DeepWater : "#0030d3",
      DeepWater2 : "#0023b2",
      ShallowWater : "#40daf5",
      ShallowWater2 : "#49b1f1",
    },
    Land : {
      Grass: "#52ff3e",
      Grass2 : "#3fce31",
      Hill : "#017001",
      Hill2 : "#015001",
      Mountain : "#964b00",
      Mountain2 : "#854202",
      Lava : "#e74045",
      Snow : "#ffffff",
      Snow2 : "#eeeeee",
      Sand : "#fcdd75",
      Sand2 : "#e5c864",
      DeepWater : "#0030d3",
      DeepWater2 : "#0023b2",
      ShallowWater : "#40daf5",
      ShallowWater2 : "#49b1f1",
    },
    Plains : {
      Grass: "#52ff3e",
      Grass2 : "#3fce31",
      Hill : "#017001",
      Hill2 : "#015001",
      Mountain : "#964b00",
      Mountain2 : "#854202",
      Lava : "#e74045",
      Snow : "#ffffff",
      Snow2 : "#eeeeee",
      Sand : "#fcdd75",
      Sand2 : "#e5c864",
      DeepWater : "#0030d3",
      DeepWater2 : "#0023b2",
      ShallowWater : "#40daf5",
      ShallowWater2 : "#49b1f1",
    },
    Tundra : {
      Grass: "#8eff88",
      Grass2 : "#76d973",
      Hill : "#017001",
      Hill2 : "#015001",
      Mountain : "#964b00",
      Mountain2 : "#854202",
      Lava : "#e74045",
      Snow : "#ffffff",
      Snow2 : "#eeeeee",
      Sand : "#fcdd75",
      Sand2 : "#e5c864",
      DeepWater : "#0030d3",
      DeepWater2 : "#0023b2",
      ShallowWater : "#40daf5",
      ShallowWater2 : "#49b1f1",
    },
    ShrubLand : {
      Grass: "#dbf51a",
      Grass2 : "#dbf51a",
      Hill : "#94a60f",
      Hill2 : "#94a60f",
      Mountain : "#9a8f0b",
      Mountain2 : "#9a8f0b",
      Lava : "#e74045",
      Snow : "#ffffff",
      Snow2 : "#eeeeee",
      Sand : "#fcdd75",
      Sand2 : "#e5c864",
      DeepWater : "#0030d3",
      DeepWater2 : "#0023b2",
      ShallowWater : "#40daf5",
      ShallowWater2 : "#49b1f1",
    },
    Forest : {
      Grass: "#52ff3e",
      Grass2 : "#3fce31",
      Hill : "#017001",
      Hill2 : "#015001",
      Mountain : "#964b00",
      Mountain2 : "#854202",
      Lava : "#e74045",
      Snow : "#ffffff",
      Snow2 : "#eeeeee",
      Sand : "#fcdd75",
      Sand2 : "#e5c864",
      DeepWater : "#0030d3",
      DeepWater2 : "#0023b2",
      ShallowWater : "#40daf5",
      ShallowWater2 : "#49b1f1",
    },
    Swamp : {
      Grass: "#5e9f03",
      Grass2 : "#518602",
      Hill : "#017001",
      Hill2 : "#015001",
      Mountain : "#964b00",
      Mountain2 : "#854202",
      Lava : "#e74045",
      Snow : "#f41dfc",
      Snow2 : "#f41dfc",
      Sand : "#fcdd75",
      Sand2 : "#e5c864",
      DeepWater : "#0030d3",
      DeepWater2 : "#0023b2",
      ShallowWater : "#40daf5",
      ShallowWater2 : "#49b1f1",
    },
    RainForest : {
      Grass: "#32ff1d",
      Grass2 : "#23d910",
      Hill : "#017001",
      Hill2 : "#015001",
      Mountain : "#964b00",
      Mountain2 : "#854202",
      Lava : "#e74045",
      Snow : "#f41dfc",
      Snow2 : "#f41dfc",
      Sand : "#fcdd75",
      Sand2 : "#e5c864",
      DeepWater : "#0030d3",
      DeepWater2 : "#0023b2",
      ShallowWater : "#40f5da",
      ShallowWater2 : "#32bea9",
    },
    SeasonalForest : {
      Grass: "#d0fc0f",
      Grass2 : "#a5cb0e",
      Hill : "#017001",
      Hill2 : "#015001",
      Mountain : "#964b00",
      Mountain2 : "#854202",
      Lava : "#e74045",
      Snow : "#f41dfc",
      Snow2 : "#f41dfc",
      Sand : "#fcdd75",
      Sand2 : "#e5c864",
      DeepWater : "#0030d3",
      DeepWater2 : "#0023b2",
      ShallowWater : "#40daf5",
      ShallowWater2 : "#49b1f1",
    },
    Savanna : {
      Grass: "#dbf51a",
      Grass2 : "#dbf51a",
      Hill : "#94a60f",
      Hill2 : "#94a60f",
      Mountain : "#9a8f0b",
      Mountain2 : "#9a8f0b",
      Lava : "#e74045",
      Snow : "#ffffff",
      Snow2 : "#eeeeee",
      Sand : "#fcdd75",
      Sand2 : "#e5c864",
      DeepWater : "#0030d3",
      DeepWater2 : "#0023b2",
      ShallowWater : "#40daf5",
      ShallowWater2 : "#49b1f1",
    },
    Desert : {
      Grass: "#f41dfc",
      Grass2 : "#f41dfc",
      Hill : "#f41dfc",
      Hill2 : "#f41dfc",
      Mountain : "#f41dfc",
      Mountain2 : "#f41dfc",
      Lava : "#e74045",
      Snow : "#f41dfc",
      Snow2 : "#f41dfc",
      Sand : "#fcdd75",
      Sand2 : "#e5c864",
      DeepWater : "#0030d3",
      DeepWater2 : "#0023b2",
      ShallowWater : "#40daf5",
      ShallowWater2 : "#49b1f1",
    },
    Vulcan : {
      Grass: "#ffffff",
      Grass2 : "#ffffff",
      Hill : "#017001",
      Hill2 : "#015001",
      Mountain : "#964b00",
      Mountain2 : "#854202",
      Lava : "#e74045",
      Snow : "#ffffff",
      Snow2 : "#eeeeee",
      Sand : "#fcdd75",
      Sand2 : "#e5c864",
      DeepWater : "#0030d3",
      DeepWater2 : "#0023b2",
      ShallowWater : "#40daf5",
      ShallowWater2 : "#49b1f1",
    },
    Taiga : {
      Grass: "#209a11",
      Grass2 : "#1d830f",
      Hill : "#017001",
      Hill2 : "#015001",
      Mountain : "#964b00",
      Mountain2 : "#854202",
      Lava : "#e74045",
      Snow : "#ffffff",
      Snow2 : "#eeeeee",
      Sand : "#fcdd75",
      Sand2 : "#e5c864",
      DeepWater : "#0030d3",
      DeepWater2 : "#0023b2",
      ShallowWater : "#40daf5",
      ShallowWater2 : "#49b1f1",
    },
}
function generateWorld() {
    fetch('http://127.0.0.1:8000/get_data')
        .then(response => response.json())
        .then(data => {
            // Handle the received data
            worldData = {
                size: data.size,
                tiles: data.tiles,
                biomes: data.biomes,
            }
            world = worldData;
            left = 0;
            right = worldData.size;
            sopra = 0;
            sotto = worldData.size;
            gap = worldData.size/2;
            drawWorld(worldData);
        })
        .catch(error => {
            console.error('Error fetching data:', error);

        });
}

function zoom_out() {
    if (world.size*2 <= worldData.size ){
        world = {
            size: world.size*2,
            tiles: world.tiles,
            biomes: world.biomes,
        }
        gap = gap*2;
        left = 0;
        right = worldData.size;
        sopra = 0;
        sotto = worldData.size;
        drawWorld(world);
    }
}

function zoom_in() {
    if (world.size/2 >= 1 ) {
        world = {
            size: Math.floor(world.size / 2),
            tiles: world.tiles,
            biomes: world.biomes,
        }
        gap = Math.floor(gap / 2);
        if (gap === 0) {
            gap = 1;
        }
        right = Math.floor(right / 2);
        sotto = Math.floor(sotto / 2);
        drawWorld(world);
    }
}

function go_right() {
    if (right+gap <= worldData.size ){
        right += gap;
        left += gap;
        drawWorld(world);
    }
}
function go_left() {
    if (left-gap >= 0){
        right -= gap;
        left -= gap;
        drawWorld(world);
    }
}
function go_down() {
    if (sotto+gap <= worldData.size ){
        sotto += gap;
        sopra += gap;
        drawWorld(world);
    }
}
function go_up() {
    if (sopra-gap >= 0 ){
        sotto -= gap;
        sopra -= gap;
        drawWorld(world);
    }
}
function biome_colors() {
    b_colors = !b_colors;
    drawWorld(world);
}
function elevation_lines() {
    e_lines = !e_lines;
    drawWorld(world);
}
function view_content() {
    v_content = !v_content;
    drawWorld(world);
}
function visualizeBiomized(tile,leftile,context,colors){
    console.log(tile.elevation-leftile.elevation)
    if ( ( tile.elevation-leftile.elevation > 1 || tile.elevation-leftile.elevation < -1 ) && e_lines){
        if (tile.tile_type === "Grass"){
            context.fillStyle = colors.Grass2
        }else if (tile.tile_type === "Sand"){
            context.fillStyle = colors.Sand2
        }else if (tile.tile_type === "Hill"){
            context.fillStyle = colors.Hill2
        }else if (tile.tile_type === "Mountain"){
            context.fillStyle = colors.Mountain2
        }else if (tile.tile_type === "Snow") {
            context.fillStyle = colors.Snow2
        }else if (tile.tile_type === "Lava") {
            context.fillStyle = colors.Lava
        }else if (tile.tile_type === "ShallowWater") {
            context.fillStyle = colors.ShallowWater2
        }else if (tile.tile_type === "DeepWater") {
            context.fillStyle = colors.DeepWater2
        }else{ context.fillStyle = "purple" }
    }else {
        if (tile.tile_type === "Grass") {
            context.fillStyle = colors.Grass
        } else if (tile.tile_type === "Sand") {
            context.fillStyle = colors.Sand
        } else if (tile.tile_type === "Hill") {
            context.fillStyle = colors.Hill
        } else if (tile.tile_type === "Mountain") {
            context.fillStyle = colors.Mountain
        } else if (tile.tile_type === "Snow") {
            context.fillStyle = colors.Snow
        } else if (tile.tile_type === "Lava") {
            context.fillStyle = colors.Lava
        } else if (tile.tile_type === "ShallowWater") {
            context.fillStyle = colors.ShallowWater
        } else if (tile.tile_type === "DeepWater") {
            context.fillStyle = colors.DeepWater
        }else {
            context.fillStyle = "purple"
        }
    }
}

function drawWorld(w_orld) {
    console.log(w_orld);

    const canvas = document.getElementById("worldCanvas")
    const context = canvas.getContext("2d")
    const tileSize = canvas.width / w_orld.size // Adjust this based on your preference

    if (context) {
        for (let row = 0; row < w_orld.size; row++) {
            for (let col = 0; col < w_orld.size; col++) {
                const tile = w_orld.tiles[sopra + row][left + col]
                const biome = w_orld.biomes[sopra + row][left + col]
                var leftile;
                if ((left + col) > 0) {
                    leftile = w_orld.tiles[sopra + row][left + col - 1]
                }else if((sopra + row) > 0) {
                    leftile = w_orld.tiles[sopra + row - 1][left + col]
                }else {
                    leftile = tile
                }
                const x = col * tileSize
                const y = row * tileSize

                // Draw the tile based on its biome
                if (b_colors){
                    if ( biome === "Savana" ){
                        visualizeBiomized(tile,leftile,context,colors.Savanna)
                    }else if (biome === "Plains") {
                        visualizeBiomized(tile,leftile,context,colors.Plains)
                    }else if (biome === "Swamp") {
                        visualizeBiomized(tile,leftile,context,colors.Swamp)
                    }else if (biome === "Taiga") {
                        visualizeBiomized(tile,leftile,context,colors.Taiga)
                    }else if (biome === "Tundra") {
                        visualizeBiomized(tile,leftile,context,colors.Tundra)
                    }else if (biome === "Ocean") {
                        visualizeBiomized(tile,leftile,context,colors.Ocean)
                    }else if (biome === "Land") {
                        visualizeBiomized(tile,leftile,context,colors.Land)
                    }else if (biome === "ShrubLand") {
                        visualizeBiomized(tile,leftile,context,colors.ShrubLand)
                    }else if (biome === "Forest") {
                        visualizeBiomized(tile,leftile,context,colors.Forest)
                    }else if (biome === "RainForest") {
                        visualizeBiomized(tile,leftile,context,colors.RainForest)
                    }else if (biome === "SeasonalForest") {
                        visualizeBiomized(tile,leftile,context,colors.SeasonalForest)
                    }else if (biome === "Desert") {
                        visualizeBiomized(tile,leftile,context,colors.Desert)
                    }else if (biome === "Vulcan") {
                        visualizeBiomized(tile,leftile,context,colors.Vulcan)
                    }
                }else {
                    visualizeBiomized(tile,leftile,context,colors.Land)
                }
                if (tile.tile_type === "Wall") {
                    context.fillStyle = "#572308"
                } else if (tile.tile_type === "Street") {
                    context.fillStyle = "grey"
                }
                if (tile.tile_type.Teleport === false) {
                    let img_teleport = new Image();
                    img_teleport.src = "./images/teleport.png";
                    img_teleport.onload = function (){
                        context.drawImage(img_teleport, x , y, tileSize+1, tileSize+1);
                    }
                }else if (tile.tile_type.Teleport === true){
                    let img_teleport = new Image();
                    img_teleport.src = "./images/teleport.png";
                    img_teleport.onload = function (){
                        context.drawImage(img_teleport, x , y, tileSize+1, tileSize+1);
                    }
                }
                context.fillRect(x, y, tileSize+1, tileSize+1)
            // Add additional visual representation for properties
                if (tile.content !== "None" && v_content){
                    if (tile.content.Tree >= 1) {
                        let img_tree = new Image();
                        if (biome === "Taiga" || biome === "Tundra"){
                            img_tree.src = "./images/taiga_tree.svg";
                        }else {
                            img_tree.src = "./images/tree.svg";
                        }
                        img_tree.onload = function (){
                            context.drawImage(img_tree, x , y, tileSize+1, tileSize+1);
                        }
                    }else if (tile.content.Bush >= 1) {
                        let img_tree = new Image();
                        img_tree.src = "./images/bush.svg";
                        img_tree.onload = function (){
                            context.drawImage(img_tree, x , y, tileSize+1, tileSize+1);
                        }
                    }else if (tile.content === "Fire") {
                        let img_fire = new Image();
                        img_fire.src = "./images/fire.svg";
                        img_fire.onload = function (){
                            context.drawImage(img_fire, x , y, tileSize+1, tileSize+1);
                        }
                    }
                    else if (tile.content.Rock >= 1) {
                        let img_rock = new Image();
                        if (tile.tile_type === "Snow") {
                            img_rock.src = "./images/snow_rock.svg";
                        }else{
                            img_rock.src = "./images/rock.svg";
                        }
                        img_rock.onload = function (){
                            context.drawImage(img_rock, x , y, tileSize+1, tileSize+1);
                        }
                    }
                    else if (tile.content.Garbage >= 1) {
                        let img_garbage = new Image();
                        img_garbage.src = "./images/garbage.svg";
                        img_garbage.onload = function (){
                            context.drawImage(img_garbage, x , y, tileSize+1, tileSize+1);
                        }
                    }
                    else if (tile.content.Fish >= 1) {
                        let img_fish = new Image();
                        img_fish.src = "./images/fish.svg";
                        img_fish.onload = function (){
                            context.drawImage(img_fish, x , y, tileSize+1, tileSize+1);
                        }
                    }
                    else if (tile.content.Coin >= 1) {
                        let img_coin = new Image();
                        if (tile.content.Coin < 5) {
                            img_coin.src = "./images/coin1.svg";
                        }else if(tile.content.Coin < 10){
                            img_coin.src = "./images/coin2.svg";
                        }else{
                            img_coin.src = "./images/coin3.svg";
                        }
                        img_coin.onload = function (){
                            context.drawImage(img_coin, x , y, tileSize+1, tileSize+1);
                        }
                    }
                    else if (tile.content.Bin?.start >= 0 ) {
                        let img_bin = new Image();
                        img_bin.src = "./images/bin.svg";
                        img_bin.onload = function (){
                            context.drawImage(img_bin, x , y, tileSize+1, tileSize+1);
                        }
                    }
                    else if (tile.content.Bank?.start >= 0 ) {
                        let img_bank = new Image();
                        img_bank.src = "./images/bank.svg";
                        img_bank.onload = function (){
                            context.drawImage(img_bank, x , y, tileSize+1, tileSize+1);
                        }
                    }else if (tile.content.Market >= 1) {
                        let img_market = new Image();
                        img_market.src = "./images/market.svg";
                        img_market.onload = function () {
                            context.drawImage(img_market, x, y, tileSize+1, tileSize+1);
                        }
                    }
                    if (tile.content.Crate?.start >= 0 ) {
                        let img_crate = new Image();
                        img_crate.src = "./images/crate.svg";
                        img_crate.onload = function () {
                            context.drawImage(img_crate, x, y, tileSize+1, tileSize+1);
                        }
                    }
                    else if (tile.content === "Building") {
                        let img_building = new Image();
                        img_building.src = "./images/building.svg";
                        img_building.onload = function (){
                            context.drawImage(img_building, x , y, tileSize+1, tileSize+1);
                            }
                    }
                    else if (tile.content === "Scarecrow") {
                        let img_building = new Image();
                        if (tile.tile_type === "ShallowWater"){
                            img_building.src = "./images/scarecrow.svg";
                        }else {
                            img_building.src = "./images/scarecrow2.svg";
                        }
                        img_building.onload = function (){
                            context.drawImage(img_building, x , y, tileSize+1, tileSize+1);
                        }
                    }
                }
            }
        }
    }
}