from markdown import Markdown
import frontmatter
import os.path


class AbstractContentObject(object):
    def __init__(self, path):
        self._path = path

    @property
    def path(self):
        return self._path

    def __repr__(self):
        return '{}(path={})'.format(self.__class__.__name__, self.path)


class Article(AbstractContentObject):
    """
    This represents the (abstract) parent class of all text resources in this blog.
    It consists of a markdown file, that can be converted to html.
    Articles are identified by their path on disk.
    """

    def __init__(self, path):
        super().__init__(path)
        self._markdown = None
        self._html = None
        self._meta = None

    @property
    def meta(self):
        """
        Every markdown document has the option to contain a frontmatter with yaml styled key value pairs.
        :return: dict with all frontmatter values
        """
        if not self._meta:
            self._meta = frontmatter.loads(self.markdown).to_dict()
        return self._meta

    @property
    def markdown(self):
        if self._markdown == None:
            with open(self.path, "r") as f:
                self._markdown = f.read()
        return self._markdown

    @property
    def html(self):
        if not self._html:
            self._html = markdown.convert(self.markdown)
        return self._html




class Image(AbstractContentObject):
    ...


markdown = Markdown()