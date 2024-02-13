let worldData: any; //
let world: any; //
let left: number;
let right: number;
let sopra: number;
let sotto: number;
let gap: number;
let v_content: boolean = true;
let stop_mode: boolean;
let first_request: boolean = true;
let energy: number = 1000;
let i_robot: number;
let j_robot: number;
let cont_zoom_in: number = 0;
const colors: any = {
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
      Unknown: "#000000",
    }
}

document.addEventListener('keydown', function(event: KeyboardEvent) {
    if ((event.key === '-' || event.key === 'm') && (document.getElementById("zoom_out") as HTMLButtonElement).disabled === false) {
        document.getElementById("zoom_out").click();
    }
});
document.addEventListener('keydown', function(event: KeyboardEvent) {
    if ((event.key === '+' || event.key === 'p') && (document.getElementById("zoom_in") as HTMLButtonElement).disabled === false) {
        document.getElementById("zoom_in").click();
    }
});
document.addEventListener('keydown', function(event: KeyboardEvent) {
    if (event.key === 'w' || event.key === "ArrowUp") {
        document.getElementById("go_up").click();
    }
});
document.addEventListener('keydown', function(event: KeyboardEvent) {
    if (event.key === 's' || event.key === "ArrowDown") {
        document.getElementById("go_down").click();
    }
});
document.addEventListener('keydown', function(event: KeyboardEvent) {
    if (event.key === 'd' || event.key === "ArrowRight") {
        document.getElementById("go_right").click();
    }
});
document.addEventListener('keydown', function(event: KeyboardEvent) {
    if (event.key === 'a' || event.key === "ArrowLeft") {
        document.getElementById("go_left").click();
    }
});
document.addEventListener('keydown', function(event: KeyboardEvent) {
    if (event.key === 'c') {
        if (event.repeat) { return }
        document.getElementById("view_content").click();
    }
});

function disable_button(button_id: string) {
    var button = document.getElementById(button_id) as HTMLButtonElement;
    button.disabled = true;
    button.style.color = 'Grey';
    button.style.borderColor = 'Grey';
}

function enable_button(button_id: string){
    var button = document.getElementById(button_id) as HTMLButtonElement;
    button.disabled = false;
    button.style.color = 'White';
    button.style.borderColor = 'White';
}

async function enable_auto_mode(){
    stop_mode = false;
    enable_button("stop_auto");
    disable_button("automatic_tick_processing");
    disable_button("next_tick");
    disable_button("zoom_in");
    disable_button("zoom_out");
    await process_next_tick();
    disable_button("automatic_tick_processing");
    disable_button("next_tick");
    disable_button("zoom_in");
    disable_button("zoom_out");
    await sleep(2000);
    if(stop_mode === false){
        enable_auto_mode();
    }
    else{
        enable_button("automatic_tick_processing");
        enable_button("next_tick");
        enable_button("zoom_in");
        enable_button("zoom_out");
    }
}

async function stop_auto_mode(){
    stop_mode = true;
    disable_button("stop_auto");
    await sleep(2000);
}

async function process_next_tick() {
    var was_previusly_enabled = !((document.getElementById("automatic_tick_processing") as HTMLButtonElement).disabled);
    disable_button("automatic_tick_processing");
    disable_button("next_tick");
    disable_button("zoom_in");
    disable_button("zoom_out");
    await fetch('http://127.0.0.1:8000/get_robot_data')
        .then(response => response.json())
        .then(async data => {
            worldData = {
                size: data.world_size,
                tiles: data.world,
            }
            if (first_request===true) { //se è il primo process tick
                world = worldData;
                left = 0;
                right = worldData.size;
                sopra = 0;
                sotto = worldData.size;
                gap = worldData.size / 2;
                first_request = false;

                i_robot = data.positions[0];
                j_robot = data.positions[1];

                await make_animation(world, data.ser_events);
                update_backpack(data.backpack);
                update_points(data.points);
                update_energy(data.energy);

                energy = data.energy;
                worldData.tiles[i_robot][j_robot].robot = true;
                await drawWorld(worldData);
            }
            else{
                await make_animation(world, data.ser_events);

                update_backpack(data.backpack);
                update_points(data.points);
                update_energy(data.energy);

                energy = data.energy;
                i_robot = data.positions[0];
                j_robot = data.positions[1];
                worldData.tiles[i_robot][j_robot].robot = true;
                await updateWorld(world);
                world = worldData;
            }
        })
        .catch(error => {
            console.error('Error fetching data:', error);
        });
    enable_button("automatic_tick_processing");
    enable_button("next_tick");
    enable_button("zoom_in");
    enable_button("zoom_out");
}

