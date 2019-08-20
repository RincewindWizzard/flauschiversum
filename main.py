import argparse
from blogcompile import sourcewalker, model


def main():
    for content in sourcewalker.find_sources('src'):
        print(repr(content))

if __name__ == '__main__':
    main()