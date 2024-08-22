const width = 150;
const height = 100;

//change to true if you want Highlife variation of game of life
const HIGHLIFE = true;

function generateMap(): Array<Array<number>> {

    return [...Array<Array<number>>(height)].map((e) => e = [...Array(width)].map(cell => cell = Math.random() > 0.5 ? 1 : 0));
}

function printMap(map: Array<Array<number>>) {
    // you can add any character you like
    const alive = "#"
    const dead = ".";
    for (const row of map) {
        console.log(row.map(e => {
            if (Number(e)) {
                return alive[Math.floor(Math.random() * alive.length)]
            }
            return dead
        }).join(""))
    }
}

function printAdjMap(map: Array<Array<number>>) {
    for (const row of map) {
        console.log(row.join(""))
    }
}

function countAdjecent(map: Array<Array<number>>): Array<Array<number>> {
    const alt = [...Array<Array<number>>(height)].map((e) => e = [...Array(width)]);
    for (let i = 0; i < height; i++) {
        for (let j = 0; j < width; j++) {
            let adj = 0;
            if (map[i - 1] && Number(map[i - 1][j - 1])) adj++;
            if (map[i - 1] && Number(map[i - 1][j])) adj++;
            if (map[i - 1] && Number(map[i - 1][j + 1])) adj++;
            if (Number(map[i][j - 1])) adj++;
            if (Number(map[i][j + 1])) adj++;
            if (map[i + 1] && Number(map[i + 1][j - 1])) adj++;
            if (map[i + 1] && Number(map[i + 1][j])) adj++;
            if (map[i + 1] && Number(map[i + 1][j + 1])) adj++;
            alt[i][j] = adj
        }
    }

    return alt;
}

function gameLoop(arena: Array<Array<number>>) {
    setInterval(() => {
        let neighbourhoods = countAdjecent(arena);

        for (let i = 0; i < height; i++) {
            for (let j = 0; j < width; j++) {
                if (HIGHLIFE) {
                    if (arena[i][j] === 0 && (neighbourhoods[i][j] === 3 || neighbourhoods[i][j] === 6)) {
                        arena[i][j] = 1
                    } else {
                        if (neighbourhoods[i][j] < 2 || (neighbourhoods[i][j] > 3 && neighbourhoods[i][j] < 6) || neighbourhoods[i][j] > 6) {
                            arena[i][j] = 0
                        } else if (neighbourhoods[i][j] in [2, 3]) {
                            continue;
                        }
                    }
                } else {
                    if (arena[i][j] === 0 && neighbourhoods[i][j] === 3) {
                        arena[i][j] = 1
                    } else {
                        if (neighbourhoods[i][j] < 2 || neighbourhoods[i][j] > 3) {
                            arena[i][j] = 0
                        } else if (neighbourhoods[i][j] in [2, 3]) {
                            continue;
                        }
                    }
                }

            }

        }
        console.log('\x1b[2J');
        printMap(arena)
    }, 300)
}

let arena = generateMap()
gameLoop(arena)