async function make_animation(world: any, events: any[]) {
    for (let i = 0; i < events.length; i++) {
        const event = events[i];
        if (event.EnergyConsumed) {
            energy = energy - event.EnergyConsumed;
            update_energy(energy);
            await sleep(10);
        }
        else if (event.EnergyRecharged) {
            energy = energy + event.EnergyRecharged;
            update_energy(energy);
            await sleep(10);
        }
        else if (event.Moved){
            var tile = event.Moved[0];
            var new_i_robot = event.Moved[1][0];
            var new_j_robot = event.Moved[1][1];
            await update_robot_position(i_robot, j_robot, new_i_robot, new_j_robot, tile);
            i_robot = new_i_robot;
            j_robot = new_j_robot;
            await sleep(50);
        }
        else if (event.TileContentUpdated){
            var tile = event.TileContentUpdated[0];
            var i_pos = event.TileContentUpdated[1][0];
            var j_pos = event.TileContentUpdated[1][1];
            await update_tile(i_pos, j_pos, tile);
            await sleep(50);
        }
        else if (event.AddedToBackpack){
            var item = event.AddedToBackpack[0];
            var quantity = event.AddedToBackpack[1];
            update_backpack_item(item, quantity, true);
            await sleep(10);
        }
        else if(event.RemovedFromBackpack){
            var item = event.RemovedFromBackpack[0];
            var quantity = event.RemovedFromBackpack[1];
            update_backpack_item(item, quantity, false);
            await sleep(10);
        }
        else if(event.TimeChanged){
            var hour = event.TimeChanged.time_of_day.hour;
            var minute = event.TimeChanged.time_of_day.minute;
            var weather = event.TimeChanged.weather_forecast[0];
            update_time(hour, minute);
            update_weather(weather);
            await sleep(10);
        }
    }
}

function update_time(hour: number, minute: number){
    var elem = document.getElementById("update_time") as HTMLSpanElement;
    elem.textContent = hour + ":" + minute;
}

function update_weather(weather: string){
    var elem = document.getElementById("update_weather") as HTMLImageElement;
    elem.src = "./images/weather_" + weather.toLowerCase() + ".jpg";
}

function update_item(item: string, quantity: number, insert: boolean){
    var elem = document.getElementById("update_"+item) as HTMLSpanElement;
    if (insert === true){
        elem.textContent = (parseInt(elem.textContent) + quantity).toString();;
    }
    else{
        elem.textContent = (parseInt(elem.textContent) - quantity).toString();;
    }
}

function update_backpack_item(item: any, quantity: number, insert: boolean){
    if("Tree" in item){
        update_item("Tree", quantity, insert);
    }
    else if("Rock" in item){
        update_item("Rock", quantity, insert);
    }
    else if("Garbage" in item){
        update_item("Garbage", quantity, insert);
    }
    else if("Fire" in item){
        update_item("Fire", quantity, insert);
    }
    else if("Coin" in item){
        update_item("Coin", quantity, insert);
    }
    else if("Water" in item){
        update_item("Water", quantity, insert);
    }
    else if("Fish" in item){
        update_item("Fish", quantity, insert);
    }
    else if("Bush" in item){
        update_item("Bush", quantity, insert);
    }
    else if("JollyBlock" in item){
        update_item("JollyBlock", quantity, insert);
    }
}

