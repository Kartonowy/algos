var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
var width = 150;
var height = 100;
//change to true if you want Highlife variation of game of life
var HIGHLIFE = true;
function generateMap() {
    return __spreadArray([], Array(height), true).map(function (e) { return e = __spreadArray([], Array(width), true).map(function (cell) { return cell = Math.random() > 0.5 ? 1 : 0; }); });
}
function printMap(map) {
    // you can add any character you like
    var alive = "#";
    var dead = ".";
    for (var _i = 0, map_1 = map; _i < map_1.length; _i++) {
        var row = map_1[_i];
        console.log(row.map(function (e) {
            if (Number(e)) {
                return alive[Math.floor(Math.random() * alive.length)];
            }
            return dead;
        }).join(""));
    }
}
function printAdjMap(map) {
    for (var _i = 0, map_2 = map; _i < map_2.length; _i++) {
        var row = map_2[_i];
        console.log(row.join(""));
    }
}
function countAdjecent(map) {
    var alt = __spreadArray([], Array(height), true).map(function (e) { return e = __spreadArray([], Array(width), true); });
    for (var i = 0; i < height; i++) {
        for (var j = 0; j < width; j++) {
            var adj = 0;
            if (map[i - 1] && Number(map[i - 1][j - 1]))
                adj++;
            if (map[i - 1] && Number(map[i - 1][j]))
                adj++;
            if (map[i - 1] && Number(map[i - 1][j + 1]))
                adj++;
            if (Number(map[i][j - 1]))
                adj++;
            if (Number(map[i][j + 1]))
                adj++;
            if (map[i + 1] && Number(map[i + 1][j - 1]))
                adj++;
            if (map[i + 1] && Number(map[i + 1][j]))
                adj++;
            if (map[i + 1] && Number(map[i + 1][j + 1]))
                adj++;
            alt[i][j] = adj;
        }
    }
    return alt;
}
function gameLoop(arena) {
    setInterval(function () {
        var neighbourhoods = countAdjecent(arena);
        for (var i = 0; i < height; i++) {
            for (var j = 0; j < width; j++) {
                if (HIGHLIFE) {
                    if (arena[i][j] === 0 && (neighbourhoods[i][j] === 3 || neighbourhoods[i][j] === 6)) {
                        arena[i][j] = 1;
                    }
                    else {
                        if (neighbourhoods[i][j] < 2 || (neighbourhoods[i][j] > 3 && neighbourhoods[i][j] < 6) || neighbourhoods[i][j] > 6) {
                            arena[i][j] = 0;
                        }
                        else if (neighbourhoods[i][j] in [2, 3]) {
                            continue;
                        }
                    }
                }
                else {
                    if (arena[i][j] === 0 && neighbourhoods[i][j] === 3) {
                        arena[i][j] = 1;
                    }
                    else {
                        if (neighbourhoods[i][j] < 2 || neighbourhoods[i][j] > 3) {
                            arena[i][j] = 0;
                        }
                        else if (neighbourhoods[i][j] in [2, 3]) {
                            continue;
                        }
                    }
                }
            }
        }
        console.log('\x1b[2J');
        printMap(arena);
    }, 300);
}
var arena = generateMap();
gameLoop(arena);
