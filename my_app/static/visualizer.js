var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var worldData; //
var world; //
var left;
var right;
var sopra;
var sotto;
var gap;
var v_content = true;
var stop_mode;
var first_request = true;
var energy = 1000;
var i_robot;
var j_robot;
var cont_zoom_in = 0;
var colors = {
    Land: {
        Grass: "#52ff3e",
        Grass2: "#3fce31",
        Hill: "#017001",
        Hill2: "#015001",
        Mountain: "#964b00",
        Mountain2: "#854202",
        Lava: "#e74045",
        Snow: "#ffffff",
        Snow2: "#eeeeee",
        Sand: "#fcdd75",
        Sand2: "#e5c864",
        DeepWater: "#0030d3",
        DeepWater2: "#0023b2",
        ShallowWater: "#40daf5",
        ShallowWater2: "#49b1f1",
        Unknown: "#000000"
    }
};
document.addEventListener('keydown', function (event) {
    if ((event.key === '-' || event.key === 'm') && document.getElementById("zoom_out").disabled === false) {
        document.getElementById("zoom_out").click();
    }
});
document.addEventListener('keydown', function (event) {
    if ((event.key === '+' || event.key === 'p') && document.getElementById("zoom_in").disabled === false) {
        document.getElementById("zoom_in").click();
    }
});
document.addEventListener('keydown', function (event) {
    if (event.key === 'w' || event.key === "ArrowUp") {
        document.getElementById("go_up").click();
    }
});
document.addEventListener('keydown', function (event) {
    if (event.key === 's' || event.key === "ArrowDown") {
        document.getElementById("go_down").click();
    }
});
document.addEventListener('keydown', function (event) {
    if (event.key === 'd' || event.key === "ArrowRight") {
        document.getElementById("go_right").click();
    }
});
document.addEventListener('keydown', function (event) {
    if (event.key === 'a' || event.key === "ArrowLeft") {
        document.getElementById("go_left").click();
    }
});
document.addEventListener('keydown', function (event) {
    if (event.key === 'c') {
        if (event.repeat) {
            return;
        }
        document.getElementById("view_content").click();
    }
});
function disable_button(button_id) {
    var button = document.getElementById(button_id);
    button.disabled = true;
    button.style.color = 'Grey';
    button.style.borderColor = 'Grey';
}
function enable_button(button_id) {
    var button = document.getElementById(button_id);
    button.disabled = false;
    button.style.color = 'White';
    button.style.borderColor = 'White';
}
function enable_auto_mode() {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    stop_mode = false;
                    enable_button("stop_auto");
                    disable_button("automatic_tick_processing");
                    disable_button("next_tick");
                    disable_button("zoom_in");
                    disable_button("zoom_out");
                    return [4 /*yield*/, process_next_tick()];
                case 1:
                    _a.sent();
                    disable_button("automatic_tick_processing");
                    disable_button("next_tick");
                    disable_button("zoom_in");
                    disable_button("zoom_out");
                    return [4 /*yield*/, sleep(2000)];
                case 2:
                    _a.sent();
                    if (stop_mode === false) {
                        enable_auto_mode();
                    }
                    else {
                        enable_button("automatic_tick_processing");
                        enable_button("next_tick");
                        enable_button("zoom_in");
                        enable_button("zoom_out");
                    }
                    return [2 /*return*/];
            }
        });
    });
}
function stop_auto_mode() {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    stop_mode = true;
                    disable_button("stop_auto");
                    return [4 /*yield*/, sleep(2000)];
                case 1:
                    _a.sent();
                    return [2 /*return*/];
            }
        });
    });
}
function process_next_tick() {
    return __awaiter(this, void 0, void 0, function () {
        var was_previusly_enabled;
        var _this = this;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    was_previusly_enabled = !(document.getElementById("automatic_tick_processing").disabled);
                    disable_button("automatic_tick_processing");
                    disable_button("next_tick");
                    disable_button("zoom_in");
                    disable_button("zoom_out");
                    return [4 /*yield*/, fetch('http://127.0.0.1:8000/get_robot_data')
                            .then(function (response) { return response.json(); })
                            .then(function (data) { return __awaiter(_this, void 0, void 0, function () {
                            return __generator(this, function (_a) {
                                switch (_a.label) {
                                    case 0:
                                        worldData = {
                                            size: data.world_size,
                                            tiles: data.world
                                        };
                                        if (!(first_request === true)) return [3 /*break*/, 3];
                                        world = worldData;
                                        left = 0;
                                        right = worldData.size;
                                        sopra = 0;
                                        sotto = worldData.size;
                                        gap = worldData.size / 2;
                                        first_request = false;
                                        i_robot = data.positions[0];
                                        j_robot = data.positions[1];
                                        return [4 /*yield*/, make_animation(world, data.ser_events)];
                                    case 1:
                                        _a.sent();
                                        update_backpack(data.backpack);
                                        update_points(data.points);
                                        update_energy(data.energy);
                                        energy = data.energy;
                                        worldData.tiles[i_robot][j_robot].robot = true;
                                        return [4 /*yield*/, drawWorld(worldData)];
                                    case 2:
                                        _a.sent();
                                        return [3 /*break*/, 6];
                                    case 3: return [4 /*yield*/, make_animation(world, data.ser_events)];
                                    case 4:
                                        _a.sent();
                                        update_backpack(data.backpack);
                                        update_points(data.points);
                                        update_energy(data.energy);
                                        energy = data.energy;
                                        i_robot = data.positions[0];
                                        j_robot = data.positions[1];
                                        worldData.tiles[i_robot][j_robot].robot = true;
                                        return [4 /*yield*/, updateWorld(world)];
                                    case 5:
                                        _a.sent();
                                        world = worldData;
                                        _a.label = 6;
                                    case 6: return [2 /*return*/];
                                }
                            });
                        }); })["catch"](function (error) {
                            console.error('Error fetching data:', error);
                        })];
                case 1:
                    _a.sent();
                    enable_button("automatic_tick_processing");
                    enable_button("next_tick");
                    enable_button("zoom_in");
                    enable_button("zoom_out");
                    return [2 /*return*/];
            }
        });
    });
}
function make_animation(world, events) {
    return __awaiter(this, void 0, void 0, function () {
        var i, event_1, tile, new_i_robot, new_j_robot, tile, i_pos, j_pos, item, quantity, item, quantity, hour, minute, weather;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    i = 0;
                    _a.label = 1;
                case 1:
                    if (!(i < events.length)) return [3 /*break*/, 18];
                    event_1 = events[i];
                    if (!event_1.EnergyConsumed) return [3 /*break*/, 3];
                    energy = energy - event_1.EnergyConsumed;
                    update_energy(energy);
                    return [4 /*yield*/, sleep(10)];
                case 2:
                    _a.sent();
                    return [3 /*break*/, 17];
                case 3:
                    if (!event_1.EnergyRecharged) return [3 /*break*/, 5];
                    energy = energy + event_1.EnergyRecharged;
                    update_energy(energy);
                    return [4 /*yield*/, sleep(10)];
                case 4:
                    _a.sent();
                    return [3 /*break*/, 17];
                case 5:
                    if (!event_1.Moved) return [3 /*break*/, 8];
                    tile = event_1.Moved[0];
                    new_i_robot = event_1.Moved[1][0];
                    new_j_robot = event_1.Moved[1][1];
                    return [4 /*yield*/, update_robot_position(i_robot, j_robot, new_i_robot, new_j_robot, tile)];
                case 6:
                    _a.sent();
                    i_robot = new_i_robot;
                    j_robot = new_j_robot;
                    return [4 /*yield*/, sleep(50)];
                case 7:
                    _a.sent();
                    return [3 /*break*/, 17];
                case 8:
                    if (!event_1.TileContentUpdated) return [3 /*break*/, 11];
                    tile = event_1.TileContentUpdated[0];
                    i_pos = event_1.TileContentUpdated[1][0];
                    j_pos = event_1.TileContentUpdated[1][1];
                    return [4 /*yield*/, update_tile(i_pos, j_pos, tile)];
                case 9:
                    _a.sent();
                    return [4 /*yield*/, sleep(50)];
                case 10:
                    _a.sent();
                    return [3 /*break*/, 17];
                case 11:
                    if (!event_1.AddedToBackpack) return [3 /*break*/, 13];
                    item = event_1.AddedToBackpack[0];
                    quantity = event_1.AddedToBackpack[1];
                    update_backpack_item(item, quantity, true);
                    return [4 /*yield*/, sleep(10)];
                case 12:
                    _a.sent();
                    return [3 /*break*/, 17];
                case 13:
                    if (!event_1.RemovedFromBackpack) return [3 /*break*/, 15];
                    item = event_1.RemovedFromBackpack[0];
                    quantity = event_1.RemovedFromBackpack[1];
                    update_backpack_item(item, quantity, false);
                    return [4 /*yield*/, sleep(10)];
                case 14:
                    _a.sent();
                    return [3 /*break*/, 17];
                case 15:
                    if (!event_1.TimeChanged) return [3 /*break*/, 17];
                    hour = event_1.TimeChanged.time_of_day.hour;
                    minute = event_1.TimeChanged.time_of_day.minute;
                    weather = event_1.TimeChanged.weather_forecast[0];
                    update_time(hour, minute);
                    update_weather(weather);
                    return [4 /*yield*/, sleep(10)];
                case 16:
                    _a.sent();
                    _a.label = 17;
                case 17:
                    i++;
                    return [3 /*break*/, 1];
                case 18: return [2 /*return*/];
            }
        });
    });
}
function update_time(hour, minute) {
    var elem = document.getElementById("update_time");
    elem.textContent = hour + ":" + minute;
}
function update_weather(weather) {
    var elem = document.getElementById("update_weather");
    elem.src = "./images/weather_" + weather.toLowerCase() + ".jpg";
}
function update_item(item, quantity, insert) {
    var elem = document.getElementById("update_" + item);
    if (insert === true) {
        elem.textContent = (parseInt(elem.textContent) + quantity).toString();
        ;
    }
    else {
        elem.textContent = (parseInt(elem.textContent) - quantity).toString();
        ;
    }
}
function update_backpack_item(item, quantity, insert) {
    if ("Tree" in item) {
        update_item("Tree", quantity, insert);
    }
    else if ("Rock" in item) {
        update_item("Rock", quantity, insert);
    }
    else if ("Garbage" in item) {
        update_item("Garbage", quantity, insert);
    }
    else if ("Fire" in item) {
        update_item("Fire", quantity, insert);
    }
    else if ("Coin" in item) {
        update_item("Coin", quantity, insert);
    }
    else if ("Water" in item) {
        update_item("Water", quantity, insert);
    }
    else if ("Fish" in item) {
        update_item("Fish", quantity, insert);
    }
    else if ("Bush" in item) {
        update_item("Bush", quantity, insert);
    }
    else if ("JollyBlock" in item) {
        update_item("JollyBlock", quantity, insert);
    }
}
function update_robot_position(i_robot, j_robot, new_i_robot, new_j_robot, tile) {
    return __awaiter(this, void 0, void 0, function () {
        var canvas, context, tileSize;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    canvas = document.getElementById("worldCanvas");
                    context = canvas.getContext("2d");
                    tileSize = canvas.width / world.size;
                    world.tiles[i_robot][j_robot].robot = false;
                    world.tiles[new_i_robot][new_j_robot] = tile;
                    world.tiles[new_i_robot][new_j_robot].robot = true;
                    return [4 /*yield*/, draw_single_cell(world, i_robot, j_robot, context, tileSize)];
                case 1:
                    _a.sent();
                    return [4 /*yield*/, draw_single_cell(world, new_i_robot, new_j_robot, context, tileSize)];
                case 2:
                    _a.sent();
                    return [2 /*return*/];
            }
        });
    });
}
function update_tile(i, j, tile) {
    return __awaiter(this, void 0, void 0, function () {
        var canvas, context, tileSize;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    canvas = document.getElementById("worldCanvas");
                    context = canvas.getContext("2d");
                    tileSize = canvas.width / world.size;
                    world.tiles[i][j] = tile;
                    return [4 /*yield*/, draw_single_cell(world, i, j, context, tileSize)];
                case 1:
                    _a.sent();
                    return [2 /*return*/];
            }
        });
    });
}
function update_backpack(backpack) {
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
function update_points(points) {
    var spanElement = document.getElementById("update_points");
    spanElement.textContent = points.toString();
}
function update_energy(energy) {
    var spanElement = document.getElementById("update_energy");
    if (energy > 1000) {
        energy = 1000;
    }
    if (energy < 0) {
        energy = 0;
    }
    spanElement.textContent = energy.toString();
}
function zoom_out() {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    if (!(world.size * 2 <= worldData.size)) return [3 /*break*/, 2];
                    cont_zoom_in = cont_zoom_in - 1;
                    if (cont_zoom_in <= 0) {
                        enable_button("automatic_tick_processing");
                        enable_button("next_tick");
                        cont_zoom_in = 0;
                    }
                    world = {
                        size: world.size * 2,
                        tiles: world.tiles
                    };
                    gap = gap * 2;
                    left = 0;
                    right = worldData.size;
                    sopra = 0;
                    sotto = worldData.size;
                    return [4 /*yield*/, drawWorld(world)];
                case 1:
                    _a.sent();
                    _a.label = 2;
                case 2: return [2 /*return*/];
            }
        });
    });
}
function zoom_in() {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    if (!(world.size / 2 >= 1)) return [3 /*break*/, 2];
                    cont_zoom_in = cont_zoom_in + 1;
                    disable_button("automatic_tick_processing");
                    disable_button("next_tick");
                    world = {
                        size: Math.floor(world.size / 2),
                        tiles: world.tiles
                    };
                    gap = Math.floor(gap / 2);
                    if (gap === 0) {
                        gap = 1;
                    }
                    right = Math.floor(right / 2);
                    sotto = Math.floor(sotto / 2);
                    return [4 /*yield*/, drawWorld(world)];
                case 1:
                    _a.sent();
                    _a.label = 2;
                case 2: return [2 /*return*/];
            }
        });
    });
}
function go_right() {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    if (!(right + gap <= worldData.size)) return [3 /*break*/, 2];
                    right += gap;
                    left += gap;
                    return [4 /*yield*/, drawWorld(world)];
                case 1:
                    _a.sent();
                    _a.label = 2;
                case 2: return [2 /*return*/];
            }
        });
    });
}
function go_left() {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    if (!(left - gap >= 0)) return [3 /*break*/, 2];
                    right -= gap;
                    left -= gap;
                    return [4 /*yield*/, drawWorld(world)];
                case 1:
                    _a.sent();
                    _a.label = 2;
                case 2: return [2 /*return*/];
            }
        });
    });
}
function go_down() {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    if (!(sotto + gap <= worldData.size)) return [3 /*break*/, 2];
                    sotto += gap;
                    sopra += gap;
                    return [4 /*yield*/, drawWorld(world)];
                case 1:
                    _a.sent();
                    _a.label = 2;
                case 2: return [2 /*return*/];
            }
        });
    });
}
function go_up() {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    if (!(sopra - gap >= 0)) return [3 /*break*/, 2];
                    sotto -= gap;
                    sopra -= gap;
                    return [4 /*yield*/, drawWorld(world)];
                case 1:
                    _a.sent();
                    _a.label = 2;
                case 2: return [2 /*return*/];
            }
        });
    });
}
function view_content() {
    return __awaiter(this, void 0, void 0, function () {
        var tmpWorld;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    v_content = !v_content;
                    tmpWorld = {
                        size: world.size,
                        tiles: worldData.tiles
                    };
                    return [4 /*yield*/, drawWorld(tmpWorld)];
                case 1:
                    _a.sent();
                    return [2 /*return*/];
            }
        });
    });
}
function display_tiletype(tile, context, colors) {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            if (tile) {
                //console.log(tile.elevation - leftile.elevation);
                if (tile.tile_type === "Grass") {
                    context.fillStyle = colors.Grass2;
                }
                else if (tile.tile_type === "Sand") {
                    context.fillStyle = colors.Sand2;
                }
                else if (tile.tile_type === "Hill") {
                    context.fillStyle = colors.Hill2;
                }
                else if (tile.tile_type === "Mountain") {
                    context.fillStyle = colors.Mountain2;
                }
                else if (tile.tile_type === "Snow") {
                    context.fillStyle = colors.Snow2;
                }
                else if (tile.tile_type === "ShallowWater") {
                    context.fillStyle = colors.ShallowWater2;
                }
                else if (tile.tile_type === "DeepWater") {
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
                return [2 /*return*/, true];
            }
            else {
                // console.log("One or both tiles are null or do not have elevation property.");
                // Handle the situation where one or both tiles are null or do not have elevation property.
                return [2 /*return*/, false];
            }
            return [2 /*return*/];
        });
    });
}
function drawWorld(world) {
    return __awaiter(this, void 0, void 0, function () {
        var canvas, context, tileSize, row, col;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    canvas = document.getElementById("worldCanvas");
                    context = canvas.getContext("2d");
                    context.clearRect(0, 0, canvas.width, canvas.height);
                    tileSize = canvas.width / world.size;
                    if (!context) return [3 /*break*/, 6];
                    row = 0;
                    _a.label = 1;
                case 1:
                    if (!(row < world.size)) return [3 /*break*/, 6];
                    col = 0;
                    _a.label = 2;
                case 2:
                    if (!(col < world.size)) return [3 /*break*/, 5];
                    return [4 /*yield*/, draw_single_cell(world, row, col, context, tileSize)];
                case 3:
                    _a.sent();
                    _a.label = 4;
                case 4:
                    col++;
                    return [3 /*break*/, 2];
                case 5:
                    row++;
                    return [3 /*break*/, 1];
                case 6: return [2 /*return*/];
            }
        });
    });
}
function updateWorld(world) {
    return __awaiter(this, void 0, void 0, function () {
        var canvas, context, tileSize, row, col;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    canvas = document.getElementById("worldCanvas");
                    context = canvas.getContext("2d");
                    tileSize = canvas.width / world.size;
                    if (!context) return [3 /*break*/, 6];
                    row = 0;
                    _a.label = 1;
                case 1:
                    if (!(row < world.size)) return [3 /*break*/, 6];
                    col = 0;
                    _a.label = 2;
                case 2:
                    if (!(col < world.size)) return [3 /*break*/, 5];
                    if (!(JSON.stringify(world.tiles[row][col]) !== JSON.stringify(worldData.tiles[row][col]))) return [3 /*break*/, 4];
                    return [4 /*yield*/, draw_single_cell(worldData, row, col, context, tileSize)];
                case 3:
                    _a.sent();
                    _a.label = 4;
                case 4:
                    col++;
                    return [3 /*break*/, 2];
                case 5:
                    row++;
                    return [3 /*break*/, 1];
                case 6: return [2 /*return*/];
            }
        });
    });
}
function draw_single_cell(world, row, col, context, tileSize) {
    var _a, _b, _c;
    return __awaiter(this, void 0, void 0, function () {
        var tile, flag_robot, x_1, y_1, res, img_teleport_1, jpg_lava_1, img_robot_1, img_tree_1, img_tree_2, img_fire_1, img_rock_1, img_garbage_1, img_fish_1, img_coin_1, img_bin_1, img_bank_1, img_market_1, img_crate_1, img_building_1, img_scarecrow_1, img_jollyblock_1;
        return __generator(this, function (_d) {
            switch (_d.label) {
                case 0:
                    if (!(world.tiles && world.tiles.length > 0 &&
                        world.tiles[row + sopra] && world.tiles[row + sopra].length > 0 &&
                        world.tiles[row + sopra][col + left])) return [3 /*break*/, 2];
                    tile = world.tiles[row + sopra][col + left];
                    flag_robot = false;
                    x_1 = col * tileSize;
                    y_1 = row * tileSize;
                    return [4 /*yield*/, display_tiletype(tile, context, colors.Land)];
                case 1:
                    res = _d.sent();
                    if (tile.tile_type.Teleport && tile.tile_type.Teleport === true) {
                        img_teleport_1 = new Image();
                        img_teleport_1.src = "./images/teleport.png";
                        img_teleport_1.onload = function () {
                            context.drawImage(img_teleport_1, x_1, y_1, tileSize + 1, tileSize + 1);
                        };
                        res = true;
                    }
                    else if (tile.tile_type === "Lava") {
                        jpg_lava_1 = new Image();
                        jpg_lava_1.src = "./images/lava.jpg";
                        jpg_lava_1.onload = function () {
                            context.drawImage(jpg_lava_1, x_1, y_1, tileSize + 1, tileSize + 1);
                        };
                        res = true;
                    }
                    if (tile.robot && tile.robot === true) {
                        img_robot_1 = new Image();
                        img_robot_1.src = "./images/robot.svg";
                        img_robot_1.onload = function () {
                            context.drawImage(img_robot_1, x_1, y_1, tileSize + 1, tileSize + 1);
                        };
                        flag_robot = true;
                    }
                    if (!res) {
                        return [2 /*return*/];
                    }
                    // Fill the tile
                    context.fillRect(x_1, y_1, tileSize + 1, tileSize + 1);
                    // Add additional visual representation for properties
                    if (tile.content !== "None" && v_content && !flag_robot) {
                        if (tile.content.Tree && tile.content.Tree >= 1) {
                            img_tree_1 = new Image();
                            img_tree_1.src = "./images/tree.svg";
                            img_tree_1.onload = function () {
                                context.drawImage(img_tree_1, x_1, y_1, tileSize + 1, tileSize + 1);
                            };
                        }
                        else if (tile.content.Bush && tile.content.Bush >= 1) {
                            img_tree_2 = new Image();
                            img_tree_2.src = "./images/bush.svg";
                            img_tree_2.onload = function () {
                                context.drawImage(img_tree_2, x_1, y_1, tileSize + 1, tileSize + 1);
                            };
                        }
                        else if (tile.content === "Fire") {
                            img_fire_1 = new Image();
                            img_fire_1.src = "./images/fire.svg";
                            img_fire_1.onload = function () {
                                context.drawImage(img_fire_1, x_1, y_1, tileSize + 1, tileSize + 1);
                            };
                        }
                        else if (tile.content.Rock && tile.content.Rock >= 1) {
                            img_rock_1 = new Image();
                            if (tile.tile_type === "Snow") {
                                img_rock_1.src = "./images/snow_rock.svg";
                            }
                            else {
                                img_rock_1.src = "./images/rock.svg";
                            }
                            img_rock_1.onload = function () {
                                context.drawImage(img_rock_1, x_1, y_1, tileSize + 1, tileSize + 1);
                            };
                        }
                        else if (tile.content.Garbage && tile.content.Garbage >= 1) {
                            img_garbage_1 = new Image();
                            img_garbage_1.src = "./images/garbage.svg";
                            img_garbage_1.onload = function () {
                                context.drawImage(img_garbage_1, x_1, y_1, tileSize + 1, tileSize + 1);
                            };
                        }
                        else if (tile.content.Fish && tile.content.Fish >= 1) {
                            img_fish_1 = new Image();
                            img_fish_1.src = "./images/fish.svg";
                            img_fish_1.onload = function () {
                                context.drawImage(img_fish_1, x_1, y_1, tileSize + 1, tileSize + 1);
                            };
                        }
                        else if (tile.content.Coin && tile.content.Coin >= 1) {
                            img_coin_1 = new Image();
                            if (tile.content.Coin < 5) {
                                img_coin_1.src = "./images/coin1.svg";
                            }
                            else if (tile.content.Coin < 10) {
                                img_coin_1.src = "./images/coin2.svg";
                            }
                            else {
                                img_coin_1.src = "./images/coin3.svg";
                            }
                            img_coin_1.onload = function () {
                                context.drawImage(img_coin_1, x_1, y_1, tileSize + 1, tileSize + 1);
                            };
                        }
                        else if (tile.content.Bin && ((_a = tile.content.Bin) === null || _a === void 0 ? void 0 : _a.start) >= 0) {
                            img_bin_1 = new Image();
                            img_bin_1.src = "./images/bin.svg";
                            img_bin_1.onload = function () {
                                context.drawImage(img_bin_1, x_1, y_1, tileSize + 1, tileSize + 1);
                            };
                        }
                        else if (tile.content.Bank && ((_b = tile.content.Bank) === null || _b === void 0 ? void 0 : _b.start) >= 0) {
                            img_bank_1 = new Image();
                            img_bank_1.src = "./images/bank.svg";
                            img_bank_1.onload = function () {
                                context.drawImage(img_bank_1, x_1, y_1, tileSize + 1, tileSize + 1);
                            };
                        }
                        else if (tile.content.Market && tile.content.Market >= 0) {
                            img_market_1 = new Image();
                            img_market_1.src = "./images/market.svg";
                            img_market_1.onload = function () {
                                context.drawImage(img_market_1, x_1, y_1, tileSize + 1, tileSize + 1);
                            };
                        }
                        else if (tile.content.Crate && ((_c = tile.content.Crate) === null || _c === void 0 ? void 0 : _c.start) >= 0) {
                            img_crate_1 = new Image();
                            img_crate_1.src = "./images/crate.svg";
                            img_crate_1.onload = function () {
                                context.drawImage(img_crate_1, x_1, y_1, tileSize + 1, tileSize + 1);
                            };
                        }
                        else if (tile.content === "Building") {
                            img_building_1 = new Image();
                            img_building_1.src = "./images/building.svg";
                            img_building_1.onload = function () {
                                context.drawImage(img_building_1, x_1, y_1, tileSize + 1, tileSize + 1);
                            };
                        }
                        else if (tile.content === "Scarecrow") {
                            img_scarecrow_1 = new Image();
                            if (tile.tile_type === "ShallowWater") {
                                img_scarecrow_1.src = "./images/scarecrow.svg";
                            }
                            else {
                                img_scarecrow_1.src = "./images/scarecrow2.svg";
                            }
                            img_scarecrow_1.onload = function () {
                                context.drawImage(img_scarecrow_1, x_1, y_1, tileSize + 1, tileSize + 1);
                            };
                        }
                        else if (tile.content.JollyBlock && tile.content.JollyBlock >= 1) {
                            img_jollyblock_1 = new Image();
                            img_jollyblock_1.src = "./images/jollyblock.svg";
                            img_jollyblock_1.onload = function () {
                                context.drawImage(img_jollyblock_1, x_1, y_1, tileSize + 1, tileSize + 1);
                            };
                        }
                    }
                    return [3 /*break*/, 3];
                case 2:
                    console.log(row + " " + col + " " + world.tiles[row][col]);
                    _d.label = 3;
                case 3: return [2 /*return*/];
            }
        });
    });
}
function sleep(ms) {
    return new Promise(function (resolve) { return setTimeout(resolve, ms); });
}
