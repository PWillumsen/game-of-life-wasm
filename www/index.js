// import * as wasm from "wasm-game-of-life";
import { Universe} from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 7; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

let universe = Universe.new();
const width = universe.getWidth();
const height = universe.getHeight();

const canvas = document.getElementById("game-of-life-canvas");
const playPauseButton = document.getElementById("play-pause");
const resetButton = document.getElementById("reset-board");
const clearButton = document.getElementById("clear-board");
playPauseButton.textContent = "▶";
clearButton.textContent = "⎚";
resetButton.textContent = "↻";

const ctx = canvas.getContext('2d');

canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

let animationId = null;

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const isPaused = () => {
  return animationId === null;
};

const getIndex = (row, column) => {
  return row * width + column;
};

playPauseButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

resetButton.addEventListener("click", event => {
  universe = Universe.new();
  drawCells();

});

clearButton.addEventListener("click", event => {
  universe.clear();
  drawCells();
}
);

// const renderLoop = () => {
//   drawGrid();
//   drawCells();

//   universe.tick();

//   animationId = requestAnimationFrame(renderLoop);
// };

canvas.addEventListener("click", event => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  universe.toggle_cell(row, col);

  drawGrid();
  drawCells();
});


const fps = new class {
  constructor() {
    this.fps = document.getElementById("fps");
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = 1 / delta * 1000;

    // Save only the latest 100 timings.
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    // Find the max, min, and mean of our 100 latest timings.
    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    let mean = sum / this.frames.length;

    // Render the statistics.
    this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
  }
};

const renderLoop = () => {
  fps.render(); //new

  universe.tick();
  drawGrid();
  drawCells();

  animationId = requestAnimationFrame(renderLoop);
  pause();
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const drawCells = () => {
  const cellsAlivePtr = universe.getNewAlive();
  const cellsAliveLen = universe.getAliveLen(); 
  const cellsAlive = new Uint32Array(memory.buffer, cellsAlivePtr, cellsAliveLen);
  
  const cellsDeadPtr = universe.getNewDead();
  const cellsDeadLen = universe.getDeadLen(); 
  const cellsDead = new Uint32Array(memory.buffer, cellsDeadPtr , cellsDeadLen);
  
  ctx.beginPath();

  ctx.fillStyle = ALIVE_COLOR;
  for (let idx = 0; idx < cellsAliveLen; idx = idx + 2) {
    const row = cellsAlive[idx];
    const col = cellsAlive[idx+1];
      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
  }
  

  ctx.fillStyle = DEAD_COLOR;
  for (let idx = 0; idx < cellsDeadLen; idx = idx + 2) {
    const row = cellsDead[idx];
    const col = cellsDead[idx+1];
      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
  }
  ctx.stroke();
};

drawGrid();
drawCells();
play();
