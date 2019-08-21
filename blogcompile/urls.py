from slugify import slugify
import os
import settings

def get_url_for(obj):
    from .model import Post, Image, Page
    if isinstance(obj, Post):
        return get_url_for_post(obj)
    elif isinstance(obj, Image):
        return get_url_for_image(obj, width=settings.IMAGE_LARGE_WIDTH)
    elif isinstance(obj, Page):
        return get_url_for_page(obj)
    else:
        raise ValueError('URL creation not supported for given object!')

def get_url_for_pagination(index, prefix=''):
    return prefix + ('/index.html' if index == 0 else '/page/{}/index.html'.format(index))


def get_url_for_post(post):
    return '/{:04d}/{:02d}/{}/'.format(post.date.year, post.date.month, slugify(post.title))

def get_url_for_page(page):
    return '/{}/'.format(slugify(page.title))


def get_url_for_image(img, width=None, height=None, prefix=''):
    """ Creates an url to a resized version of the image which is not bigger than width x height """

    if hasattr(img, 'post') and img.post:
        prefix = get_url_for_post(img.post)

    from .model import Image
    if isinstance(img, Image):
        basename = img.basename
    else:
        basename = str(img)

    if width and height:
        return os.path.join(prefix, '{}x{}'.format(width, height), basename)
    elif width:
        return os.path.join(prefix, '{}x'.format(width), basename)
    elif height:
        return os.path.join(prefix, 'x{}'.format(height), basename)
    else:
        return os.path.join(prefix, basename)
