from functools import lru_cache
from .model import Post, Page, Image, StaticFile
from . import urls
import types

"""
This module contains all queries to the dataset of Post, Pages and Images.
Do all filtering here!
"""

def query_by_class(clazz):
    def query(obj):
        return isinstance(obj, clazz)
    return query

def execute(query, dataset):
    return [ doc for doc in dataset if query(doc)]

def paginate(dataset, pagesize):
    if pagesize == 0 or pagesize == None:
        return dataset
    else:
        return [dataset[i:i + pagesize] for i in range(0, len(dataset), pagesize)]


query_posts = query_by_class(Post)
query_pages = query_by_class(Page)
query_images = query_by_class(Image)
query_static = query_by_class(StaticFile)

def filtered_dataset(filter_func):
    """
    Decorates a method, so that you can apply the whole dataset to it and it gets only called with the desired objects.
    :param filter_func: returns true for every object in dataset which we want
    :return: a generator mapping return values
    """
    def decorator(func):
        def apply(dataset):
            for doc in dataset:
                if filter_func(doc):
                    result = func(doc)
                    if isinstance(result, types.GeneratorType):
                        for e in result:
                            yield e
                    else:
                        yield func(doc)
        return apply
    return decorator

def pagination(filter_func, sort_key=None, reverse=False, pagesize=1):
    """
    Decorates a method, so that you can apply the whole dataset to it and it gets only called with the desired objects.
    :param filter_func: returns true for every object in dataset which we want
    :param pagesize: Count of elements in a page
    :param sort_key: function to extract sorting key
    :param reverse: if sorted list should be reversed
    :return:
    """
    def decorator(func):
        def apply(dataset):
            docs = [ doc for doc in dataset if filter_func(doc) ]
            if sort_key:
                docs = sorted(docs, key=sort_key)

            if reverse:
                docs = list(reversed(docs))

            pagination = paginate(docs, pagesize)
            for index, page in enumerate(pagination):
                yield func(index, page, len(pagination))

        return apply
    return decorator

def only_once(func):
    def apply(dataset):
        yield func()
    return apply