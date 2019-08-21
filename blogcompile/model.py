from markdown import Markdown
import frontmatter
import os.path
from functools import lru_cache
from datetime import datetime, date


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
    def url(self):
        from .urls import get_url_for
        return get_url_for(self)

    @property
    @lru_cache(maxsize=None)
    def content(self):
        with open(self.path) as f:
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
    def date(self):
        dt = self.meta.get('date')
        if isinstance(dt, date):
            return datetime(dt.year, dt.month, dt.day)
        else:
            return dt

    @property
    def published(self):
        return self.date.today() >= self.date

    @property
    def datestring(self):
        months = [
            'Januar', 'Februar', 'März', 'April',
            'Mai', 'Juni', 'Juli', 'August',
            'September', 'Oktober', 'November', 'Dezember'
        ]
        return '{}. {} {}'.format(self.date.day, months[self.date.month - 1], self.date.year)


    @property
    def excerpt(self):
        return self.meta.get('excerpt')

class Page(Article):
    ...

class Image(AbstractContentObject):
    ...


markdown = Markdown()
