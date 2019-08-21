from .model import Post
from slugify import slugify

def get_url_for_pagination(index, prefix=''):
    return prefix + ('/index.html' if index == 0 else '/page/{}/index.html'.format(index))

def get_url_for(obj):
    if isinstance(obj, Post):
        return '/{:04d}/{:02d}/{}/'.format(obj.date.year, obj.date.month, slugify(obj.title))
    raise ValueError("type not supported")