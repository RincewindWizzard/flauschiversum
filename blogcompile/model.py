from markdown import Markdown
import frontmatter
import os.path
from functools import lru_cache
from datetime import datetime, date
from PIL import Image as PImage
import settings
import io
import sys
import hashlib
from . import cache
from . import markdown_slideshow


class AbstractContentObject(object):
    """
    An object which points to a content resource. These are singletons, so if paths equal then created objects are identical.
    """
    CONTENT_OBJECTS = {}

    def __new__(cls, path):
        path = os.path.abspath(path)
        if not path in AbstractContentObject.CONTENT_OBJECTS:
            AbstractContentObject.CONTENT_OBJECTS[path] = super().__new__(cls)
        return AbstractContentObject.CONTENT_OBJECTS[path]

    def __init__(self, path):
        self._path = os.path.abspath(path)

    @property
    def path(self):
        return self._path

    @property
    def basename(self):
        return os.path.basename(self.path)

    @property
    def url(self):
        from .urls import get_url_for
        return get_url_for(self)

    @property
    @lru_cache(maxsize=None)
    def sha256sum(self):
        return hashlib.sha256(self.content).hexdigest()

    @property
    @lru_cache(maxsize=None)
    def content(self):
        with open(self.path, 'rb') as f:
            return f.read()

    def __repr__(self):
        return '{}(path={})'.format(self.__class__.__name__, self.path)


class Article(AbstractContentObject):
    """
    This represents the (abstract) parent class of all text resources in this blog.
    It consists of a markdown file, that can be converted to html.
    Articles are identified by their path on disk.
    """

    @property
    def title(self):
        return self.meta.get('title')

    @property
    @lru_cache(maxsize=None)
    def meta(self):
        """
        Every markdown document has the option to contain a frontmatter with yaml styled key value pairs.
        :return: dict with all frontmatter values
        """
        return frontmatter.loads(self.content).to_dict()

    @property
    @lru_cache(maxsize=None)
    def markdown(self):
        return self.meta.get('content')

    @property
    @lru_cache(maxsize=None)
    def html(self):
        return markdown.convert(self.markdown)


class Post(Article):
    @property
    @lru_cache(maxsize=None)
    def date(self):
        dt = self.meta.get('date')
        if isinstance(dt, date):
            return datetime(dt.year, dt.month, dt.day)
        else:
            return dt

    @property
    @lru_cache(maxsize=None)
    def published(self):
        return self.date.today() >= self.date

    @property
    @lru_cache(maxsize=None)
    def datestring(self):
        months = [
            'Januar', 'Februar', 'März', 'April',
            'Mai', 'Juni', 'Juli', 'August',
            'September', 'Oktober', 'November', 'Dezember'
        ]
        return '{}. {} {}'.format(self.date.day, months[self.date.month - 1], self.date.year)


    @property
    @lru_cache(maxsize=None)
    def excerpt(self):
        return self.meta.get('excerpt')

    @property
    @lru_cache(maxsize=None)
    def thumb(self):
        if self.meta.get('image'):
            return Image(os.path.join(os.path.dirname(self.path), self.meta.get('image')))

    @property
    @lru_cache(maxsize=None)
    def html(self):
        return markdown_slideshow.convert(self.markdown)


class Page(Article):
    ...


class Image(AbstractContentObject):
    def __init__(self, path):
        super().__init__(path)

        self._small, self._medium, self._large = cache.retrieve_image(self)

    @property
    def post(self):
        """
        Image can be contained in a blog post.
        If that is the case this method returns the parent post.
        :return: parent post if present
        """
        post_path = os.path.join(os.path.dirname(self.path), 'index.md')
        if os.path.isfile(post_path):
            return Post(post_path)
        return None

    @property
    @lru_cache(maxsize=None)
    def small(self):
        if self._small:
            return self._small
        else:
            cache.store_image(self)
            return self.thumbnail(settings.IMAGE_SMALL_WIDTH)

    @property
    @lru_cache(maxsize=None)
    def medium(self):
        if self._medium:
            return self._medium
        else:
            cache.store_image(self)
            return self.thumbnail(settings.IMAGE_MEDIUM_WIDTH)

    @property
    @lru_cache(maxsize=None)
    def large(self):
        if self._large:
            return self._large
        else:
            cache.store_image(self)
            return self.thumbnail(settings.IMAGE_LARGE_WIDTH)

    def thumbnail(self, width=200, height=sys.maxsize):
        img = PImage.open(self.path)

        overlay = PImage.open(settings.OVERLAY_IMG)
        overlay.thumbnail(img.size, PImage.ANTIALIAS)

        img.paste(overlay, (img.size[0] - overlay.size[0], img.size[1] - overlay.size[1]), overlay)
        img.thumbnail((width, height), PImage.ANTIALIAS)

        f = io.BytesIO()
        img.save(f, format='JPEG')
        return f.getvalue()


markdown = Markdown()
