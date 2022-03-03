import circuit_board
from ortools.init import pywrapinit


def main():
    circuit_board.CB_solver()


if __name__ == "__main__":
    pywrapinit.CppBridge.InitLogging("basic_example.py")
    cpp_flags = pywrapinit.CppFlags()
    cpp_flags.logtostderr = True
    cpp_flags.log_prefix = False
    pywrapinit.CppBridge.SetFlags(cpp_flags)

    main()
