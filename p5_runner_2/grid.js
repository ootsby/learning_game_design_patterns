class Grid {
  constructor(cellSize = 100) {
    this.horizontileCount = Math.floor(width / cellSize);
    this.verticalCount = Math.floor(height / cellSize);
    this.cellWidth = cellSize;
    this.cellHeight = cellSize;
    this.cells = [];

    for (let yCount = 0; yCount < this.verticalCount; yCount += 1) {
      const yCells = [];
      for (let xCount = 0; xCount < this.horizontileCount; xCount += 1) {
        yCells.push([]);
      }
      this.cells.push(yCells);
    }
    this.lineColor = color(255, 255, 255, 100);
  }

  drawGrid() {
    stroke(this.lineColor);
    this.cells.forEach((xCells, yIndex) => {
      line(0, yIndex * this.cellHeight, width, yIndex * this.cellHeight);
      xCells.forEach((cell, xIndex) => {
        line(xIndex * this.cellWidth, 0, xIndex * this.cellWidth, height);
      });
    });
  }

  add(gameObject) {
    const xIndex = Math.floor(gameObject.centerLocation.x / this.cellWidth);
    const yIndex = Math.floor(gameObject.centerLocation.y / this.cellHeight);
    this.cells[yIndex][xIndex].push(GameObject);
  }

  draw() {
    this.cells[5][1].forEach((gameObject) => gameObject.draw());
  }
}
