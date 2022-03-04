<a href="https://developers.google.com/optimization/introduction/python">Get Started with OR-Tools for Python</a>

## Steps for TSP & VRP
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
