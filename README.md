# Closest Points Project

This project demonstrates an efficient algorithm for finding the closest points to a given point in three-dimensional space. The implementation uses a binary heap (specifically, a max heap) to optimize the process, allowing for quick identification of the k closest points.

## How to Use

Build and run the project: `cargo run`

The program will generate a random set of points and then find and print the 10 closest points to a randomly chosen test point.

## Project Structure

- `main.rs`: The main file containing the implementation and the entry point of the program.
- `Point`: A struct representing a point in three-dimensional space.
- `DistancePoint`: A struct storing the distance and corresponding point.
- `closest_points` function: Finds the k closest points to a given point using a binary heap.

## Performance Considerations

The project emphasizes performance by utilizing a binary heap (max heap) for efficiently maintaining the closest points. This allows the program to process a large number of points quickly.

## Results

The program measures the execution time and prints the 10 closest points to a randomly chosen test point. The results demonstrate the efficiency of the algorithm in finding the closest points.

## Complexity

- Time complexity:
  - The generation of the random points is O(n) where n is the number of points we want to generate.
  - The binary operation inside the loop is O(log k) where k is the size of the Binary Heap.
  - The overall time complexity is O(n * log k).

- Space complexity:
  - The space complexity for storing the points vector is O(n), where n is the number of points generated.
  - The space complexity for the binary heap is O(k), where k is the limit of closest points
  - The overall space complexity is O(n + k).