import sys
from pathlib import Path
from typing import Optional

import h5py
from scipy.spatial import Voronoi
import numpy as np


class Grid:
    def __init__(self, cells, points):
        self.cells = cells
        self.points = points

    def connect(self, cell1, cell2):
        cell1.addNeighbour(cell2)
        cell2.addNeighbour(cell1)


class Domain:
    def __init__(self, xRange, yRange, zRange):
        self.xRange = xRange
        self.yRange = yRange
        self.zRange = zRange


class Cell:
    def __init__(self, pos: np.ndarray, processor: int):
        self.pos = pos
        self.neighbours = []
        self.processor = int(processor)

    def __repr__(self):
        return "{}@{}".format(self.pos)

    def addNeighbour(self, cell):
        self.neighbours.append(cell)


def getVoronoiGrid(points: np.ndarray, tasks: np.ndarray, domain: Domain):
    numPoints = len(points)
    cells = []
    for (point, task) in zip(points, tasks):
        cells.append(Cell(point, task))
    voronoiGrid = Voronoi(points)
    grid = Grid(cells, points)
    for (i, cell) in enumerate(grid.cells):
        cell.name = i
    for ridge in voronoiGrid.ridge_points:
        if ridge[0] < numPoints and ridge[1] < numPoints:
            grid.connect(cells[ridge[0]], cells[ridge[1]])
    return grid


def getVertices(voronoiGrid, region, domain):
    vertices = []
    center = sum(voronoiGrid.vertices[i] for i in region if i != -1)
    for i in region:
        if i != -1:
            vertices.append(voronoiGrid.vertices[i])
        else:
            vertices.append(closestCorner(center, domain))
    return vertices


def closestCorner(pos, domain):
    x = snap(pos[0], domain.xRange[0], domain.xRange[1])
    y = snap(pos[1], domain.yRange[0], domain.yRange[1])
    return np.array([x, y])


def snap(value, minValue, maxValue):
    if value < (minValue + maxValue) / 2:
        return minValue
    else:
        return maxValue


def readFile(filename: Path):
    with h5py.File(filename, "r") as f:
        boxSize = f["Parameters"].attrs["BoxSize"]
        coordinates = readDataset(f, f["PartType0"]["Coordinates"])
        tasks = readDataset(f, f["PartType0"]["task"])
        return boxSize, coordinates, tasks

def readDataset(f, dataset):
    data = np.zeros(dataset.shape)
    dataset.read_direct(data)
    return data


def writeGridToFile(filename: str, grid: Grid):
    def writeCell(cell):
        formattedNeighbours = " ".join(str(neighbour.name) for neighbour in cell.neighbours)
        return "{} {} {} {} {} {}".format(cell.name, cell.processor, cell.pos[0], cell.pos[1], cell.pos[2], formattedNeighbours)

    content = "\n".join(writeCell(cell) for cell in grid.cells)
    with open(filename, "w") as f:
        f.write(content)


def main():
    filename = sys.argv[1]
    boxSize, coordinates, tasks = readFile(filename)
    domain = Domain((0, boxSize), (0, boxSize), (0, boxSize))
    grid = getVoronoiGrid(coordinates, tasks, domain)
    outFile = filename.replace("hdf5", "dat")
    writeGridToFile(outFile, grid)


main()