async function update_robot_position(i_robot: number, j_robot: number, new_i_robot: number, new_j_robot: number, tile: any){
    const canvas = document.getElementById("worldCanvas") as HTMLCanvasElement;
    const context = canvas.getContext("2d");
    const tileSize = canvas.width / world.size;
    world.tiles[i_robot][j_robot].robot = false;
    world.tiles[new_i_robot][new_j_robot] = tile;
    world.tiles[new_i_robot][new_j_robot].robot = true;
    await draw_single_cell(world, i_robot, j_robot, context, tileSize);
    await draw_single_cell(world, new_i_robot, new_j_robot, context, tileSize);
}

async function update_tile(i: number, j: number, tile: any){
    const canvas = document.getElementById("worldCanvas") as HTMLCanvasElement;
    const context = canvas.getContext("2d");
    const tileSize = canvas.width / world.size;
    world.tiles[i][j] = tile;
    await draw_single_cell(world, i, j, context, tileSize);
}

function update_backpack(backpack: any){
    document.getElementById("update_Rock").textContent = backpack.ROCK;
    document.getElementById("update_Tree").textContent = backpack.TREE;
    document.getElementById("update_Garbage").textContent = backpack.GARBAGE;
    document.getElementById("update_Fire").textContent = backpack.FIRE;
    document.getElementById("update_Coin").textContent = backpack.COIN;
    document.getElementById("update_Water").textContent = backpack.WATER;
    document.getElementById("update_Fish").textContent = backpack.FISH;
    document.getElementById("update_Bush").textContent = backpack.BUSH;
    document.getElementById("update_JollyBlock").textContent = backpack.JOLLYBLOCK;
}
function update_points(points: number){
    var spanElement = document.getElementById("update_points") as HTMLSpanElement;
    spanElement.textContent = points.toString();
}

function update_energy(energy: number){
    var spanElement = document.getElementById("update_energy") as HTMLSpanElement;
    if (energy > 1000){
        energy = 1000;
    }
    if (energy<0){
        energy = 0;
    }
    spanElement.textContent = energy.toString();
}

async function zoom_out() {
    if (world.size*2 <= worldData.size ){
        cont_zoom_in = cont_zoom_in - 1;
        if (cont_zoom_in <= 0){
            enable_button("automatic_tick_processing");
            enable_button("next_tick");
            cont_zoom_in = 0;
        }
        world = {
            size: world.size*2,
            tiles: world.tiles,
        }
        gap = gap*2;
        left = 0;
        right = worldData.size;
        sopra = 0;
        sotto = worldData.size;
        await drawWorld(world);
    }
}

async function zoom_in() {
    if (world.size/2 >= 1 ) {
        cont_zoom_in = cont_zoom_in + 1;
        disable_button("automatic_tick_processing");
        disable_button("next_tick");
        world = {
            size: Math.floor(world.size / 2),
            tiles: world.tiles,
        }
        gap = Math.floor(gap / 2);
        if (gap === 0) {
            gap = 1;
        }
        right = Math.floor(right / 2);
        sotto = Math.floor(sotto / 2);
        await drawWorld(world);
    }
}

async function go_right() {
    if (right+gap <= worldData.size ){
        right += gap;
        left += gap;
        await drawWorld(world);
    }
}
async function go_left() {
    if (left-gap >= 0){
        right -= gap;
        left -= gap;
        await drawWorld(world);
    }
}
async function go_down() {
    if (sotto+gap <= worldData.size ){
        sotto += gap;
        sopra += gap;
        await drawWorld(world);
    }
}
async function go_up() {
    if (sopra-gap >= 0 ){
        sotto -= gap;
        sopra -= gap;
        await drawWorld(world);
    }
}
async function view_content() {
    v_content = !v_content;
    let tmpWorld: any = {
        size: world.size,
        tiles: worldData.tiles,
    };
    await drawWorld(tmpWorld);
}
async function display_tiletype(tile: any, context: any, colors: any) {
    if (tile) {
        //console.log(tile.elevation - leftile.elevation);
        if (tile.tile_type === "Grass") {
            context.fillStyle = colors.Grass2;
        } else if (tile.tile_type === "Sand") {
            context.fillStyle = colors.Sand2;
        } else if (tile.tile_type === "Hill") {
            context.fillStyle = colors.Hill2;
        } else if (tile.tile_type === "Mountain") {
            context.fillStyle = colors.Mountain2;
        } else if (tile.tile_type === "Snow") {
            context.fillStyle = colors.Snow2;
        } else if (tile.tile_type === "ShallowWater") {
            context.fillStyle = colors.ShallowWater2;
        } else if (tile.tile_type === "DeepWater") {
            context.fillStyle = colors.DeepWater2;
        }
        else if (tile.tile_type === "Wall") {
            context.fillStyle = "#572308";
        }
        else if (tile.tile_type === "Street") {
            context.fillStyle = "grey";
        }
        else {
            context.fillStyle = colors.Unknown;
        }
        return true;
    } else {
        // console.log("One or both tiles are null or do not have elevation property.");
        // Handle the situation where one or both tiles are null or do not have elevation property.
        return false;
    }
}


