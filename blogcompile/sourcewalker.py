import os
import re
from .model import Article, Image

CONTENT_MODELS = [
    (re.compile(regex), clazz)
    for regex, clazz in [
        (r'.*\.md', Article),
        (r'.*\.jpg', Image),
        (r'.*\.JPG', Image)
    ]
]


def find_sources(path):
    """
    This is a filewalker function which looks at your content repository an finds all content objects in it.
    All content objects found will be yielded.
    (see also model.py for more information about content objects)
    :param path: Path to your content sources
    :return: a yielded list of content objects
    """

    for dirpath, dirnames, filenames in os.walk(path):
        for filename in filenames:
            content_object = parse_source(os.path.join(dirpath, filename))
            if content_object:
                yield content_object


def parse_source(path):
    """
    Creates a content object from given path by looking at the regular expressions in CONTENT_MODELS.
    Whatever matches first gets the object created.
    :param path: Path to source file
    :return: content object or None
    """
    for regex, clazz in CONTENT_MODELS:
        if regex.match(path):
            return clazz(path)
