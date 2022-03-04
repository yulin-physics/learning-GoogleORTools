from ortools.linear_solver import pywraplp


def MIP_solver():
    data = create_data()
    solver = pywraplp.Solver.CreateSolver("SCIP")

    # x[i, j] is an array of 0-1 variables
    x = {}
    for i in range(data["num_depots"]):
        for j in range(data["num_locations"]):
            x[i, j] = solver.IntVar(0, 1, "")

    # Create constraints
    # Each depot is assigned to at most 1 location
    # for i in range(data["num_depots"]):
    #     solver.Add(solver.Sum([x[i, j] for j in range(data["num_locations"])]) <= 1)
    # Each location is assigned to exactly one depot
    for j in range(data["num_locations"]):
        solver.Add(solver.Sum([x[i, j] for i in range(data["num_depots"])]) == 1)

    # Create objective function
    objective_terms = []
    for i in range(data["num_depots"]):
        for j in range(data["num_locations"]):
            objective_terms.append(data["costs"][i][j] * x[i, j])
    solver.Minimize(solver.Sum(objective_terms))

    status = solver.Solve()

    if status == pywraplp.Solver.OPTIMAL or status == pywraplp.Solver.FEASIBLE:
        print("Total cost = ", solver.Objective().Value(), "\n")
        for i in range(data["num_depots"]):
            for j in range(data["num_locations"]):
                # Test if x[i,j] is 1 (with tolerance for floating point arithmetic).
                if x[i, j].solution_value() > 0.5:
                    print(
                        "Depot %d assigned to location %d.  Cost = %d"
                        % (i, j, data["costs"][i][j])
                    )


def create_data():
    # Cost of assign node (column) to each depot (row)
    data = {}
    data["costs"] = [
        [90, 80, 75, 70],
        [35, 85, 55, 65],
        [125, 95, 90, 95],
        [45, 110, 95, 115],
        [50, 100, 90, 100],
    ]
    data["num_depots"] = len(data["costs"])
    data["num_locations"] = len(data["costs"][0])
    return data