async function drawWorld(world: any) {
    //console.log(world);

    const canvas = document.getElementById("worldCanvas") as HTMLCanvasElement;
    const context = canvas.getContext("2d");
    context.clearRect(0, 0, canvas.width, canvas.height);
    const tileSize = canvas.width / world.size; // Adjust this based on your preference

    if (context) {
        for (let row = 0; row < world.size; row++) {
            for (let col = 0; col < world.size; col++) {
                await draw_single_cell(world, row, col, context, tileSize);
            }
        }
    }
}

async function updateWorld(world: any){
    const canvas = document.getElementById("worldCanvas") as HTMLCanvasElement;
    const context = canvas.getContext("2d");
    const tileSize = canvas.width / world.size; // Adjust this based on your preference

    if (context) {
        for (let row = 0; row < world.size; row++) {
            for (let col = 0; col < world.size; col++) {
                if(JSON.stringify(world.tiles[row][col]) !== JSON.stringify(worldData.tiles[row][col])){
                    await draw_single_cell(worldData, row, col, context, tileSize);
                }
            }
        }
    }
}

async function draw_single_cell(world: any, row: number, col: number, context: any, tileSize: number){
    // Check if the indices are within bounds
    if (world.tiles && world.tiles.length > 0 &&
        world.tiles[row + sopra] && world.tiles[row + sopra].length > 0 &&
        world.tiles[row + sopra][col + left]) {

        const tile = world.tiles[row + sopra][col + left];
        let flag_robot = false;

        const x = col * tileSize;
        const y = row * tileSize;

        // Draw the tile based on its biome
        let res: boolean = await display_tiletype(tile, context, colors.Land);

        if (tile.tile_type.Teleport && tile.tile_type.Teleport === true) {
            let img_teleport = new Image();
            img_teleport.src = "./images/teleport.png";
            img_teleport.onload = function () {
                context.drawImage(img_teleport, x, y, tileSize + 1, tileSize + 1);
            }
            res = true;
        }
        else if (tile.tile_type === "Lava"){
            let jpg_lava = new Image();
            jpg_lava.src = "./images/lava.jpg";
            jpg_lava.onload = function () {
                context.drawImage(jpg_lava, x, y, tileSize + 1, tileSize + 1);
            }
            res = true;
        }

        if (tile.robot && tile.robot === true) {
            let img_robot = new Image();
            img_robot.src = "./images/robot.svg";
            img_robot.onload = function () {
                context.drawImage(img_robot, x, y, tileSize + 1, tileSize + 1);
            }
            flag_robot = true;
        }

        if (!res){
            return;
        }

        // Fill the tile
        context.fillRect(x, y, tileSize + 1, tileSize + 1);


        // Add additional visual representation for properties
        if (tile.content !== "None" && v_content && !flag_robot) {
            if (tile.content.Tree && tile.content.Tree >= 1) {
                let img_tree = new Image();
                img_tree.src = "./images/tree.svg";
                img_tree.onload = function () {
                    context.drawImage(img_tree, x, y, tileSize + 1, tileSize + 1);
                }
            } else if (tile.content.Bush && tile.content.Bush >= 1) {
                let img_tree = new Image();
                img_tree.src = "./images/bush.svg";
                img_tree.onload = function () {
                    context.drawImage(img_tree, x, y, tileSize + 1, tileSize + 1);
                }
            } else if (tile.content === "Fire") {
                let img_fire = new Image();
                img_fire.src = "./images/fire.svg";
                img_fire.onload = function () {
                    context.drawImage(img_fire, x, y, tileSize + 1, tileSize + 1);
                }
            } else if (tile.content.Rock && tile.content.Rock >= 1) {
                let img_rock = new Image();
                if (tile.tile_type === "Snow") {
                    img_rock.src = "./images/snow_rock.svg";
                } else {
                    img_rock.src = "./images/rock.svg";
                }
                img_rock.onload = function () {
                    context.drawImage(img_rock, x, y, tileSize + 1, tileSize + 1);
                }
            } else if (tile.content.Garbage && tile.content.Garbage >= 1) {
                let img_garbage = new Image();
                img_garbage.src = "./images/garbage.svg";
                img_garbage.onload = function () {
                    context.drawImage(img_garbage, x, y, tileSize + 1, tileSize + 1);
                }
            } else if (tile.content.Fish && tile.content.Fish >= 1) {
                let img_fish = new Image();
                img_fish.src = "./images/fish.svg";
                img_fish.onload = function () {
                    context.drawImage(img_fish, x, y, tileSize + 1, tileSize + 1);
                }
            } else if (tile.content.Coin && tile.content.Coin >= 1) {
                let img_coin = new Image();
                if (tile.content.Coin < 5) {
                    img_coin.src = "./images/coin1.svg";
                } else if (tile.content.Coin < 10) {
                    img_coin.src = "./images/coin2.svg";
                } else {
                    img_coin.src = "./images/coin3.svg";
                }
                img_coin.onload = function () {
                    context.drawImage(img_coin, x, y, tileSize + 1, tileSize + 1);
                }
            } else if (tile.content.Bin && tile.content.Bin?.start >= 0) {
                let img_bin = new Image();
                img_bin.src = "./images/bin.svg";
                img_bin.onload = function () {
                    context.drawImage(img_bin, x, y, tileSize + 1, tileSize + 1);
                }
            } else if (tile.content.Bank && tile.content.Bank?.start >= 0) {
                let img_bank = new Image();
                img_bank.src = "./images/bank.svg";
                img_bank.onload = function () {
                    context.drawImage(img_bank, x, y, tileSize + 1, tileSize + 1);
                }
            } else if (tile.content.Market && tile.content.Market >= 0) {
                let img_market = new Image();
                img_market.src = "./images/market.svg";
                img_market.onload = function () {
                    context.drawImage(img_market, x, y, tileSize + 1, tileSize + 1);
                }
            }
            else if (tile.content.Crate && tile.content.Crate?.start >= 0) {
                let img_crate = new Image();
                img_crate.src = "./images/crate.svg";
                img_crate.onload = function () {
                    context.drawImage(img_crate, x, y, tileSize + 1, tileSize + 1);
                }
            } else if (tile.content === "Building") {
                let img_building = new Image();
                img_building.src = "./images/building.svg";
                img_building.onload = function () {
                    context.drawImage(img_building, x, y, tileSize + 1, tileSize + 1);
                }
            } else if (tile.content === "Scarecrow") {
                let img_scarecrow = new Image();
                if (tile.tile_type === "ShallowWater") {
                    img_scarecrow.src = "./images/scarecrow.svg";
                } else {
                    img_scarecrow.src = "./images/scarecrow2.svg";
                }
                img_scarecrow.onload = function () {
                    context.drawImage(img_scarecrow, x, y, tileSize + 1, tileSize + 1);
                }
            } else if (tile.content.JollyBlock && tile.content.JollyBlock >= 1) {
                let img_jollyblock = new Image();
                img_jollyblock.src = "./images/jollyblock.svg";
                img_jollyblock.onload = function () {
                    context.drawImage(img_jollyblock, x, y, tileSize + 1, tileSize + 1);
                }
            }
        }
    }
    else{
        console.log(row + " " + col + " " + world.tiles[row][col]);
    }
}

function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}