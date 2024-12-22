import sys
from ._cli import run_cli


def main():
    run_cli(sys.argv[1:])


if __name__ == "__main__":
    main()
