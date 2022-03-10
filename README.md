# Experimenting with VRP Solvers

[Google OR-Tools](#google-or-tools)

[Rust VRP Solver](#rust-vrp-solver)

# [Google OR-Tools](ortools)

<a href="https://developers.google.com/optimization/introduction/python">Get Started with OR-Tools for Python</a>

## Steps for TSP& VRP

1. Create the data

   - distance_matrix, len(distance_matrix)=num_locations; computed from location coordinates
   - num_vehicles,
   - depot (depot index)

2. Setup routing model with routing index manager

3. Define distance callback  
   Takes internal routing index and returns distance between 2 nodes

4. Set cost of travel

5. **If VRP**: Create <a href="https://developers.google.com/optimization/routing/dimensions">dimensions</a> to track each vehicles's cumulative quantity along its route

6. Set first solution heuristic (choose from `routing_enums_pb2.FirstSolutionStrategy`); Or parse initial_routes in step 1

7. **Optional**: Set metaheuristic (choose from `routing_enums_pb2.LocalSearchMetaheuristic`)

8. Solve and print solution

## Metaheuristic Options

Change strategy for a large data set where first solution strategy doesn't yield a good solution.

In [TSP](tsp/travelling_sales_man.py) (10 nodes), the added advance search makes no difference to the result and first solution strategy is sufficient. In [Circuit Board](tsp/circuit_board.py) (280 nodes), the advance search returns a different solution as you tune the limit and is usually better than the deterministic first solution.

The local search metaheuristic let solver escape a local minimum—a solution that is shorter than all nearby routes, but which is not the global minimum.

Local search requires a set limit on run time or number of solution trials.

```
search_parameters = pywrapcp.DefaultRoutingSearchParameters()
search_parameters.local_search_metaheuristic = (
routing_enums_pb2.LocalSearchMetaheuristic.GUIDED_LOCAL_SEARCH)
search_parameters.time_limit.seconds = 30
search_parameters.solution_limit = 1000
search_parameters.log_search = True
```

<a href="https://developers.google.com/optimization/routing/routing_options#local_search_options"> Local Search Options </a>

# [Rust VRP Solver](rust_vrp)

<a href="https://reinterpretcat.github.io/vrp/getting-started/index.html"> A Vehicle Routing Problem Solver Documentation </a>

Use CLI:

1. Create pragmatic format `problem.json` from csv:

   ```
   vrp-cli import csv -i jobs.csv vehicles.csv -o problem.json
   ```

   jobs.csv defines a plan of the problem and vehicles.csv defines a fleet. csv import is limited to Capacitated Vehicle Routing Problem with Time Windows (CVRPTW).

2. Get routing matrix (1D) from external routing services or skip to use approximation

   A routing matrix is a matrix with rows labeled by origins and columns by destinations. Each entry of the matrix is the travel time or distance from the origin to the destination.

   Extract geolocations from problem.json:

   ```
   vrp-cli solve pragmatic problem.json --get-locations -o locations.json
   ```

3. Run Solver

   ```
   vrp-cli solve pragmatic problem.json -o solution.json -g solution.geojson --log
   ```

   Options:

   - `-o`, write solution to file
   - `-g`, write solution to geojson
   - `-m` parse routing matrix, omit to use routing matrix approximation with default speed of 10m/s
   - `--search-mode=deep`, `broad` by default
   - `--max-time`, duration of run
   - `--max-generations`, number of steps before termination
   - `--min-cv`, coefficient of variation for objectives
