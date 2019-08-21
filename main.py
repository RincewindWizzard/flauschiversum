import argparse
from blogcompile import sourcewalker, model, query, builder


def main():
    #for post in query.query_posts(sourcewalker.find_sources('src')):
    #    print(repr(post))
    builder.clean()
    builder.build()


if __name__ == '__main__':
    main()