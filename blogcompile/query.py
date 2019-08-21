from functools import lru_cache
from .model import Post, Page, Image

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
    return [dataset[i:i + pagesize] for i in range(0, len(dataset), pagesize)]


query_posts = query_by_class(Post)
query_pages = query_by_class(Page)
query_images = query_by_class(Image)

def filtered_dataset(filter_func, pagesize=None):
    """
    Decorates a method, so that you can apply the whole dataset to it and it gets only called with the desired objects.
    :param filter_func: returns true for every object in dataset which we want
    :param pagesize: Count of elements in a page
    :return:
    """
    def decorator(func):
        def apply(dataset):
            docs = [ doc for doc in dataset if filter_func(doc) ]
            if pagesize:
                for index, page in enumerate(paginate(docs, pagesize)):
                    yield func(index, page)
            else:
                for doc in docs:
                    yield func(doc)
        return apply
    return decorator
