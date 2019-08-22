import os
import argparse
import settings
from blogcompile import builder


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--dst", help="Path to destination of build.")
    parser.add_argument("--cache", help="Path to image cache.")
    parser.add_argument("--clean", help="Remove previous build artifacts")
    args = parser.parse_args()

    if args.cache:
        settings.CACHE_PATH = os.path.abspath(args.cache)

    if args.dst:
        settings.BUILD_PATH = os.path.abspath(args.dst)

    if args.clean:
        builder.clean()

    builder.build()

if __name__ == '__main__':
    main()