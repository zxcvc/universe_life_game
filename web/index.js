import { Universe, BitItem, BitMap } from 'universe_life_game';

const ROW = 60;
const COL = 60;
const CELL_SIZE = 10;
const CELL_ALIVE_COLOR = 'black';
const CELL_DEAD_COLOR = 'white';

const universe = Universe.new(COL, ROW);

const canvas = document.querySelector('#cavs');
canvas.width = (ROW) * CELL_SIZE;
canvas.height = (COL + 2) * CELL_SIZE;
const ctx = canvas.getContext('2d');

function draw_grid() {
  ctx.beginPath();
  ctx.strokeStyle = '#ccc';
  for (let i = 0; i <= ROW; ++i) {
    ctx.moveTo((CELL_SIZE) * i, 0);
    ctx.lineTo((CELL_SIZE) * i, COL * CELL_SIZE);
  }
  for (let i = 0; i <= COL; ++i) {
    ctx.moveTo(0, (CELL_SIZE) * i);
    ctx.lineTo((CELL_SIZE) * ROW, CELL_SIZE * i);
  }

  ctx.stroke();
}

function draw_cell() {
  // const cell_ptr = universe.cell_ptr()
  // const cell = new Uint8Array(memory.buffer, cell_ptr, WIDTH * HEIGHT)

  ctx.beginPath();
  ctx.fillStyle = CELL_ALIVE_COLOR;
  for (let row = 0; row < ROW; ++row) {
    for (let col = 0; col < COL; ++col) {
      if (universe.get_index(row, col) === BitItem.ZERO) continue;
      const idx = universe.get_index(row, col);
      ctx.fillRect(CELL_SIZE * row + 1, CELL_SIZE * col + 1, CELL_SIZE - 1, CELL_SIZE - 1);
    }
  }

  ctx.fillStyle = CELL_DEAD_COLOR;
  for (let row = 0; row < ROW; ++row) {
    for (let col = 0; col < COL; ++col) {
      if (universe.get_index(row, col) === BitItem.ONE) continue;
      const idx = universe.get_index(row, col);
      ctx.fillRect(CELL_SIZE * row + 1, CELL_SIZE * col + 1, CELL_SIZE - 1, CELL_SIZE - 1);
    }
  }

  ctx.stroke();
}

draw_grid();

let handler = null;

function tick() {
  draw_cell();
  //   const d = performance.now();
  universe.tick();
  //   console.log(performance.now() - d);
  handler = requestAnimationFrame(tick);
}

requestAnimationFrame(tick);

// requestAnimationFrame(draw_cell)
// handler = tick();
document.querySelector('button').addEventListener('click', () => {
  if (handler === null) {
    handler = tick();
  } else {
    cancelAnimationFrame(handler);
    handler = null;
  }
  draw_cell();
});

canvas.addEventListener('click', (e) => {
  if (handler !== null) return;
  cancelAnimationFrame(handler);
  handler = null;
  const row = Math.floor(e.offsetX / CELL_SIZE);
  const col = Math.floor(e.offsetY / CELL_SIZE);
  universe.toggle(row, col);
  draw_cell();
});

document.querySelector('#reset').addEventListener('click', () => {
  universe.reset();
  draw_cell();
});
