import vrp.vehicle_routing_problem as vrp
from ortools.init import pywrapinit


def main():
    vrp.VRP_solvers()


if __name__ == "__main__":
    pywrapinit.CppBridge.InitLogging("basic_example.py")
    cpp_flags = pywrapinit.CppFlags()
    cpp_flags.logtostderr = True
    cpp_flags.log_prefix = False
    pywrapinit.CppBridge.SetFlags(cpp_flags)

    main()
